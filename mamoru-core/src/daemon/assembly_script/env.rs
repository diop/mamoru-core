use std::sync::{mpsc, Arc};

use as_ffi_bindings::{Read, StringPtr, Write};
use wasmer::{
    AsStoreMut, AsStoreRef, FunctionEnv, FunctionEnvMut, Instance, Memory, Store, TypedFunction,
    WasmPtr, WasmTypeList,
};

use crate::blockchain_data::serialize::pack_values;
use crate::{BlockchainCtx, BlockchainData, DaemonParameters, DataError, Incident};

/// The context available to all exported host functions.
pub(crate) struct WasmEnv<T> {
    pub(crate) bindings_env: as_ffi_bindings::Env,
    pub(crate) data_ctx: BlockchainData<T>,
    pub(crate) incidents_tx: mpsc::SyncSender<Incident>,
    pub(crate) parameters: Arc<DaemonParameters>,
}

impl<T: BlockchainCtx> WasmEnv<T> {
    /// Imports WASM memory and AssemblyScript's GC functions.
    pub(crate) fn init_bindings_env(
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

    pub(crate) fn memory(&self) -> &Memory {
        self.bindings_env
            .memory
            .as_ref()
            .expect("BUG: Memory is not initialized.")
    }

    pub(crate) fn read_string_ptr(
        &self,
        ptr: &StringPtr,
        store: &impl AsStoreRef,
    ) -> Result<String, wasmer::RuntimeError> {
        let value = ptr
            .read(self.memory(), store)
            .map_err(|e| wasmer::RuntimeError::new(e.to_string()))?;

        Ok(value)
    }

    pub(crate) fn alloc_string_ptr(
        env: as_ffi_bindings::Env,
        value: String,
        store: &mut impl AsStoreMut,
    ) -> Result<StringPtr, wasmer::RuntimeError> {
        let ptr = StringPtr::alloc(&value, &env, store)
            .map_err(|e| wasmer::RuntimeError::new(e.to_string()))?;

        Ok(*ptr)
    }

    pub(crate) fn alloc_slice(
        mut ctx: &mut FunctionEnvMut<WasmEnv<T>>,
        value: &[u8],
    ) -> Result<u64, wasmer::RuntimeError> {
        let len = value.len() as u32;
        let fn_new = ctx
            .data()
            .bindings_env
            .fn_new
            .clone()
            .expect("BUG: bindings_env is not initialized.");
        let fn_pin = ctx
            .data()
            .bindings_env
            .fn_pin
            .clone()
            .expect("BUG: bindings_env is not initialized.");

        let offset = fn_new.call(&mut ctx, len as i32, 1)? as u32;
        fn_pin.call(&mut ctx, offset as i32)?;

        let memory = ctx.data().memory().view(&ctx);

        WasmPtr::<u8>::new(offset)
            .slice(&memory, len)
            .map_err(|err| wasmer::RuntimeError::new(err.to_string()))?
            .write_slice(value)
            .map_err(|err| wasmer::RuntimeError::new(err.to_string()))?;

        Ok(pack_values(offset, len))
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

pub(crate) fn get_typed_function<Args, Rets>(
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
