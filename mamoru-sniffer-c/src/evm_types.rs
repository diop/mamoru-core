use crate::ffi_types::FfiBlockchainData;
use mamoru_evm_types::{
    Block, BlockBatch, CallTrace, CallTraceBatch, Event, EventBatch, Transaction, TransactionBatch,
};
use safer_ffi::prelude::*;

// Transaction
#[derive_ReprC]
#[ReprC::opaque]
pub struct FfiTransactionBatch {
    pub inner: Box<TransactionBatch>,
}

#[ffi_export]
fn new_transaction_batch() -> repr_c::Box<FfiTransactionBatch> {
    let inner = Box::new(TransactionBatch(Vec::new()));

    repr_c::Box::new(FfiTransactionBatch { inner })
}

#[ffi_export]
fn transaction_batch_append<'a>(
    batch: &mut FfiTransactionBatch,
    tx_index: u32,
    tx_hash: char_p::Ref<'a>,
    block_index: u64,
    typ: u8,
    nonce: u64,
    status: u64,
    from: char_p::Ref<'a>,
    to: Option<char_p::Ref<'a>>,
    value: u64,
    fee: u64,
    gas_price: u64,
    gas_limit: u64,
    gas_used: u64,
    input: c_slice::Ref<'a, u8>,
    size: f64,
) {
    let batch = &mut batch.inner.0;

    batch.push(Transaction {
        tx_index,
        tx_hash: tx_hash.to_str().to_string(),
        typ,
        nonce,
        status,
        block_index,
        from: from.to_str().to_string(),
        to: to.map(|to| to.to_str().to_string()),
        value,
        fee,
        gas_price,
        gas_limit,
        gas_used,
        input: input.to_vec(),
        size,
    });
}

/// Frees `object` argument.
#[ffi_export]
fn transaction_batch_finish(
    object: repr_c::Box<FfiTransactionBatch>,
) -> repr_c::Box<FfiBlockchainData> {
    let inner = object.into().inner;

    repr_c::Box::new(FfiBlockchainData { inner })
}

// CallTrace
#[derive_ReprC]
#[ReprC::opaque]
pub struct FfiCallTraceBatch {
    pub inner: Box<CallTraceBatch>,
}

#[ffi_export]
fn new_call_trace_batch() -> repr_c::Box<FfiCallTraceBatch> {
    let inner = Box::new(CallTraceBatch(Vec::new()));
    repr_c::Box::new(FfiCallTraceBatch { inner })
}

#[ffi_export]
fn call_trace_batch_append<'a>(
    batch: &mut FfiCallTraceBatch,
    seq: u32,
    depth: u32,
    tx_index: u32,
    block_index: u64,
    typ: char_p::Ref<'a>,
    from: char_p::Ref<'a>,
    to: char_p::Ref<'a>,
    value: u64,
    gas_limit: u64,
    gas_used: u64,
    input: c_slice::Ref<'a, u8>,
) {
    let batch = &mut batch.inner.0;
    batch.push(CallTrace {
        seq,
        tx_index,
        block_index,
        depth,
        typ: typ.to_str().to_string(),
        from: from.to_str().to_string(),
        to: to.to_str().to_string(),
        value,
        gas_limit,
        gas_used,
        input: input.to_vec(),
    });
}

/// Frees `object` argument.
#[ffi_export]
fn call_trace_batch_finish(
    object: repr_c::Box<FfiCallTraceBatch>,
) -> repr_c::Box<FfiBlockchainData> {
    let inner = object.into().inner;

    repr_c::Box::new(FfiBlockchainData { inner })
}

// Block
#[derive_ReprC]
#[ReprC::opaque]
pub struct FfiBlockBatch {
    pub inner: Box<BlockBatch>,
}

#[ffi_export]
fn new_block_batch() -> repr_c::Box<FfiBlockBatch> {
    let inner = Box::new(BlockBatch(Vec::new()));
    repr_c::Box::new(FfiBlockBatch { inner })
}

#[ffi_export]
fn block_batch_append<'a>(
    batch: &mut FfiBlockBatch,
    block_index: u64,
    hash: char_p::Ref<'a>,
    parent_hash: char_p::Ref<'a>,
    state_root: char_p::Ref<'a>,
    nonce: u64,
    status: char_p::Ref<'a>,
    timestamp: u64,
    block_reward: c_slice::Ref<'a, u8>,
    fee_recipient: char_p::Ref<'a>,
    total_difficulty: u64,
    size: f64,
    gas_used: u64,
    gas_limit: u64,
) {
    let batch = &mut batch.inner.0;

    batch.push(Block {
        block_index,
        hash: hash.to_str().to_string(),
        parent_hash: parent_hash.to_str().to_string(),
        state_root: state_root.to_str().to_string(),
        nonce,
        status: status.to_str().to_string(),
        timestamp,
        block_reward: block_reward.to_vec(),
        fee_recipient: fee_recipient.to_str().to_string(),
        total_difficulty,
        size,
        gas_used,
        gas_limit,
    });
}

/// Frees `object` argument.
#[ffi_export]
fn block_batch_finish(object: repr_c::Box<FfiBlockBatch>) -> repr_c::Box<FfiBlockchainData> {
    let inner = object.into().inner;
    repr_c::Box::new(FfiBlockchainData { inner })
}

// Event
#[derive_ReprC]
#[ReprC::opaque]
pub struct FfiEventBatch {
    pub inner: Box<EventBatch>,
}

#[ffi_export]
fn new_event_batch() -> repr_c::Box<FfiEventBatch> {
    let inner = Box::new(EventBatch(Vec::new()));
    repr_c::Box::new(FfiEventBatch { inner })
}

#[ffi_export]
fn event_batch_append<'a>(
    batch: &mut FfiEventBatch,
    index: u32,
    address: char_p::Ref<'a>,
    block_number: u64,
    tx_hash: char_p::Ref<'a>,
    tx_index: u32,
    block_hash: char_p::Ref<'a>,
    topic0: c_slice::Ref<'a, u8>,
    topic1: c_slice::Ref<'a, u8>,
    topic2: c_slice::Ref<'a, u8>,
    topic3: c_slice::Ref<'a, u8>,
    topic4: c_slice::Ref<'a, u8>,
    data: c_slice::Ref<'a, u8>,
) {
    let batch = &mut batch.inner.0;
    batch.push(Event {
        index,
        address: address.to_str().to_string(),
        block_number,
        tx_hash: tx_hash.to_str().to_string(),
        tx_index,
        block_hash: block_hash.to_str().to_string(),
        topic0: topic0.to_vec(),
        topic1: topic1.to_vec(),
        topic2: topic2.to_vec(),
        topic3: topic3.to_vec(),
        topic4: topic4.to_vec(),
        data: data.to_vec(),
    });
}

/// Frees `object` argument.
#[ffi_export]
fn event_batch_finish(object: repr_c::Box<FfiEventBatch>) -> repr_c::Box<FfiBlockchainData> {
    let inner = object.into().inner;
    repr_c::Box::new(FfiBlockchainData { inner })
}
