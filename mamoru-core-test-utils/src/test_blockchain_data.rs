use datafusion::arrow::datatypes::{DataType, TimeUnit};
use maplit::hashmap;
use serde::{Deserialize, Serialize};

use blockchain_data_macro::BlockchainData;
use mamoru_core::{
    BlockchainCtx, BlockchainData, BlockchainDataBuilder, BlockchainSpecificImports,
    BlockchainTableItem, StructValue, Value, ValueData,
};

pub const TEST_ETH_TOPIC: &str = "442e715f626346e8c54381002da614f62bee8d27386535b2521ec8540898556e";

pub fn data_ctx(tx_hash: impl Into<String>) -> BlockchainData<TestCtx> {
    let (tx_seq, digest) = (42, tx_hash.into());
    let now = chrono::Utc::now().naive_utc();

    let mut builder: BlockchainDataBuilder<TestCtx> = BlockchainDataBuilder::new();
    let mut data = builder.data_mut();

    data.tx = Some(TestTransaction {
        seq: tx_seq,
        time: now.timestamp(),
        eth_topic: hex::decode(TEST_ETH_TOPIC).unwrap(),
        digest: digest.clone(),
        gas_used: 42_000,
    });

    data.call_traces.extend(vec![
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
    ]);

    data.call_trace_args.extend(vec![
        TestCallTraceArg {
            seq: 0,
            call_trace_seq: 1,
            arg: ValueData::new(Value::U64(42)).unwrap(),
        },
        TestCallTraceArg {
            seq: 1,
            call_trace_seq: 1,
            arg: ValueData::new(Value::Struct(StructValue::new(
                "dummy".to_string(),
                hashmap! {
                     "foo".to_string() => Value::U64(42),
                     "bar".to_string() => Value::String("hello".to_string()),
                },
            )))
            .unwrap(),
        },
        TestCallTraceArg {
            seq: 2,
            call_trace_seq: 1,
            arg: ValueData::new(Value::Bool(false)).unwrap(),
        },
    ]);

    builder.build(format!("{}", tx_seq), digest).unwrap()
}

#[derive(BlockchainData, Clone, Serialize, Deserialize)]
#[schema(table_name = "transactions")]
pub struct TestTransaction {
    #[schema(type = "DataType::UInt64")]
    pub seq: u64,

    #[schema(type = "DataType::Utf8")]
    pub digest: String,

    #[schema(type = "DataType::Binary")]
    pub eth_topic: Vec<u8>,

    #[schema(type = "DataType::Timestamp(TimeUnit::Second, None)")]
    pub time: i64,

    #[schema(type = "DataType::UInt64")]
    pub gas_used: u64,
}

#[derive(BlockchainData, Clone, Serialize, Deserialize)]
#[schema(table_name = "call_traces")]
pub struct TestCallTrace {
    #[schema(type = "DataType::UInt64")]
    pub seq: u64,

    #[schema(type = "DataType::UInt64")]
    pub tx_seq: u64,

    #[schema(type = "DataType::Utf8")]
    pub function: String,
}

#[derive(BlockchainData, Clone, Serialize, Deserialize)]
#[schema(table_name = "call_trace_args")]
pub struct TestCallTraceArg {
    #[schema(type = "DataType::UInt64")]
    pub seq: u64,

    #[schema(type = "DataType::UInt64")]
    pub call_trace_seq: u64,

    #[schema(type = "DataType::Binary")]
    pub arg: ValueData,
}

pub struct TestCtx {
    pub tx: Option<TestTransaction>,
    pub call_traces: Vec<TestCallTrace>,
    pub call_trace_args: Vec<TestCallTraceArg>,
}

impl TestCtx {
    pub fn set_tx(&mut self, tx: TestTransaction) {
        self.tx = Some(tx);
    }
}

impl BlockchainCtx for TestCtx {
    fn empty() -> Self {
        Self {
            tx: None,
            call_traces: vec![],
            call_trace_args: vec![],
        }
    }

    fn module() -> &'static str {
        "mamoru_test"
    }

    fn imports() -> BlockchainSpecificImports<Self> {
        hashmap! {}
    }

    fn as_tables(&self) -> Vec<Box<dyn BlockchainTableItem>> {
        vec![
            TestTransactionBatch::new(match &self.tx {
                Some(tx) => vec![tx.clone()],
                None => vec![],
            })
            .boxed(),
            TestCallTraceBatch::new(self.call_traces.clone()).boxed(),
            TestCallTraceArgBatch::new(self.call_trace_args.clone()).boxed(),
        ]
    }
}
