use safer_ffi::prelude::*;

use mamoru_core::rule::{Comparison, ComparisonOperator, ComparisonValue, Expression, Rule};
use mamoru_core::value::Value;
use mamoru_core::vendor::ethnum::{I256, U256};
use paste::paste;

use mamoru_core::blockchain_data_types::{Block, CallTrace, Event, Transaction};
use std::collections::HashMap;

macro_rules! value_constructor {
    ($value_ty:tt, $variant:expr, $ty:ident) => {
        paste! {
            /// Drops `value` argument
            #[ffi_export]
            fn [<ffi_value_ $ty _new>](value: repr_c::Box<$value_ty>) -> repr_c::Box<FfiValue> {
                let value = value.into();

                repr_c::Box::new(FfiValue {
                    inner: $variant(value.inner),
                })
            }
        }
    };
}

macro_rules! list_methods {
    ($value_ty:expr, $ty:ident) => {
        paste! {
            #[ffi_export]
            fn [<ffi_list_ $ty _new>]() -> repr_c::Box<FfiList<$value_ty>> {
                repr_c::Box::new(FfiList {
                    inner: vec![],
                })
            }

            /// Drops `value` argument
            #[ffi_export]
            fn [<ffi_list_ $ty _append>](list: &mut FfiList<$value_ty>, value: repr_c::Box<$value_ty>) {
                let value = *value.into();

                list.inner.push(value);
            }
        }
    };
}

macro_rules! ffi_free {
    ($value_ty:ty, $ty:ident) => {
        paste! {
            #[ffi_export]
            fn [<ffi_free_ $ty>](value: $value_ty) {
                drop(value);
            }
        }
    };
}


#[derive_ReprC]
#[ReprC::opaque]
pub struct FfiU256 {
    inner: U256,
}

impl FfiU256 {
    fn as_u128(&self) -> u128 {
        self.inner.as_u128()
    }
}

#[ffi_export]
fn ffi_u256_new(str_hex: char_p::Ref<'_>) -> repr_c::Box<FfiU256> {
    let str_hex = str_hex.to_str();

    repr_c::Box::new(FfiU256 {
        inner: U256::from_str_hex(str_hex).expect("Wrong hex value for u256."),
    })
}

#[derive_ReprC]
#[ReprC::opaque]
pub struct FfiI256 {
    inner: I256,
}

#[ffi_export]
fn ffi_i256_new(str_hex: char_p::Ref<'_>) -> repr_c::Box<FfiI256> {
    let str_hex = str_hex.to_str();

    repr_c::Box::new(FfiI256 {
        inner: I256::from_str_hex(str_hex).expect("Wrong hex value for i256."),
    })
}

#[derive_ReprC]
#[ReprC::opaque]
pub struct FfiList<T> {
    inner: Vec<T>,
}

list_methods!(FfiEvent, events);
list_methods!(FfiCallTrace, call_traces);
list_methods!(FfiTransaction, transactions);
list_methods!(FfiValue, values);

#[derive_ReprC]
#[ReprC::opaque]
pub struct FfiBlock {
    #[allow(dead_code)]
    inner: Block,
}

#[ffi_export]
fn ffi_block_new(
    block_index: &FfiU256,
    time: u64,
    transactions: &FfiList<FfiTransaction>,
    extra: &FfiHashMap,
) -> repr_c::Box<FfiBlock> {
    let transactions = transactions.inner.iter().map(|t| t.inner.clone()).collect();
    let extra = extra.inner.clone();

    repr_c::Box::new(FfiBlock {
        inner: Block::new(block_index.as_u128(), time, transactions, extra),
    })
}

#[derive_ReprC]
#[ReprC::opaque]
pub struct FfiTransaction {
    inner: Transaction,
}

#[ffi_export]
fn ffi_transaction_new(
    block_index: &FfiU256,
    tx_index: &FfiU256,
    time: u64,
    events: &FfiList<FfiEvent>,
    call_traces: &FfiList<FfiCallTrace>,
    extra: &FfiHashMap,
) -> repr_c::Box<FfiTransaction> {
    let events = events
        .inner
        .iter()
        .map(|event| event.inner.clone())
        .collect();
    let call_traces = call_traces
        .inner
        .iter()
        .map(|event| event.inner.clone())
        .collect();
    let extra = extra.inner.clone();

    repr_c::Box::new(FfiTransaction {
        inner: Transaction::new(
            block_index.as_u128(),
            tx_index.as_u128(),
            time,
            events,
            call_traces,
            extra,
        ),
    })
}

#[derive_ReprC]
#[ReprC::opaque]
pub struct FfiEvent {
    inner: Event,
}

#[ffi_export]
fn ffi_event_new(
    block_index: &FfiU256,
    tx_index: &FfiU256,
    event_index: &FfiU256,
    event_id: c_slice::Ref<u8>,
    extra: &FfiHashMap,
) -> repr_c::Box<FfiEvent> {
    let event_id = event_id.as_slice();
    let extra = extra.inner.clone();

    repr_c::Box::new(FfiEvent {
        inner: Event::new(
            block_index.as_u128(),
            tx_index.as_u128(),
            event_index.as_u128(),
            event_id.into(),
            extra,
        ),
    })
}

#[derive_ReprC]
#[ReprC::opaque]
pub struct FfiCallTrace {
    inner: CallTrace,
}

#[ffi_export]
fn ffi_call_trace_new(
    block_index: &FfiU256,
    tx_index: &FfiU256,
    call_trace_index: &FfiU256,
    events: &FfiList<FfiEvent>,
    extra: &FfiHashMap,
) -> repr_c::Box<FfiCallTrace> {
    let events = events
        .inner
        .iter()
        .map(|event| event.inner.clone())
        .collect();
    let extra = extra.inner.clone();

    repr_c::Box::new(FfiCallTrace {
        inner: CallTrace::new(
            block_index.inner.as_u128(),
            tx_index.inner.as_u128(),
            call_trace_index.inner.as_u128(),
            events,
            extra,
        ),
    })
}

#[derive_ReprC]
#[ReprC::opaque]
pub struct FfiHashMap {
    inner: HashMap<String, Value>,
}

#[ffi_export]
fn ffi_hash_map_new() -> repr_c::Box<FfiHashMap> {
    repr_c::Box::new(FfiHashMap {
        inner: HashMap::new(),
    })
}

/// Drops `value` argument
#[ffi_export]
fn ffi_hash_map_append(map: &mut FfiHashMap, key: char_p::Ref<'_>, value: repr_c::Box<FfiValue>) {
    let map = &mut map.inner;
    let key = key.to_string();
    let value = value.into().inner;

    map.insert(key, value);
}

#[derive_ReprC]
#[ReprC::opaque]
#[derive(Clone)]
pub struct FfiValue {
    inner: Value,
}

value_constructor!(FfiI256, Value::Int8, i8);
value_constructor!(FfiI256, Value::Int16, i16);
value_constructor!(FfiI256, Value::Int32, i32);
value_constructor!(FfiI256, Value::Int64, i64);
value_constructor!(FfiI256, Value::Int128, i128);
value_constructor!(FfiI256, Value::Int256, i256);

value_constructor!(FfiU256, Value::UInt8, u8);
value_constructor!(FfiU256, Value::UInt16, u16);
value_constructor!(FfiU256, Value::UInt32, u32);
value_constructor!(FfiU256, Value::UInt64, u64);
value_constructor!(FfiU256, Value::UInt128, u128);
value_constructor!(FfiU256, Value::UInt256, u256);

value_constructor!(FfiHashMap, Value::Object, object);

#[ffi_export]
fn ffi_value_binary_new(value: c_slice::Ref<u8>) -> repr_c::Box<FfiValue> {
    let value = value.iter().copied().collect();

    repr_c::Box::new(FfiValue {
        inner: Value::Binary(value),
    })
}

/// Drops `value` argument
#[ffi_export]
fn ffi_value_array_new(value: repr_c::Box<FfiList<FfiValue>>) -> repr_c::Box<FfiValue> {
    let value = value.inner.iter().map(|v| v.inner.clone()).collect();

    repr_c::Box::new(FfiValue {
        inner: Value::Array(value),
    })
}

#[ffi_export]
fn check_matches(transaction: &FfiTransaction) -> bool {
    let comparison = Comparison {
        left: ComparisonValue::Reference("$.events[0].block_index".into()),
        right: ComparisonValue::Value(Value::UInt128(U256::from(42u32))),
        operator: ComparisonOperator::Equal,
    };

    let rule = Rule::new(Expression::Comparison(comparison));

    rule.verify(&transaction.inner, None).unwrap().matched()
}

ffi_free!(repr_c::Box<FfiU256>, u256);
ffi_free!(repr_c::Box<FfiI256>, i256);
ffi_free!(repr_c::Box<FfiHashMap>, hash_map);

ffi_free!(repr_c::Box<FfiList<FfiTransaction>>, list_transactions);
ffi_free!(repr_c::Box<FfiList<FfiEvent>>, list_events);
ffi_free!(repr_c::Box<FfiList<FfiCallTrace>>, list_call_traces);

ffi_free!(repr_c::Box<FfiBlock>, block);
ffi_free!(repr_c::Box<FfiTransaction>, transaction);
ffi_free!(repr_c::Box<FfiEvent>, event);
ffi_free!(repr_c::Box<FfiCallTrace>, call_trace);

#[safer_ffi::cfg_headers]
#[test]
fn generate_headers() -> ::std::io::Result<()> {
    safer_ffi::headers::builder()
        .to_file("headers.h")?
        .generate()
}
