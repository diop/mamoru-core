use test_log::test;

use mamoru_aptos_types::{
    AptosCtx, Block, CallTrace, CallTraceArg, CallTraceTypeArg, Event, Transaction,
};
use mamoru_core::{BlockchainData, BlockchainDataBuilder, Value, ValueData};
use mamoru_core_test_utils::assembly_script::{
    AssemblyScriptModule, AS_APTOS_SDK_PATH, AS_SDK_PATH,
};
use mamoru_core_test_utils::test_daemon;

fn aptos_ctx() -> BlockchainData<AptosCtx> {
    let mut builder: BlockchainDataBuilder<AptosCtx> = BlockchainDataBuilder::new();

    builder.data_mut().set_block(Block {
        hash: "some-hash".to_string(),
        epoch: 1,
        timestamp_usecs: 2,
    });

    builder.data_mut().transactions.extend(vec![
        Transaction {
            seq: 3,
            block_hash: "some-hash".to_string(),
            hash: "another-some-hash".to_string(),
            event_root_hash: "event-root-hash".to_string(),
            state_change_hash: "state_change_hash".to_string(),
            gas_used: 4,
            max_gas_amount: 5,
            gas_unit_price: 6,
            expiration_timestamp_secs: 7,
            status: 8,
            sender: "some-sender".to_string(),
            sequence_number: 9,
        },
        Transaction {
            seq: 10,
            block_hash: "some-another-hash".to_string(),
            hash: "some-another-hash".to_string(),
            event_root_hash: "event-root-hash".to_string(),
            state_change_hash: "state_change_hash".to_string(),
            gas_used: 11,
            max_gas_amount: 12,
            gas_unit_price: 13,
            expiration_timestamp_secs: 14,
            status: 15,
            sender: "some-sender".to_string(),
            sequence_number: 16,
        },
    ]);

    builder.data_mut().events.extend(vec![Event {
        tx_seq: 17,
        key: "some-key".to_string(),
        sequence_number: 18,
        typ: "some-typ".to_string(),
        data: vec![19, 20, 21],
    }]);

    builder.data_mut().call_traces.extend(vec![
        CallTrace {
            seq: 0,
            tx_seq: 3,
            depth: 0,
            call_type: 0,
            gas_used: 500,
            transaction_module: Some("some-module".to_string()),
            function: "some-function".to_string(),
        },
        CallTrace {
            seq: 1,
            tx_seq: 42,
            depth: 2,
            call_type: 1,
            gas_used: 501,
            transaction_module: None,
            function: "some-function2".to_string(),
        },
    ]);

    builder.data_mut().call_trace_args.extend(vec![
        CallTraceArg {
            seq: 0,
            call_trace_seq: 0,
            arg: ValueData::new(Value::U64(42)).unwrap(),
        },
        CallTraceArg {
            seq: 1,
            call_trace_seq: 1,
            arg: ValueData::new(Value::String("forty-two".to_owned())).unwrap(),
        },
    ]);

    builder.data_mut().call_trace_type_args.extend(vec![
        CallTraceTypeArg {
            seq: 0,
            call_trace_seq: 0,
            arg: "Test1".to_string(),
        },
        CallTraceTypeArg {
            seq: 1,
            call_trace_seq: 1,
            arg: "Test2".to_string(),
        },
    ]);

    builder.build("test_tx", "test_digest").unwrap()
}

#[test(tokio::test)]
async fn smoke() {
    let ctx = aptos_ctx();

    let module = AssemblyScriptModule::with_deps(
        r#"""
        import {assert} from "@mamoru-ai/mamoru-sdk-as/assembly";
        import {AptosCtx} from "@mamoru-ai/mamoru-aptos-sdk-as/assembly";

        export function main(): void {
            const ctx = AptosCtx.load();

            const block = ctx.block;

            assert(block.hash == "some-hash", "block.hash == \"some-hash\"");
            assert(block.epoch == 1, "block.epoch == 1");
            assert(block.timestampUsecs == 2, "block.timestamp_usecs == 2");
            assert(block.txs.length == 1, "block.txs.length == 1");

            const tx1 = ctx.txs[0];

            assert(tx1.seq == 3, "tx1.seq == 3");
            assert(tx1.blockHash == "some-hash", "tx1.block_hash == \"some-hash\"");
            assert(tx1.hash == "another-some-hash", "tx1.hash == \"another-some-hash\"");
            assert(tx1.eventRootHash == "event-root-hash", "tx1.event_root_hash == \"event-root-hash\"");
            assert(tx1.stateChangeHash == "state_change_hash", "tx1.state_change_hash == \"state_change_hash\"");
            assert(tx1.gasUsed == 4, "tx1.gas_used == 4");
            assert(tx1.maxGasAmount == 5, "tx1.max_gas_amount == 5");
            assert(tx1.gasUnitPrice == 6, "tx1.gas_unit_price == 6");
            assert(tx1.expirationTimestampSecs == 7, "tx1.expiration_timestamp_secs == 7");
            assert(tx1.status == 8, "tx1.status == 8");
            assert(tx1.sender == "some-sender", "tx1.sender == \"some-sender\"");
            assert(tx1.sequenceNumber == 9, "tx1.sequence_number == 9");
            assert(tx1.callTraces.length == 1, "tx1.call_traces.length == 1");

            const tx2 = ctx.txs[1];

            assert(tx2.seq == 10, "tx2.seq == 10");
            assert(tx2.blockHash == "some-another-hash", "tx2.block_hash == \"some-another-hash\"");
            assert(tx2.hash == "some-another-hash", "tx2.hash == \"some-another-hash\"");
            assert(tx2.eventRootHash == "event-root-hash", "tx2.event_root_hash == \"event-root-hash\"");
            assert(tx2.stateChangeHash == "state_change_hash", "tx2.state_change_hash == \"state_change_hash\"");
            assert(tx2.gasUsed == 11, "tx2.gas_used == 11");
            assert(tx2.maxGasAmount == 12, "tx2.max_gas_amount == 12");
            assert(tx2.gasUnitPrice == 13, "tx2.gas_unit_price == 13");
            assert(tx2.expirationTimestampSecs == 14, "tx2.expiration_timestamp_secs == 14");
            assert(tx2.status == 15, "tx2.status == 15");
            assert(tx2.sender == "some-sender", "tx2.sender == \"some-sender\"");
            assert(tx2.sequenceNumber == 16, "tx2.sequence_number == 16");

            const event1 = ctx.events[0];

            assert(event1.txSeq == 17, "event1.tx_seq == 17");
            assert(event1.key == "some-key", "event1.key == \"some-key\"");
            assert(event1.sequenceNumber == 18, "event1.sequence_number == 18");
            assert(event1.typ == "some-typ", "event1.typ == \"some-typ\"");
            assert(event1.data.toString() == "19,20,21", "event1.data == [19, 20, 21]");

            const callTrace1 = ctx.callTraces[0];

            assert(callTrace1.seq == 0, "callTrace1.seq == 0");
            assert(callTrace1.txSeq == 3, "callTrace1.tx_seq == 3");
            assert(callTrace1.depth == 0, "callTrace1.depth == 0");
            assert(callTrace1.callType == 0, "callTrace1.call_type == 0");
            assert(callTrace1.gasUsed == 500, "callTrace1.gas_used == 500");
            assert(callTrace1.transactionModule == "some-module", "callTrace1.transaction_module == \"some-module\"");
            assert(callTrace1.func == "some-function", "callTrace1.function == \"some-function\"");
            assert(callTrace1.args.length == 1, "callTrace1.args.length == 1");
            assert(callTrace1.typeArgs.length == 1, "callTrace1.typeArgs.length == 1");

            const callTrace2 = ctx.callTraces[1];

            assert(callTrace2.seq == 1, "callTrace2.seq == 1");
            assert(callTrace2.txSeq == 42, "callTrace2.tx_seq == 42");
            assert(callTrace2.depth == 2, "callTrace2.depth == 2");
            assert(callTrace2.callType == 1, "callTrace2.call_type == 1");
            assert(callTrace2.gasUsed == 501, "callTrace2.gas_used == 501");
            assert(callTrace2.transactionModule == null, "callTrace2.transaction_module == null");
            assert(callTrace2.func == "some-function2", "callTrace2.function == \"some-function2\"");

            const callTraceArg1 = ctx.callTraceArgs[0];

            assert(callTraceArg1.seq == 0, "callTraceArg1.seq == 0");
            assert(callTraceArg1.callTraceSeq == 0, "callTraceArg1.call_trace_seq == 0");
            assert(callTraceArg1.value.asU64() == 42, "callTraceArg1.arg == 42");

            const callTraceArg2 = ctx.callTraceArgs[1];

            assert(callTraceArg2.seq == 1, "callTraceArg2.seq == 1");
            assert(callTraceArg2.callTraceSeq == 1, "callTraceArg2.call_trace_seq == 1");
            assert(callTraceArg2.value.asString() == "forty-two", "callTraceArg2.arg == \"forty-two\"");

            const callTraceTypeArg1 = ctx.callTraceTypeArgs[0];

            assert(callTraceTypeArg1.seq == 0, "callTraceTypeArg1.seq == 0");
            assert(callTraceTypeArg1.callTraceSeq == 0, "callTraceTypeArg1.call_trace_seq == 0");
            assert(callTraceTypeArg1.arg == "Test1", "callTraceTypeArg1.arg == \"Test1\"");

            const callTraceTypeArg2 = ctx.callTraceTypeArgs[1];

            assert(callTraceTypeArg2.seq == 1, "callTraceTypeArg2.seq == 1");
            assert(callTraceTypeArg2.callTraceSeq == 1, "callTraceTypeArg2.call_trace_seq == 1");
            assert(callTraceTypeArg2.arg == "Test2", "callTraceTypeArg2.arg == \"Test2\"");

        }
    """#,
        &[AS_SDK_PATH, AS_APTOS_SDK_PATH],
    );

    test_daemon(&module)
        .verify(&ctx)
        .await
        .expect("Assertion failed");
}
