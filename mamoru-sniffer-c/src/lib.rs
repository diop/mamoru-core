mod evm_types;
mod ffi_types;

use lazy_static::lazy_static;
use mamoru_sniffer::{
    core::{StructValue, Value, ValueData},
    Sniffer, SnifferConfig,
};
use safer_ffi::prelude::*;
use std::collections::HashMap;

pub use evm_types::*;
pub use ffi_types::*;

lazy_static! {
    static ref RUNTIME: tokio::runtime::Runtime = {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("BUG: Failed to init async runtime.")
    };
}

#[ffi_export]
fn new_sniffer() -> repr_c::Box<FfiSnifferResult<FfiSniffer>> {
    let result = RUNTIME.block_on(async {
        let config = SnifferConfig::from_env()?;

        Sniffer::new(config).await
    });

    let (data, error) = match result {
        Ok(sniffer) => (Some(FfiSniffer { inner: sniffer }), None),
        Err(err) => (None, Some(err)),
    };

    repr_c::Box::new(FfiSnifferResult { data, error })
}

/// Returns a pointer to C-like string or NULL if there is no error.
#[ffi_export]
fn sniffer_result_get_error_message(result: &FfiSnifferResult<FfiSniffer>) -> Option<char_p::Box> {
    result
        .error
        .as_ref()
        .map(|err| format!("{:#?}", err).try_into().unwrap())
}

/// Returns a pointer to `FfiSniffer` or NULL.
/// Frees `result` arguments, so be sure to call `sniffer_result_get_error_message` to get an error message.
#[ffi_export]
fn sniffer_result_get_sniffer(
    result: repr_c::Box<FfiSnifferResult<FfiSniffer>>,
) -> Option<repr_c::Box<FfiSniffer>> {
    let result = result.into();

    result.data.map(repr_c::Box::new)
}

/// Frees `value` argument.
#[ffi_export]
fn new_value_data(value: repr_c::Box<FfiValue>) -> repr_c::Box<FfiValueData> {
    let value = value.into().inner;

    repr_c::Box::new(FfiValueData {
        inner: ValueData::new(value).expect("BUG: `Value` serialization failed."),
    })
}

#[ffi_export]
fn new_value_bool(data: bool) -> repr_c::Box<FfiValue> {
    repr_c::Box::new(FfiValue {
        inner: Value::Bool(data),
    })
}

#[ffi_export]
fn new_value_u64(data: u64) -> repr_c::Box<FfiValue> {
    repr_c::Box::new(FfiValue {
        inner: Value::U64(data),
    })
}

#[ffi_export]
fn new_value_list() -> repr_c::Box<FfiValue> {
    repr_c::Box::new(FfiValue {
        inner: Value::List(vec![]),
    })
}

/// Returns `true` if success.
/// Frees `data` argument.
#[ffi_export]
fn value_list_append(value_list: &mut FfiValue, data: repr_c::Box<FfiValue>) -> bool {
    let mut ok = false;
    let data = data.into().inner;

    if let Value::List(list) = &mut value_list.inner {
        list.push(data);

        ok = true;
    }

    ok
}

#[ffi_export]
fn new_value_struct(ty: char_p::Ref<'_>) -> repr_c::Box<FfiValue> {
    let ty = ty.to_str().to_string();

    repr_c::Box::new(FfiValue {
        inner: Value::Struct(StructValue::new(ty, HashMap::new())),
    })
}

/// Returns `true` if success.
/// Frees `data` argument.
#[ffi_export]
fn value_struct_add_field(
    value_struct: &mut FfiValue,
    key: char_p::Ref<'_>,
    data: repr_c::Box<FfiValue>,
) -> bool {
    let data = data.into().inner;
    let key = key.to_str().to_string();

    if let Value::Struct(value) = &mut value_struct.inner {
        if value.field_exist(&key) {
            return false;
        } else {
            value.add_field(key, data);
        }
    } else {
        return false;
    }

    true
}

#[safer_ffi::cfg_headers]
#[test]
fn generate_headers() -> ::std::io::Result<()> {
    safer_ffi::headers::builder()
        .to_file("headers.h")?
        .generate()
}
