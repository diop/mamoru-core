use crate::daemon::{Executor, Incident};
use crate::{BlockchainDataCtx, DataError};
use as_ffi_bindings::{Read, StringPtr, Write};
use async_trait::async_trait;
use std::error::Error;
use std::fmt::{Debug, Formatter};
use std::sync::mpsc;
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
}

impl AssemblyScriptExecutor {
    pub fn new(wasm: impl AsRef<[u8]>) -> Result<Self, DataError> {
        let store = Store::default();
        let engine = store.engine().clone();
        let module = Module::from_binary(&engine, wasm.as_ref()).map_err(DataError::WasmCompile)?;

        Ok(Self { module, engine })
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

/// The context available to all exported host functions.
pub(crate) struct WasmEnv {
    bindings_env: as_ffi_bindings::Env,
    data_ctx: BlockchainDataCtx,
    incidents_tx: mpsc::SyncSender<Incident>,
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

mod imports {
    use super::{runtime_error_ctx, Incident, WasmEnv};
    use crate::daemon::sql::SqlExecutor;
    use as_ffi_bindings::StringPtr;
    use tokio::runtime::Handle;
    use tracing::error;
    use wasmer::{imports, AsStoreMut, Function, FunctionEnv, FunctionEnvMut, Imports};

    pub(crate) fn all(store: &mut impl AsStoreMut, env: &FunctionEnv<WasmEnv>) -> Imports {
        imports! {
            "env" => {
                "abort" => Function::new_typed_with_env(store,  env, abort),
            },
            "mamoru" => {
                "query" => Function::new_typed_with_env(store, env, mamoru_query),
                "report" => Function::new_typed_with_env(store, env, mamoru_report),
            }
        }
    }

    pub(crate) fn abort(
        ctx: FunctionEnvMut<WasmEnv>,
        message: StringPtr,
        filename: StringPtr,
        line: u32,
        col: u32,
    ) -> Result<(), wasmer::RuntimeError> {
        let env = ctx.data();
        let message = env.read_string_ptr(&message, &ctx)?;
        let filename = env.read_string_ptr(&filename, &ctx)?;

        error!("Error: {} at {}:{} col: {}", message, filename, line, col);

        Ok(())
    }

    #[tracing::instrument(skip(ctx, query), level = "trace")]
    pub(crate) fn mamoru_query(
        mut ctx: FunctionEnvMut<WasmEnv>,
        query: StringPtr,
    ) -> Result<StringPtr, wasmer::RuntimeError> {
        runtime_error_ctx(|| {
            let env = ctx.data();
            let query = env.read_string_ptr(&query, &ctx)?;
            let sql_executor = SqlExecutor::new(&query)?;

            let outputs = Handle::current()
                .block_on(async move { sql_executor.query(&env.data_ctx).await })?;

            let serialized = serde_json::to_string(&outputs)?;
            let ptr = WasmEnv::alloc_string_ptr(env.bindings_env.clone(), serialized, &mut ctx)?;

            Ok(ptr)
        })
    }

    #[tracing::instrument(skip(ctx), level = "trace")]
    pub(crate) fn mamoru_report(ctx: FunctionEnvMut<WasmEnv>) -> Result<(), wasmer::RuntimeError> {
        let tx = &ctx.data().incidents_tx;

        runtime_error_ctx(|| {
            tx.try_send(Incident)?;

            Ok(())
        })
    }
}

fn runtime_error_ctx<F, T>(fun: F) -> Result<T, wasmer::RuntimeError>
where
    F: FnOnce() -> Result<T, Box<dyn Error>>,
{
    fun().map_err(|err| wasmer::RuntimeError::new(err.to_string()))
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
