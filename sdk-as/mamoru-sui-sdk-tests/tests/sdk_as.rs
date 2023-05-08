use test_log::test;

use mamoru_core::{BlockchainData, BlockchainDataBuilder, Value, ValueData};
use mamoru_core_test_utils::assembly_script::{AssemblyScriptModule, AS_SDK_PATH, AS_SUI_SDK_PATH};
use mamoru_core_test_utils::test_daemon;
use mamoru_sui_types::{CallTrace, CallTraceArg, CallTraceTypeArg, Event, SuiCtx, Transaction};

fn sui_ctx() -> BlockchainData<SuiCtx> {
    let mut builder: BlockchainDataBuilder<SuiCtx> = BlockchainDataBuilder::new();

    builder.data_mut().set_tx(Transaction {
        seq: 42,
        digest: "some-digest".to_string(),
        time: 43,
        gas_used: 44,
        gas_computation_cost: 45,
        gas_storage_cost: 46,
        gas_budget: 47,
        sender: "some-sender".to_string(),
        kind: "some-kind".to_string(),
    });

    builder.data_mut().events.extend(vec![
        Event {
            tx_seq: 42,
            package_id: "some-package-id".to_string(),
            transaction_module: "some-module".to_string(),
            sender: "some-sender".to_string(),
            typ: "some-typ".to_string(),
            contents: vec![49, 50, 51],
        },
        Event {
            tx_seq: 42,
            package_id: "some-package-id".to_string(),
            transaction_module: "some-module".to_string(),
            sender: "some-sender".to_string(),
            typ: "some-typ".to_string(),
            contents: vec![52, 53, 54],
        },
    ]);

    builder.data_mut().call_traces.extend(vec![
        CallTrace {
            seq: 0,
            tx_seq: 42,
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
    let ctx = sui_ctx();

    let module = AssemblyScriptModule::with_deps(
        r#"""
        import {assert} from "@mamoru-ai/mamoru-sdk-as/assembly";
        import {SuiCtx} from "@mamoru-ai/mamoru-sui-sdk-as/assembly";

        export function main(): void {
            const ctx = SuiCtx.load();

            const tx = ctx.tx;

            assert(tx.seq == 42, "tx.seq == 42");
            assert(tx.digest == "some-digest", "tx.digest == \"some-digest\"");
            assert(tx.time == 43, "tx.time == 43");
            assert(tx.gasUsed == 44, "tx.gas_used == 44");
            assert(tx.gasComputationCost == 45, "tx.gas_computation_cost == 45");
            assert(tx.gasStorageCost == 46, "tx.gas_storage_cost == 46");
            assert(tx.gasBudget == 47, "tx.gas_budget == 47");
            assert(tx.sender == "some-sender", "tx.sender == \"some-sender\"");
            assert(tx.kind == "some-kind", "tx.kind == \"some-kind\"");

            const event1 = ctx.events[0];

            assert(event1.txSeq == 42, "event1.tx_seq == 42");
            assert(event1.packageId == "some-package-id", "event1.package_id == \"some-package-id\"");
            assert(event1.transactionModule == "some-module", "event1.transaction_module == \"some-module\"");
            assert(event1.sender == "some-sender", "event1.sender == \"some-sender\"");
            assert(event1.typ == "some-typ", "event1.typ == \"some-typ\"");
            assert(event1.contents.toString() == "49,50,51", "event1.contents == [49, 50, 51]");

            const event2 = ctx.events[1];

            assert(event2.txSeq == 42, "event2.tx_seq == 42");
            assert(event2.packageId == "some-package-id", "event2.package_id == \"some-package-id\"");
            assert(event2.transactionModule == "some-module", "event2.transaction_module == \"some-module\"");
            assert(event2.sender == "some-sender", "event2.sender == \"some-sender\"");
            assert(event2.typ == "some-typ", "event2.typ == \"some-typ\"");
            assert(event2.contents.toString() == "52,53,54", "event2.contents == [52, 53, 54]");

            const callTrace1 = ctx.callTraces[0];

            assert(callTrace1.seq == 0, "callTrace1.seq == 0");
            assert(callTrace1.txSeq == 42, "callTrace1.tx_seq == 42");
            assert(callTrace1.depth == 0, "callTrace1.depth == 0");
            assert(callTrace1.callType == 0, "callTrace1.call_type == 0");
            assert(callTrace1.gasUsed == 500, "callTrace1.gas_used == 500");
            assert(callTrace1.transactionModule == "some-module", "callTrace1.transaction_module == \"some-module\"");
            assert(callTrace1.func == "some-function", "callTrace1.function == \"some-function\"");

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
            assert(callTraceArg1.value.asU64()!.value == 42, "callTraceArg1.arg == 42");

            const callTraceArg2 = ctx.callTraceArgs[1];

            assert(callTraceArg2.seq == 1, "callTraceArg2.seq == 1");
            assert(callTraceArg2.callTraceSeq == 1, "callTraceArg2.call_trace_seq == 1");
            assert(callTraceArg2.value.asString()!.value == "forty-two", "callTraceArg2.arg == \"forty-two\"");

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
        &[AS_SDK_PATH, AS_SUI_SDK_PATH],
    );

    test_daemon(&module)
        .verify(&ctx)
        .await
        .expect("Assertion failed");
}
