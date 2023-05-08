use maplit::hashmap;

pub use block::*;
pub use call_trace::*;
pub use event::*;
use mamoru_core::{
    serialize_data, BlockchainCtx, BlockchainSpecificImports, BlockchainTableItem, CtxImportFn,
};
pub use transaction::*;

mod block;
mod call_trace;
mod event;
mod transaction;

pub struct EvmCtx {
    pub block: Option<Block>,
    pub transactions: Vec<Transaction>,
    pub call_traces: Vec<CallTrace>,
    pub events: Vec<Event>,
}

impl EvmCtx {
    pub fn set_block(&mut self, block: Block) {
        self.block = Some(block);
    }
}

impl BlockchainCtx for EvmCtx {
    fn empty() -> Self {
        Self {
            block: None,
            transactions: vec![],
            call_traces: vec![],
            events: vec![],
        }
    }

    fn module() -> &'static str {
        "mamoru_evm"
    }

    fn imports() -> BlockchainSpecificImports<Self> {
        hashmap! {
            "get_blocks" => CtxImportFn::NoArgs(|ctx: &Self| -> Vec<u8> {
                serialize_data(&[&ctx.block])
            }),
            "get_transactions" => CtxImportFn::NoArgs(|ctx: &Self| -> Vec<u8> {
                serialize_data(&ctx.transactions)
            }),
            "get_call_traces" => CtxImportFn::NoArgs(|ctx: &Self| -> Vec<u8> {
                serialize_data(&ctx.call_traces)
            }),
            "get_events" => CtxImportFn::NoArgs(|ctx: &Self| -> Vec<u8> {
                serialize_data(&ctx.events)
            }),
        }
    }

    fn as_tables(&self) -> Vec<Box<dyn BlockchainTableItem>> {
        vec![
            BlockBatch::new(match &self.block {
                Some(block) => vec![block.clone()],
                None => vec![],
            })
            .boxed(),
            TransactionBatch::new(self.transactions.clone()).boxed(),
            CallTraceBatch::new(self.call_traces.clone()).boxed(),
            EventBatch::new(self.events.clone()).boxed(),
        ]
    }
}
