#![allow(clippy::redundant_clone)]

mod common;
use common::*;

fn create_block_15705350(transactions: Vec<Transaction>) -> Block {
    Block::new(
        15705350,
        1665255959,
        transactions,
        HashMap::from([
            (
                "hash".to_string(),
                Value::Binary(string_to_binary_vec(
                    "0xa3f8ea762b59b9ebd1e373c1acf42dc8d624365492688716f47816c8660ec9f0",
                )),
            ),
            (
                "parent_hash".to_string(),
                Value::Binary(string_to_binary_vec(
                    "0x14459a50cf6e40522f578053f15d8f4f4c91353fe7b8695f8b9e9b9d6d46c2d9",
                )),
            ),
            (
                "state_root".to_string(),
                Value::Binary(string_to_binary_vec(
                    "0x15a0edff862683f39ee941acb3d3097aaadf3f3c0a3630c73b83c9695d3ac57f",
                )),
            ),
            (
                "nonce".to_string(),
                Value::UInt64(U256::from(0x0000000000000000u64)),
            ),
            (
                "status".to_string(),
                Value::Binary(string_to_binary_vec("Finalized")),
            ),
            (
                "block_reward".to_string(),
                Value::Binary(string_to_binary_vec("0.00324994271482852")),
            ),
            (
                "fee_recipient".to_string(),
                Value::Binary(string_to_binary_vec(
                    "0x4675c7e5baafbffbca748158becba61ef3b0a263",
                )),
            ),
            (
                "total_difficulty".to_string(),
                Value::Binary(string_to_binary_vec("58,750,003,716,598,352,816,469")),
            ),
            ("size".to_string(), Value::UInt32(U256::from(4060u32))),
            (
                "gas_used".to_string(),
                Value::UInt32(U256::from(1197119u32)),
            ),
            (
                "gas_limit".to_string(),
                Value::UInt32(U256::from(30000000u32)),
            ),
            (
                "burnt_fees".to_string(),
                Value::Binary(string_to_binary_vec("0.073804376653712782")),
            ),
            (
                "pos_slot".to_string(),
                Value::UInt32(U256::from(4869328u32)),
            ),
            (
                "pos_epoch".to_string(),
                Value::UInt32(U256::from(152166u32)),
            ),
        ]),
    )
}

fn create_tx_for_block_15705350(events: Vec<Event>, calltraces: Vec<CallTrace>) -> Transaction {
    Transaction::new(
        15705350,
        8,
        1665255959,
        events,
        calltraces,
        HashMap::from([
            (
                "tx_hash".to_string(),
                Value::Binary(string_to_binary_vec(
                    "0x5cf49195244656d1c727bf0b25c0dd896f951a87213e3fd952f4c99752af1062",
                )),
            ),
            ("type".to_string(), Value::UInt8(U256::from(2u8))),
            ("nonce".to_string(), Value::UInt32(U256::from(1241u32))),
            (
                "status".to_string(),
                Value::Binary(string_to_binary_vec("Success")),
            ),
            (
                "from".to_string(),
                Value::Binary(string_to_binary_vec(
                    "0x452429cd43341359fafcf021fd85957851d7acfd",
                )),
            ),
            (
                "to".to_string(),
                Value::Binary(string_to_binary_vec(
                    "0x68b3465833fb72a70ecdf485e0e4c7bd8665fc45",
                )),
            ),
            (
                "value".to_string(),
                Value::Binary(string_to_binary_vec("0")),
            ),
            (
                "fee".to_string(),
                Value::Binary(string_to_binary_vec("61.651662578")),
            ),
            (
                "gas_price".to_string(),
                Value::Binary(string_to_binary_vec("0.000000063151662578")),
            ),
            (
                "gas_limit".to_string(),
                Value::UInt32(U256::from(241128u32)),
            ),
            ("gas_used".to_string(), Value::UInt32(U256::from(160011u32))),
            (
                "method".to_string(),
                Value::Binary(string_to_binary_vec("5ae401dc00000000000000000000000000000000000000000000000000000000")),
            ),
            (
                "args".to_string(),
                Value::Binary(string_to_binary_vec("6341d1070000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000160000000000000000000000000000000000000000000000000000000000000028000000000000000000000000000000000000000000000000000000000000000c4f3995c67000000000000000000000000a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48000000000000000000000000000000000000000000000000000000000ee6b280000000000000000000000000000000000000000000000000000000006341d5ab000000000000000000000000000000000000000000000000000000000000001c0474ed7f67c49f1a52952a78f8f0632b3e97199baaa51deb96ffc798b2e27f5a2a70841c1d616fce1dd1bea67dd1806c519fe042e797d111ceec555d09017c370000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000e404e45aaf000000000000000000000000a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc200000000000000000000000000000000000000000000000000000000000001f40000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000ee6b280000000000000000000000000000000000000000000000000029b3110036f3197000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004449404b7c000000000000000000000000000000000000000000000000029b3110036f3197000000000000000000000000452429cd43341359fafcf021fd85957851d7acfd00000000000000000000000000000000000000000000000000000000")),
            ),
        ]),
    )
}

fn create_events_for_tx_8() -> Vec<Event> {
    let event_0 = Event::new(
        15705350,
        8,
        0,
        string_to_binary_vec("0x8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925"),
        HashMap::from([
            (
                "1".to_string(),
                Value::Binary(string_to_binary_vec(
                    "0x000000000000000000000000452429cd43341359fafcf021fd85957851d7acfd",
                )),
            ),
            (
                "2".to_string(),
                Value::Binary(string_to_binary_vec(
                    "0x00000000000000000000000068b3465833fb72a70ecdf485e0e4c7bd8665fc45",
                )),
            ),
            (
                "data".to_string(),
                Value::Binary(string_to_binary_vec(
                    "0x000000000000000000000000000000000000000000000000000000000ee6b280",
                )),
            ),
        ]),
    );

    let event_1 = Event::new(
        15705350,
        8,
        1,
        string_to_binary_vec("0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"),
        HashMap::from([
            (
                "1".to_string(),
                Value::Binary(string_to_binary_vec(
                    "0x00000000000000000000000088e6a0c2ddd26feeb64f039a2c41296fcb3f5640",
                )),
            ),
            (
                "2".to_string(),
                Value::Binary(string_to_binary_vec(
                    "0x00000000000000000000000068b3465833fb72a70ecdf485e0e4c7bd8665fc45",
                )),
            ),
            (
                "data".to_string(),
                Value::Binary(string_to_binary_vec(
                    "0x000000000000000000000000000000000000000000000000029e871d72633a07",
                )),
            ),
        ]),
    );

    let event_2 = Event::new(
        15705350,
        8,
        2,
        string_to_binary_vec("0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"),
        HashMap::from([
            (
                "1".to_string(),
                Value::Binary(string_to_binary_vec(
                    "0x000000000000000000000000452429cd43341359fafcf021fd85957851d7acfd",
                )),
            ),
            (
                "2".to_string(),
                Value::Binary(string_to_binary_vec(
                    "0x00000000000000000000000088e6a0c2ddd26feeb64f039a2c41296fcb3f5640",
                )),
            ),
            (
                "data".to_string(),
                Value::Binary(string_to_binary_vec(
                    "0x000000000000000000000000000000000000000000000000000000000ee6b280",
                )),
            ),
        ]),
    );

    vec![event_0, event_1, event_2]
}

fn create_calltraces_for_tx_8(events: Vec<Event>) -> Vec<CallTrace> {
    let call_0_1_1 = CallTrace::new(
        15705350,
        8,
        0,
        vec![events[0].clone()],
        HashMap::from([
            (
                "from".to_string(),
                Value::Binary(string_to_binary_vec(
                    "0x68b3465833fb72a70ecdf485e0e4c7bd8665fc45",
                )),
            ),
            (
                "to".to_string(),
                Value::Binary(string_to_binary_vec(
                    "0x452429cd43341359fafcf021fd85957851d7acfd",
                )),
            ),
            (
                "value".to_string(),
                Value::Binary(string_to_binary_vec("0.188736794939046407")),
            ),
            ("gas_limit".to_string(), Value::UInt32(U256::from(41242u32))),
        ]),
    );
    let call_0_1_1_1 = CallTrace::new(
        15705350,
        8,
        1,
        vec![events[1].clone(), events[2].clone()],
        HashMap::from([
            (
                "from".to_string(),
                Value::Binary(string_to_binary_vec(
                    "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
                )),
            ),
            (
                "to".to_string(),
                Value::Binary(string_to_binary_vec(
                    "0x68b3465833fb72a70ecdf485e0e4c7bd8665fc45",
                )),
            ),
            (
                "value".to_string(),
                Value::Binary(string_to_binary_vec("0.188736794939046407")),
            ),
            ("gas_limit".to_string(), Value::UInt32(U256::from(2300u32))),
        ]),
    );

    vec![call_0_1_1, call_0_1_1_1]
}

fn create_false_condition_for_tx_fields() -> Expression {
    Expression::Condition(create_condition_with_comparison(
        create_reference_value_comparison(
            "$.block_index",
            Value::UInt64(U256::from(15705350u64)),
            ComparisonOperator::Equal,
        ),
        create_reference_value_comparison(
            "$.time",
            Value::UInt64(U256::from(1665255959u64)),
            ComparisonOperator::LessThan,
        ),
        ConditionOperator::And,
    ))
}

fn create_true_events_sequence_expression() -> Vec<Expression> {
    vec![
        Expression::Comparison(create_value_reference_comparison(
            "$.event_id",
            Value::Binary(string_to_binary_vec(
                "0x8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925",
            )),
            ComparisonOperator::Equal,
        )),
        Expression::Comparison(create_reference_value_comparison(
            "$.event_id",
            Value::Binary(string_to_binary_vec(
                "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
            )),
            ComparisonOperator::Equal,
        )),
        Expression::Comparison(create_value_reference_comparison(
            "$.event_id",
            Value::Binary(string_to_binary_vec(
                "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
            )),
            ComparisonOperator::Equal,
        )),
    ]
}

fn create_true_calltraces_sequence_expression() -> Vec<Expression> {
    vec![
        Expression::Comparison(create_value_reference_comparison(
            "$.block_index",
            Value::UInt128(U256::from(15705350u64)),
            ComparisonOperator::Equal,
        )),
        Expression::Comparison(create_reference_value_comparison(
            "$.tx_index",
            Value::UInt128(U256::from(8u32)),
            ComparisonOperator::Equal,
        )),
    ]
}

fn create_true_comparison_for_tx_extra() -> Comparison {
    let comparison_value = Value::Object( HashMap::from([
        (
            "tx_hash".to_string(),
            Value::Binary(string_to_binary_vec(
                "0x5cf49195244656d1c727bf0b25c0dd896f951a87213e3fd952f4c99752af1062",
            )),
        ),
        ("type".to_string(), Value::UInt8(U256::from(2u8))),
        ("nonce".to_string(), Value::UInt32(U256::from(1241u32))),
        (
            "status".to_string(),
            Value::Binary(string_to_binary_vec("Success")),
        ),
        (
            "from".to_string(),
            Value::Binary(string_to_binary_vec(
                "0x452429cd43341359fafcf021fd85957851d7acfd",
            )),
        ),
        (
            "to".to_string(),
            Value::Binary(string_to_binary_vec(
                "0x68b3465833fb72a70ecdf485e0e4c7bd8665fc45",
            )),
        ),
        (
            "value".to_string(),
            Value::Binary(string_to_binary_vec("0")),
        ),
        (
            "fee".to_string(),
            Value::Binary(string_to_binary_vec("61.651662578")),
        ),
        (
            "gas_price".to_string(),
            Value::Binary(string_to_binary_vec("0.000000063151662578")),
        ),
        (
            "gas_limit".to_string(),
            Value::UInt32(U256::from(241128u32)),
        ),
        ("gas_used".to_string(), Value::UInt32(U256::from(160011u32))),
        (
            "method".to_string(),
            Value::Binary(string_to_binary_vec("5ae401dc00000000000000000000000000000000000000000000000000000000")),
        ),
        (
            "args".to_string(),
            Value::Binary(string_to_binary_vec("6341d1070000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000160000000000000000000000000000000000000000000000000000000000000028000000000000000000000000000000000000000000000000000000000000000c4f3995c67000000000000000000000000a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48000000000000000000000000000000000000000000000000000000000ee6b280000000000000000000000000000000000000000000000000000000006341d5ab000000000000000000000000000000000000000000000000000000000000001c0474ed7f67c49f1a52952a78f8f0632b3e97199baaa51deb96ffc798b2e27f5a2a70841c1d616fce1dd1bea67dd1806c519fe042e797d111ceec555d09017c370000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000e404e45aaf000000000000000000000000a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc200000000000000000000000000000000000000000000000000000000000001f40000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000ee6b280000000000000000000000000000000000000000000000000029b3110036f3197000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004449404b7c000000000000000000000000000000000000000000000000029b3110036f3197000000000000000000000000452429cd43341359fafcf021fd85957851d7acfd00000000000000000000000000000000000000000000000000000000")),
        ),
    ]));

    create_reference_value_comparison("$.extra", comparison_value, ComparisonOperator::Equal)
}

fn create_true_condition_with_events_calltraces_extra() -> Expression {
    Expression::Condition(create_condition_with_condition(
        Condition {
            left: Box::new(Expression::EventsSequence(
                create_true_events_sequence_expression(),
            )),
            right: Box::new(Expression::CallTracesSequence(
                create_true_calltraces_sequence_expression(),
            )),
            operator: ConditionOperator::And,
        },
        create_true_comparison_for_tx_extra(),
        ConditionOperator::And,
    ))
}

#[test]
fn rule_evm_tx_matching() {
    let events = create_events_for_tx_8();
    let calltraces = create_calltraces_for_tx_8(events.clone());
    let tx = create_tx_for_block_15705350(events.clone(), calltraces);
    let block = create_block_15705350(vec![tx.clone()]);

    let rule_expression_true = Expression::Condition(Condition {
        left: Box::new(create_true_condition_with_events_calltraces_extra()),
        right: Box::new(create_false_condition_for_tx_fields()),
        operator: ConditionOperator::Or,
    });

    let rule = test_active_rule(rule_expression_true.clone());
    let rule_verification_context_true = VerificationRuleContext::new(true, rule_expression_true);

    assert_eq!(
        rule.verify(&tx, Some(block.clone())).unwrap(),
        rule_verification_context_true
    );

    let rule_expression_false = Expression::Condition(Condition {
        left: Box::new(create_true_condition_with_events_calltraces_extra()),
        right: Box::new(create_false_condition_for_tx_fields()),
        operator: ConditionOperator::And,
    });
    let rule = test_active_rule(rule_expression_false.clone());
    let rule_verification_context_false =
        VerificationRuleContext::new(false, rule_expression_false);

    assert_eq!(
        rule.verify(&tx, Some(block)).unwrap(),
        rule_verification_context_false
    );
}
