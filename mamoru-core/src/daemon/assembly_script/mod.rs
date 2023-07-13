use semver::Version;
use std::collections::HashMap;
use std::{
    fmt::{Debug, Formatter},
    sync::{mpsc, Arc},
};

use tracing::Level;
pub use wasmer::{imports, AsStoreMut, Function, FunctionEnv, FunctionEnvMut, Imports};
use wasmer::{Engine, Extern, Instance, Module, Store, TypedFunction};

use crate::{
    assembly_script::env::{get_typed_function, WasmEnv},
    daemon::{DaemonParameters, Incident},
    BlockchainCtx, BlockchainData, CtxImportFn, DataError,
};

mod env;
mod imports;
mod incident;

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

    /// The SDKs versions daemon uses.
    _versions: HashMap<String, Version>,
}

impl AssemblyScriptExecutor {
    pub fn new(
        wasm: impl AsRef<[u8]>,
        parameters: DaemonParameters,
        versions: HashMap<String, Version>,
    ) -> Result<Self, DataError> {
        let store = Store::default();
        let engine = store.engine().clone();
        let module = Module::from_binary(&engine, wasm.as_ref()).map_err(DataError::WasmCompile)?;

        Ok(Self {
            module,
            engine,
            parameters: Arc::new(parameters),
            _versions: versions,
        })
    }

    pub async fn execute<T: BlockchainCtx>(
        &self,
        ctx: &BlockchainData<T>,
    ) -> Result<Vec<Incident>, DataError> {
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

    /// Creates new environment for WASM execution.
    #[tracing::instrument(skip(ctx, self), level = "trace")]
    fn prepare_vm<T: BlockchainCtx>(
        &self,
        ctx: &BlockchainData<T>,
    ) -> Result<(Store, Entrypoint, mpsc::Receiver<Incident>), DataError> {
        let mut store = Store::new(self.engine.clone());

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

        let mut imports = imports::all::<T>(&mut store, &env);

        let blockchain_module = T::module();
        let blockchain_imports = T::imports();

        imports.extend(blockchain_imports.into_iter().map(|(func_name, func)| {
            let wasm_func = Extern::Function(match func {
                CtxImportFn::NoArgs(func) => Function::new_typed_with_env(
                    &mut store,
                    &env,
                    move |mut ctx: FunctionEnvMut<WasmEnv<T>>| {
                        let value = func(ctx.data().data_ctx.data());
                        let ptr = WasmEnv::alloc_slice(&mut ctx, &value)?;

                        Ok::<u64, wasmer::RuntimeError>(ptr)
                    },
                ),
                CtxImportFn::ById(func) => Function::new_typed_with_env(
                    &mut store,
                    &env,
                    move |mut ctx: FunctionEnvMut<WasmEnv<T>>, id: u64| {
                        let data = ctx.data().data_ctx.clone();
                        let value = func(data.data(), id).map_err(|err| {
                            wasmer::RuntimeError::new(format!(
                                "error calling blockchain import {}: {}",
                                func_name, err
                            ))
                        })?;
                        let ptr = WasmEnv::alloc_slice(&mut ctx, value)?;

                        Ok::<u64, wasmer::RuntimeError>(ptr)
                    },
                ),
            });

            (
                (blockchain_module.to_string(), func_name.to_string()),
                wasm_func,
            )
        }));

        let instance = Instance::new(&mut store, &self.module, &imports)
            .map_err(|err| DataError::WasmInit(Box::new(err)))?;

        WasmEnv::init_bindings_env(&env, &mut store, &instance)?;

        let entrypoint = get_typed_function(&instance, &store, ENTRYPOINT_NAME)?;

        Ok((store, entrypoint, rx))
    }
}

impl Debug for AssemblyScriptExecutor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("AssemblyScriptExecutor")?;

        Ok(())
    }
}
