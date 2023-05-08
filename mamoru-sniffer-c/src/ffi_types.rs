use mamoru_sniffer::{
    core::{Value, ValueData},
    Sniffer, SnifferError,
};
use safer_ffi::prelude::*;

#[derive_ReprC]
#[ReprC::opaque]
pub struct FfiSniffer {
    pub(crate) inner: Sniffer,
}

#[derive_ReprC]
#[ReprC::opaque]
pub struct FfiSnifferResult<T> {
    pub(crate) data: Option<T>,
    pub(crate) error: Option<SnifferError>,
}

#[derive_ReprC]
#[ReprC::opaque]
pub struct FfiValueData {
    pub inner: ValueData,
}

#[derive_ReprC]
#[ReprC::opaque]
pub struct FfiValue {
    pub(crate) inner: Value,
}
