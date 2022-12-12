use crate::rule::test_blockchain_data::{
    TestCallTrace, TestCallTraceBatch, TestTransaction, TestTransactionBatch,
};
use mamoru_core::{BlockchainDataCtx, BlockchainDataCtxBuilder, Rule};

pub(crate) mod test_blockchain_data;

mod smoke;

pub(crate) fn data_ctx(tx_hash: impl Into<String>) -> BlockchainDataCtx {
    let (tx_seq, digest) = (42, tx_hash.into());
    let now = chrono::Utc::now().naive_utc();

    let builder = BlockchainDataCtxBuilder::new();

    builder
        .add_data(TestTransactionBatch::new(vec![TestTransaction {
            seq: tx_seq,
            time: now.timestamp(),
            hash: digest.clone(),
            gas_used: 42_000,
        }]))
        .unwrap();

    builder
        .add_data(TestCallTraceBatch::new(vec![
            TestCallTrace {
                seq: 0,
                tx_seq,
                function: "func1".to_string(),
            },
            TestCallTrace {
                seq: 1,
                tx_seq,
                function: "func2".to_string(),
            },
        ]))
        .unwrap();

    builder.finish(format!("{}", tx_seq), digest, now)
}

pub fn active_rule(expression: impl AsRef<str>) -> Rule {
    Rule::new("dummy".to_string(), 0, i64::MAX, expression.as_ref())
        .expect("Failed to create rule.")
}

pub fn inactive_rule(expression: impl AsRef<str>) -> Rule {
    Rule::new("dummy".to_string(), 0, 0, expression.as_ref()).expect("Failed to create rule.")
}
