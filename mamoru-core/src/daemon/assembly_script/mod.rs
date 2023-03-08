mod imports;

use crate::{
    daemon::{DaemonParameters, Executor, Incident},
    BlockchainDataCtx, DataError,
};
use as_ffi_bindings::{Read, StringPtr, Write};
use async_trait::async_trait;
use std::{
    fmt::{Debug, Formatter},
    sync::{mpsc, Arc},
};
use tracing::Level;
use wasmer::{
    AsStoreMut, AsStoreRef, Engine, FunctionEnv, Instance, Memory, Module, Store, TypedFunction,
    WasmTypeList,
};

/// Maximum incident reports by a single run.
const MAX_INCIDENTS: usize = 128;

/// The main function name.
const ENTRYPOINT_NAME: &str = "main";

/// The main function signature.
type Entrypoint = TypedFunction<(), ()>;

/// Executes WASM binaries compiled from AssemblyScript.
/// Other runtimes is not supported by this executor: we rely on AssemblyScript's GC exports.
///
/// The instantiated struct holds a compiled [`wasmer::Module`], so it's compiled only once.
///
/// Each [`AssemblyScriptExecutor::execute`] starts a fresh environment,
/// it's not possible to store data between runs in WASM memory.
pub struct AssemblyScriptExecutor {
    /// The compiled [`wasmer::Module`].
    module: Module,

    /// The engine used to compile the module.
    engine: Engine,

    /// The parameters that are passed to Daemon.
    /// Accessible from WASM via host functions.
    parameters: Arc<DaemonParameters>,
}

#[async_trait]
impl Executor for AssemblyScriptExecutor {
    async fn execute(&self, ctx: &BlockchainDataCtx) -> Result<Vec<Incident>, DataError> {
        let (mut store, entrypoint, incidents_rx) = self.prepare_vm(ctx)?;

        tokio::task::spawn_blocking(move || {
            let span = tracing::span!(Level::TRACE, "assembly_script:entrypoint");
            let _guard = span.enter();

            entrypoint.call(&mut store)
        })
        .await
        .expect("BUG: AssemblyScriptExecutor entrypoint call is panicked.")
        .map_err(DataError::WasmRuntime)?;

        let incidents = incidents_rx.into_iter().collect();

        Ok(incidents)
    }
}

impl AssemblyScriptExecutor {
    pub fn new(wasm: impl AsRef<[u8]>, parameters: DaemonParameters) -> Result<Self, DataError> {
        let store = Store::default();
        let engine = store.engine().clone();
        let module = Module::from_binary(&engine, wasm.as_ref()).map_err(DataError::WasmCompile)?;

        Ok(Self {
            module,
            engine,
            parameters: Arc::new(parameters),
        })
    }

    /// Creates new environment for WASM execution.
    #[tracing::instrument(skip(ctx, self), level = "trace")]
    fn prepare_vm(
        &self,
        ctx: &BlockchainDataCtx,
    ) -> Result<(Store, Entrypoint, mpsc::Receiver<Incident>), DataError> {
        let mut store = Store::new(&self.engine);

        let (tx, rx) = mpsc::sync_channel::<Incident>(MAX_INCIDENTS);
        let env = FunctionEnv::new(
            &mut store,
            WasmEnv {
                bindings_env: as_ffi_bindings::Env::default(),
                data_ctx: ctx.clone(),
                incidents_tx: tx,
                parameters: Arc::clone(&self.parameters),
            },
        );

        let imports = imports::all(&mut store, &env);
        let instance =
            Instance::new(&mut store, &self.module, &imports).map_err(DataError::WasmInit)?;

        WasmEnv::init_bindings_env(&env, &mut store, &instance)?;

        let entrypoint = get_typed_function(&instance, &store, ENTRYPOINT_NAME)?;

        Ok((store, entrypoint, rx))
    }
}

/// The context available to all exported host functions.
pub(crate) struct WasmEnv {
    bindings_env: as_ffi_bindings::Env,
    data_ctx: BlockchainDataCtx,
    incidents_tx: mpsc::SyncSender<Incident>,
    parameters: Arc<DaemonParameters>,
}

impl WasmEnv {
    /// Imports WASM memory and AssemblyScript's GC functions.
    fn init_bindings_env(
        env: &FunctionEnv<Self>,
        store: &mut Store,
        instance: &Instance,
    ) -> Result<(), DataError> {
        let memory = get_memory(instance, "memory")?;

        let asc_fn_new = get_typed_function(instance, store, "__new")?;
        let asc_fn_pin = get_typed_function(instance, store, "__pin")?;
        let asc_fn_unpin = get_typed_function(instance, store, "__unpin")?;
        let asc_fn_collect = get_typed_function(instance, store, "__collect")?;

        env.as_mut(store).bindings_env.init_with(
            Some(memory),
            Some(asc_fn_new),
            Some(asc_fn_pin),
            Some(asc_fn_unpin),
            Some(asc_fn_collect),
        );

        Ok(())
    }

    fn memory(&self) -> &Memory {
        self.bindings_env
            .memory
            .as_ref()
            .expect("BUG: Memory is not initialized.")
    }

    fn read_string_ptr(
        &self,
        ptr: &StringPtr,
        store: &impl AsStoreRef,
    ) -> Result<String, wasmer::RuntimeError> {
        let value = ptr
            .read(self.memory(), store)
            .map_err(|e| wasmer::RuntimeError::new(e.to_string()))?;

        Ok(value)
    }

    fn alloc_string_ptr(
        env: as_ffi_bindings::Env,
        value: String,
        store: &mut impl AsStoreMut,
    ) -> Result<StringPtr, wasmer::RuntimeError> {
        let ptr = StringPtr::alloc(&value, &env, store)
            .map_err(|e| wasmer::RuntimeError::new(e.to_string()))?;

        Ok(*ptr)
    }
}

fn get_memory(instance: &Instance, name: &str) -> Result<Memory, DataError> {
    let memory = instance
        .exports
        .get_memory(name)
        .map_err(|source| DataError::WasmExport {
            source,
            export: name.to_string(),
        })?;

    Ok(memory.clone())
}

fn get_typed_function<Args, Rets>(
    instance: &Instance,
    store: &impl AsStoreRef,
    name: &str,
) -> Result<TypedFunction<Args, Rets>, DataError>
where
    Args: WasmTypeList,
    Rets: WasmTypeList,
{
    instance
        .exports
        .get_typed_function(store, name)
        .map_err(|source| DataError::WasmExport {
            source,
            export: name.to_string(),
        })
}

impl Debug for AssemblyScriptExecutor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("AssemblyScriptExecutor")?;

        Ok(())
    }
}
