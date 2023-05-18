use crate::{FfiSniffer, RUNTIME};
use mamoru_evm_types::{Block, CallTrace, Event, EvmCtx, Transaction};
use mamoru_sniffer::core::{BlockchainData, BlockchainDataBuilder};
use safer_ffi::prelude::*;

#[derive_ReprC]
#[ReprC::opaque]
pub struct FfiEvmBlockchainDataCtx {
    pub(crate) inner: BlockchainData<EvmCtx>,
}

#[derive_ReprC]
#[ReprC::opaque]
pub struct FfiEvmBlockchainDataBuilder {
    pub(crate) inner: BlockchainDataBuilder<EvmCtx>,
}

#[ffi_export]
fn new_evm_blockchain_data_builder() -> repr_c::Box<FfiEvmBlockchainDataBuilder> {
    let inner = BlockchainDataBuilder::new();
    repr_c::Box::new(FfiEvmBlockchainDataBuilder { inner })
}

#[ffi_export]
fn evm_blockchain_data_builder_set_tx(
    builder: &mut FfiEvmBlockchainDataBuilder,
    tx_id: char_p::Ref<'_>,
    tx_hash: char_p::Ref<'_>,
) {
    let tx_id = tx_id.to_str();
    let tx_hash = tx_hash.to_str();

    builder.inner.set_tx_data(tx_id, tx_hash);
}

#[ffi_export]
fn evm_blockchain_data_builder_set_block(
    builder: &mut FfiEvmBlockchainDataBuilder,
    block_id: char_p::Ref<'_>,
    block_hash: char_p::Ref<'_>,
) {
    let block_id = block_id.to_str();
    let block_hash = block_hash.to_str();

    builder.inner.set_block_data(block_id, block_hash);
}

#[ffi_export]
fn evm_blockchain_data_builder_set_mempool_source(builder: &mut FfiEvmBlockchainDataBuilder) {
    builder.inner.set_mempool_source();
}

/// Frees `builder` argument.
#[ffi_export]
fn evm_blockchain_data_builder_finish(
    builder: repr_c::Box<FfiEvmBlockchainDataBuilder>,
) -> repr_c::Box<FfiEvmBlockchainDataCtx> {
    let builder = builder.into().inner;

    repr_c::Box::new(FfiEvmBlockchainDataCtx {
        inner: builder
            .build()
            .expect("BUG: failed to build `BlockchainData`"),
    })
}

/// Frees `data` argument.
#[ffi_export]
fn evm_sniffer_observe_data(sniffer: &FfiSniffer, data: repr_c::Box<FfiEvmBlockchainDataCtx>) {
    let sniffer = &sniffer.inner;
    let data = data.into();

    RUNTIME.block_on(async { sniffer.observe_data(data.inner).await });
}

#[ffi_export]
fn evm_transaction_append<'a>(
    builder: &mut FfiEvmBlockchainDataBuilder,
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
    let transactions = &mut builder.inner.data_mut().transactions;

    transactions.push(Transaction {
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

#[ffi_export]
fn evm_call_trace_append<'a>(
    builder: &mut FfiEvmBlockchainDataBuilder,
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
    let call_traces = &mut builder.inner.data_mut().call_traces;

    call_traces.push(CallTrace {
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

#[ffi_export]
fn evm_block_set<'a>(
    builder: &mut FfiEvmBlockchainDataBuilder,
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
    let data = builder.inner.data_mut();

    data.set_block(Block {
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

#[ffi_export]
fn evm_event_append<'a>(
    builder: &mut FfiEvmBlockchainDataBuilder,
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
    let events = &mut builder.inner.data_mut().events;

    events.push(Event {
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
