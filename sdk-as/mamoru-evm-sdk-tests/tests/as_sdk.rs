use test_log::test;

use mamoru_core::{BlockchainData, BlockchainDataBuilder};
use mamoru_core_test_utils::assembly_script::{AssemblyScriptModule, AS_EVM_SDK_PATH, AS_SDK_PATH};
use mamoru_core_test_utils::test_daemon;
use mamoru_evm_types::{Block, CallTrace, Event, EvmCtx, Transaction};

// mint(address,uint256) (0x55fe002aeff02f77364de339a1292923a15844b8,1200000000)
const TX1_INPUT: &str = "40c10f1900000000000000000000000055fe002aeff02f77364de339a1292923a15844b80000000000000000000000000000000000000000000000000000000047868c00";

fn evm_ctx() -> BlockchainData<EvmCtx> {
    let mut builder: BlockchainDataBuilder<EvmCtx> = BlockchainDataBuilder::new();

    builder.data_mut().set_block(Block {
        block_index: 0,
        hash: "some-hash".to_string(),
        parent_hash: "some-parent-hash".to_string(),
        state_root: "some-state-root".to_string(),
        nonce: 1,
        status: "some-status".to_string(),
        timestamp: 2,
        block_reward: vec![3, 4, 5],
        fee_recipient: "some-fee-recipient".to_string(),
        total_difficulty: 6,
        size: 7.0,
        gas_used: 8,
        gas_limit: 9,
    });

    builder.data_mut().transactions.extend(vec![
        Transaction {
            tx_index: 10,
            tx_hash: "some-tx-hash".to_string(),
            typ: 11,
            nonce: 12,
            status: 13,
            block_index: 14,
            from: "some-from".to_string(),
            to: Some("some-to".to_string()),
            value: 15,
            fee: 16,
            gas_price: 17,
            gas_limit: 18,
            gas_used: 19,
            input: hex::decode(TX1_INPUT).unwrap(),
            size: 23.0,
        },
        Transaction {
            tx_index: 24,
            tx_hash: "another-some-hash".to_string(),
            typ: 25,
            nonce: 26,
            status: 27,
            block_index: 28,
            from: "some-sender".to_string(),
            to: None,
            value: 29,
            fee: 30,
            gas_price: 31,
            gas_limit: 32,
            gas_used: 33,
            input: vec![34, 35, 36],
            size: 37.0,
        },
    ]);

    builder.data_mut().events.extend(vec![
        Event {
            index: 38,
            tx_index: 39,
            tx_hash: "some-tx-hash".to_string(),
            block_number: 40,
            block_hash: "some-block-hash".to_string(),
            address: "some-address".to_string(),
            topic0: vec![41, 42, 43],
            topic1: vec![44, 45, 46],
            topic2: vec![47, 48, 49],
            topic3: vec![50, 51, 52],
            topic4: vec![53, 54, 55],
            data: vec![56, 57, 58],
        },
        Event {
            index: 59,
            tx_index: 60,
            tx_hash: "some-tx-hash".to_string(),
            block_number: 61,
            block_hash: "some-block-hash".to_string(),
            address: "some-address".to_string(),
            topic0: vec![62, 63, 64],
            topic1: vec![65, 66, 67],
            topic2: vec![68, 69, 70],
            topic3: vec![71, 72, 73],
            topic4: vec![74, 75, 76],
            data: vec![77, 78, 79],
        },
    ]);

    builder.data_mut().call_traces.extend(vec![
        CallTrace {
            seq: 80,
            tx_index: 81,
            block_index: 82,
            depth: 83,
            typ: "some-type".to_string(),
            from: "some-from".to_string(),
            to: "some-to".to_string(),
            value: 84,
            gas_limit: 85,
            gas_used: 86,
            input: vec![87, 88, 89],
        },
        CallTrace {
            seq: 90,
            tx_index: 91,
            block_index: 92,
            depth: 93,
            typ: "another-type".to_string(),
            from: "another-from".to_string(),
            to: "another-to".to_string(),
            value: 94,
            gas_limit: 95,
            gas_used: 96,
            input: vec![97, 98, 99],
        },
    ]);

    builder.set_tx_data("test_tx", "test_digest");
    builder.build().unwrap()
}

#[test(tokio::test)]
async fn smoke() {
    let ctx = evm_ctx();

    let module = AssemblyScriptModule::with_deps(
        r#"""
        import {assert, u256} from "@mamoru-ai/mamoru-sdk-as/assembly";
        import {EvmCtx} from "@mamoru-ai/mamoru-evm-sdk-as/assembly";

        export function main(): void {
            const ctx = EvmCtx.load();

            const block = ctx.block;

            assert(block.blockIndex == 0, "block.block_index == 0");
            assert(block.hash == "some-hash", "block.hash == \"some-hash\"");
            assert(block.parentHash == "some-parent-hash", "block.parent_hash == \"some-parent-hash\"");
            assert(block.stateRoot == "some-state-root", "block.state_root == \"some-state-root\"");
            assert(block.nonce == 1, "block.nonce == 1");
            assert(block.status == "some-status", "block.status == \"some-status\"");
            assert(block.timestamp == 2, "block.timestamp == 2");
            assert(block.blockReward.toString() == "3,4,5", "block.block_reward == [3, 4, 5]");
            assert(block.feeRecipient == "some-fee-recipient", "block.fee_recipient == \"some-fee-recipient\"");
            assert(block.totalDifficulty == 6, "block.total_difficulty == 6");
            assert(block.size == 7.0, "block.size == 7.0");
            assert(block.gasUsed == 8, "block.gas_used == 8");
            assert(block.gasLimit == 9, "block.gas_limit == 9");

            const tx1 = ctx.txs[0];

            assert(tx1.txIndex == 10, "tx1.tx_index == 10");
            assert(tx1.txHash == "some-tx-hash", "tx1.hash == \"some-tx-hash\"");
            assert(tx1.typ == 11, "tx1.type == 11");
            assert(tx1.nonce == 12, "tx1.nonce == 12");
            assert(tx1.status == 13, "tx1.status == 13");
            assert(tx1.blockIndex == 14, "tx1.block_index == 14");
            assert(tx1.from == "some-from", "tx1.from == \"some-from\"");
            assert(tx1.to == "some-to", "tx1.to == \"some-to\"");
            assert(tx1.value == 15, "tx1.value == 15");
            assert(tx1.fee == 16, "tx1.fee == 16");
            assert(tx1.gasPrice == 17, "tx1.gas_price == 17");
            assert(tx1.gasLimit == 18, "tx1.gas_limit == 18");
            assert(tx1.gasUsed == 19, "tx1.gas_used == 19");

            const tx1Input = tx1.input.parse("mint(address,uint256)");

            if (tx1Input == null) {
                assert(false, "tx1.input == null");
                return;
            }

            assert(tx1Input[0].asAddress() == "0x55fe002aeff02f77364de339a1292923a15844b8", "address == \"0x55fe002aeff02f77364de339a1292923a15844b8\"");
            assert(tx1Input[1].asUint() == u256.fromU64(1200000000), "num == 1200000000");
            assert(tx1.size == 23.0, "tx1.size == 23.0");

            const tx2 = ctx.txs[1];

            assert(tx2.txIndex == 24, "tx2.tx_index == 24");
            assert(tx2.txHash == "another-some-hash", "tx2.hash == \"another-some-hash\"");
            assert(tx2.typ == 25, "tx2.type == 25");
            assert(tx2.nonce == 26, "tx2.nonce == 26");
            assert(tx2.status == 27, "tx2.status == 27");
            assert(tx2.blockIndex == 28, "tx2.block_index == 28");
            assert(tx2.from == "some-sender", "tx2.from == \"some-sender\"");
            assert(tx2.to == null, "tx2.to == null");
            assert(tx2.value == 29, "tx2.value == 29");
            assert(tx2.fee == 30, "tx2.fee == 30");
            assert(tx2.gasPrice == 31, "tx2.gas_price == 31");
            assert(tx2.gasLimit == 32, "tx2.gas_limit == 32");
            assert(tx2.gasUsed == 33, "tx2.gas_used == 33");
            assert(tx2.input.data.toString() == "34,35,36", "tx2.input == [34, 35, 36]");
            assert(tx2.size == 37.0, "tx2.size == 37.0");

            const event1 = ctx.events[0];

            assert(event1.index == 38, "event1.index == 38");
            assert(event1.txIndex == 39, "event1.tx_index == 39");
            assert(event1.txHash == "some-tx-hash", "event1.tx_hash == \"some-tx-hash\"");
            assert(event1.blockNumber == 40, "event1.block_number == 40");
            assert(event1.blockHash == "some-block-hash", "event1.block_hash == \"some-block-hash\"");
            assert(event1.address == "some-address", "event1.address == \"some-address\"");
            assert(event1.topic0.toString() == "41,42,43", "event1.topic0 == [41, 42, 43]");
            assert(event1.topic1.toString() == "44,45,46", "event1.topic1 == [44, 45, 46]");
            assert(event1.topic2.toString() == "47,48,49", "event1.topic2 == [47, 48, 49]");
            assert(event1.topic3.toString() == "50,51,52", "event1.topic3 == [50, 51, 52]");
            assert(event1.topic4.toString() == "53,54,55", "event1.topic4 == [53, 54, 55]");
            assert(event1.data.toString() == "56,57,58", "event1.data == [56, 57, 58]");

            const event2 = ctx.events[1];

            assert(event2.index == 59, "event2.index == 59");
            assert(event2.txIndex == 60, "event2.tx_index == 60");
            assert(event2.txHash == "some-tx-hash", "event2.tx_hash == \"some-tx-hash\"");
            assert(event2.blockNumber == 61, "event2.block_number == 61");
            assert(event2.blockHash == "some-block-hash", "event2.block_hash == \"some-block-hash\"");
            assert(event2.address == "some-address", "event2.address == \"some-address\"");
            assert(event2.topic0.toString() == "62,63,64", "event2.topic0 == [62, 63, 64]");
            assert(event2.topic1.toString() == "65,66,67", "event2.topic1 == [65, 66, 67]");
            assert(event2.topic2.toString() == "68,69,70", "event2.topic2 == [68, 69, 70]");
            assert(event2.topic3.toString() == "71,72,73", "event2.topic3 == [71, 72, 73]");
            assert(event2.topic4.toString() == "74,75,76", "event2.topic4 == [74, 75, 76]");
            assert(event2.data.toString() == "77,78,79", "event2.data == [77, 78, 79]");

            const callTrace1 = ctx.callTraces[0];

            assert(callTrace1.seq == 80, "callTrace1.seq == 80");
            assert(callTrace1.txIndex == 81, "callTrace1.tx_index == 81");
            assert(callTrace1.blockIndex == 82, "callTrace1.block_index == 82");
            assert(callTrace1.depth == 83, "callTrace1.depth == 83");
            assert(callTrace1.typ == "some-type", "callTrace1.typ == \"some-type\"");
            assert(callTrace1.from == "some-from", "callTrace1.from == \"some-from\"");
            assert(callTrace1.to == "some-to", "callTrace1.to == \"some-to\"");
            assert(callTrace1.value == 84, "callTrace1.value == 84");
            assert(callTrace1.gasLimit == 85, "callTrace1.gas_limit == 85");
            assert(callTrace1.gasUsed == 86, "callTrace1.gas_used == 86");
            assert(callTrace1.input.data.toString() == "87,88,89", "callTrace1.input == [87, 88, 89]");

            const callTrace2 = ctx.callTraces[1];

            assert(callTrace2.seq == 90, "callTrace2.seq == 90");
            assert(callTrace2.txIndex == 91, "callTrace2.tx_index == 91");
            assert(callTrace2.blockIndex == 92, "callTrace2.block_index == 92");
            assert(callTrace2.depth == 93, "callTrace2.depth == 93");
            assert(callTrace2.typ == "another-type", "callTrace2.typ == \"another-type\"");
            assert(callTrace2.from == "another-from", "callTrace2.from == \"another-from\"");
            assert(callTrace2.to == "another-to", "callTrace2.to == \"another-to\"");
            assert(callTrace2.value == 94, "callTrace2.value == 94");
            assert(callTrace2.gasLimit == 95, "callTrace2.gas_limit == 95");
            assert(callTrace2.gasUsed == 96, "callTrace2.gas_used == 96");
            assert(callTrace2.input.data.toString() == "97,98,99", "callTrace2.input == [97, 98, 99]");
        }
    """#,
        &[AS_SDK_PATH, AS_EVM_SDK_PATH],
    );

    test_daemon(&module)
        .verify(&ctx)
        .await
        .expect("Assertion failed");
}
