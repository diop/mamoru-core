use maplit::hashmap;

pub use call_trace::*;
pub use events::*;
use mamoru_core::{
    serialize_data, BlockchainCtx, BlockchainSpecificImports, BlockchainTableItem, CtxImportError,
    CtxImportFn,
};
pub use transaction::*;

mod call_trace;
mod events;
mod transaction;

pub struct SuiCtx {
    pub tx: Option<Transaction>,
    pub call_traces: Vec<CallTrace>,
    pub call_trace_type_args: Vec<CallTraceTypeArg>,
    pub call_trace_args: Vec<CallTraceArg>,
    pub events: Vec<Event>,
}

impl SuiCtx {
    pub fn set_tx(&mut self, tx: Transaction) {
        self.tx = Some(tx);
    }
}

impl BlockchainCtx for SuiCtx {
    fn empty() -> Self {
        Self {
            tx: None,
            call_traces: Vec::new(),
            call_trace_type_args: Vec::new(),
            call_trace_args: Vec::new(),
            events: Vec::new(),
        }
    }

    fn module() -> &'static str {
        "mamoru_sui"
    }

    fn imports() -> BlockchainSpecificImports<Self> {
        hashmap! {
            "get_transactions" => CtxImportFn::NoArgs(|ctx: &Self| -> Vec<u8> {
                serialize_data(&[&ctx.tx])
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
            TransactionBatch::new(match &self.tx {
                Some(tx) => vec![tx.clone()],
                None => vec![],
            })
            .boxed(),
            CallTraceBatch::new(self.call_traces.clone()).boxed(),
            CallTraceTypeArgBatch::new(self.call_trace_type_args.clone()).boxed(),
            CallTraceArgBatch::new(self.call_trace_args.clone()).boxed(),
            EventBatch::new(self.events.clone()).boxed(),
        ]
    }
}
