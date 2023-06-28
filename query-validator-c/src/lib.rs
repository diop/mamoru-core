use lazy_static::lazy_static;
use query_validator::{
    validate_assembly_script, validate_sql, validate_sql_renders, ChainType, DaemonParameters,
    ValidateError,
};
use safer_ffi::prelude::*;

#[derive_ReprC]
#[repr(u8)]
pub enum FfiChainType {
    Sui = 0,
    Evm = 1,
    Aptos = 2,
}

#[derive_ReprC]
#[repr(C)]
pub struct FfiValidationResult {
    is_error: bool,
    message: char_p::Box,
}

impl From<Result<(), ValidateError>> for FfiValidationResult {
    fn from(value: Result<(), ValidateError>) -> Self {
        let (is_error, message) = match value {
            Ok(_) => (false, "".to_string()),
            Err(err) => (true, format!("{:#?}", err)),
        };

        FfiValidationResult {
            is_error,
            message: message.try_into().unwrap(),
        }
    }
}

#[derive_ReprC]
#[ReprC::opaque]
pub struct FfiDaemonParameters {
    inner: DaemonParameters,
}

impl From<FfiChainType> for ChainType {
    fn from(chain: FfiChainType) -> Self {
        match chain {
            FfiChainType::Sui => Self::Sui,
            FfiChainType::Evm => Self::Evm,
            FfiChainType::Aptos => Self::Aptos,
        }
    }
}

lazy_static! {
    static ref RUNTIME: tokio::runtime::Runtime = {
        tokio::runtime::Builder::new_multi_thread()
            .build()
            .expect("BUG: Failed to init async runtime.")
    };
}

/// Drops `parameters` argument.
#[ffi_export]
fn ffi_validate_sql<'a>(
    chain: FfiChainType,
    query: char_p::Ref<'a>,
    parameters: repr_c::Box<FfiDaemonParameters>,
) -> FfiValidationResult {
    let chain = chain.into();
    let query = query.to_str();
    let parameters = parameters.into().inner;

    let result = RUNTIME
        .handle()
        .block_on(async move { validate_sql(chain, query, parameters).await });

    result.into()
}

/// Drops `parameters` argument.
#[ffi_export]
fn ffi_validate_sql_renders<'a>(
    query: char_p::Ref<'a>,
    parameters: repr_c::Box<FfiDaemonParameters>,
) -> FfiValidationResult {
    let query = query.to_str();
    let parameters = parameters.into().inner;

    let result = validate_sql_renders(query, parameters);

    result.into()
}

#[ffi_export]
fn ffi_validate_assembly_script<'a>(
    chain: FfiChainType,
    bytes: c_slice::Ref<'a, u8>,
) -> FfiValidationResult {
    let chain = chain.into();
    let bytes = bytes.as_slice();

    let result = RUNTIME
        .handle()
        .block_on(async move { validate_assembly_script(chain, bytes).await });

    let (is_error, message) = match result {
        Ok(_) => (false, "".to_string()),
        Err(err) => (true, format!("{:#?}", err)),
    };

    FfiValidationResult {
        is_error,
        message: message.try_into().unwrap(),
    }
}

#[ffi_export]
fn ffi_drop_validation_result(result: FfiValidationResult) {
    drop(result)
}

#[ffi_export]
fn ffi_new_daemon_parameters() -> repr_c::Box<FfiDaemonParameters> {
    repr_c::Box::new(FfiDaemonParameters {
        inner: DaemonParameters::default(),
    })
}

#[ffi_export]
fn ffi_append_daemon_parameter<'a>(
    parameters: &mut FfiDaemonParameters,
    key: char_p::Ref<'a>,
    value: char_p::Ref<'a>,
) {
    let key = key.to_str();
    let value = value.to_str();

    parameters.inner.insert(key.to_string(), value.to_string());
}

#[safer_ffi::cfg_headers]
#[test]
fn generate_headers() -> ::std::io::Result<()> {
    safer_ffi::headers::builder()
        .to_file("headers.h")?
        .generate()
}
