use maplit::hashmap;

pub use block::*;
pub use call_trace::*;
pub use event::*;
use mamoru_core::{
    serialize_data, BlockchainCtx, BlockchainSpecificImports, BlockchainTableItem, CtxImportError,
    CtxImportFn,
};
pub use transaction::*;

mod block;
mod call_trace;
mod event;
mod transaction;

pub struct AptosCtx {
    pub block: Option<Block>,
    pub transactions: Vec<Transaction>,
    pub events: Vec<Event>,
    pub call_traces: Vec<CallTrace>,
    pub call_trace_type_args: Vec<CallTraceTypeArg>,
    pub call_trace_args: Vec<CallTraceArg>,
}

impl AptosCtx {
    pub fn set_block(&mut self, block: Block) {
        self.block = Some(block);
    }
}

impl BlockchainCtx for AptosCtx {
    fn empty() -> Self {
        Self {
            block: None,
            transactions: vec![],
            events: vec![],
            call_traces: vec![],
            call_trace_type_args: vec![],
            call_trace_args: vec![],
        }
    }

    fn module() -> &'static str {
        "mamoru_aptos"
    }

    fn imports() -> BlockchainSpecificImports<Self> {
        hashmap! {
            "get_blocks" => CtxImportFn::NoArgs(|ctx: &Self| -> Vec<u8> {
                serialize_data(&[&ctx.block])
            }),
            "get_transactions" => CtxImportFn::NoArgs(|ctx: &Self| -> Vec<u8> {
                serialize_data(&ctx.transactions)
            }),
            "get_events" => CtxImportFn::NoArgs(|ctx: &Self| -> Vec<u8> {
                serialize_data(&ctx.events)
            }),
            "get_call_traces" => CtxImportFn::NoArgs(|ctx: &Self| -> Vec<u8> {
                serialize_data(&ctx.call_traces)
            }),
            "get_call_trace_type_args" => CtxImportFn::NoArgs(|ctx: &Self| -> Vec<u8> {
                serialize_data(&ctx.call_trace_type_args)
            }),
            "get_call_trace_args" => CtxImportFn::NoArgs(|ctx: &Self| -> Vec<u8> {
                serialize_data(&ctx.call_trace_args)
            }),
            "get_call_trace_arg_by_id" => CtxImportFn::ById(|ctx: &Self, seq: u64| -> Result<&[u8], CtxImportError> {
                let arg = ctx.call_trace_args
                    .iter()
                    .find(|arg| arg.seq == seq)
                    .ok_or_else(|| CtxImportError::ById("Failed to find call trace arg".to_string()))?;

                Ok(arg.arg.as_ref())
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
            EventBatch::new(self.events.clone()).boxed(),
            CallTraceBatch::new(self.call_traces.clone()).boxed(),
            CallTraceTypeArgBatch::new(self.call_trace_type_args.clone()).boxed(),
            CallTraceArgBatch::new(self.call_trace_args.clone()).boxed(),
        ]
    }
}
