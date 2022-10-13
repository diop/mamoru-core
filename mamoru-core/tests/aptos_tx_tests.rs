mod common;
use common::*;

fn create_block_107809124(transactions: Vec<Transaction>) -> Block {
    Block::new(
        107809124,
        1665270201388757,
        transactions,
        HashMap::from([
            (
                "hash".to_string(),
                Value::Binary(string_to_binary_vec(
                    "0xdf543dac015ec95b9746cbebbe30747d5770dab25d70e867fd9944f85aaba697",
                )),
            ),
            (
                "status".to_string(),
                Value::Binary(string_to_binary_vec("Success")),
            ),
            ("gas_used".to_string(), Value::UInt64(U256::from(0u64))),
            (
                "vm_status".to_string(),
                Value::Binary(string_to_binary_vec("Executed successfully")),
            ),
            (
                "accumulator_root_hash".to_string(),
                Value::Binary(string_to_binary_vec(
                    "0xa286e3ed6241b262d96e19678bcb7e9c317eaa0a398ab68626e09d4e0b093c2c",
                )),
            ),
        ]),
    )
}

fn create_tx_for_block_107809124(events: Vec<Event>, calltraces: Vec<CallTrace>) -> Transaction {
    Transaction::new(
        107809124,
        519,
        1665270245816325,
        events,
        calltraces,
        HashMap::from([
            (
                "tx_hash".to_string(),
                Value::Binary(string_to_binary_vec(
                    "0xb8acfa10044da2ebca673abb7b9e30c14460d62304937746044de1c18b8569c2",
                )),
            ),
            (
                "sender".to_string(),
                Value::Binary(string_to_binary_vec(
                    "0x28579897603d801047315d16f75b646ca97c57007968944b4df94b1f9a4dd1f3",
                )),
            ),
            (
                "expiration_timestamp".to_string(),
                Value::UInt32(U256::from(1665270485u32)),
            ),
            (
                "version".to_string(),
                Value::UInt32(U256::from(107853285u32)),
            ),
            (
                "status".to_string(),
                Value::Binary(string_to_binary_vec("Success")),
            ),
            (
                "state_change_hash".to_string(),
                Value::Binary(string_to_binary_vec(
                    "0x371eaf09bd88c929883f15b68143f436ef644eaa67d4b7c87540339a00345949",
                )),
            ),
            (
                "event_root_hash".to_string(),
                Value::Binary(string_to_binary_vec(
                    "0x543aef57846cd4954e3b81f4dc743e855d3b3b238b5426a48661987d16ed1bc4",
                )),
            ),
            (
                "gas_limit".to_string(),
                Value::UInt32(U256::from(100000u32)),
            ),
            (
                "gas_unit_price".to_string(),
                Value::Binary(string_to_binary_vec("0.000001")),
            ),
            ("gas_used".to_string(), Value::UInt32(U256::from(1389u32))),
            (
                "gas_fee".to_string(),
                Value::Binary(string_to_binary_vec("0.001389")),
            ),
            (
                "accumulator_root_hash".to_string(),
                Value::Binary(string_to_binary_vec(
                    "0x76c62844d64cffa9852e6273e3b99b0c0699e51660cc94d6f1fe93661c8cb3c6",
                )),
            ),
            (
                "type".to_string(),
                Value::Binary(string_to_binary_vec("user_transaction")),
            ),
            (
                "function".to_string(),
                Value::Binary(string_to_binary_vec("0x1::aptos_account::create_account")),
            ),
            ("type_arguments".to_string(), Value::Array(Vec::new())),
            (
                "arguments".to_string(),
                Value::Array(vec![Value::Binary(string_to_binary_vec(
                    "0x75b0571ce9b1f9bbb059ec8ac5ef2b0c82904a873bb3241f690b958b57190fb3",
                ))]),
            ),
        ]),
    )
}

fn create_events_for_tx_519() -> Vec<Event> {
    let event_0 = Event::new(
        107809124,
        519,
        0,
        string_to_binary_vec("0x1::account::CoinRegisterEvent"),
        HashMap::from([(
            "data".to_string(),
            Value::Object(HashMap::from([(
                "type_info".to_string(),
                Value::Object(HashMap::from([
                    (
                        "account_address".to_string(),
                        Value::Binary(string_to_binary_vec("0x1")),
                    ),
                    (
                        "module_name".to_string(),
                        Value::Binary(string_to_binary_vec("0x6170746f735f636f696e")),
                    ),
                    (
                        "struct_name".to_string(),
                        Value::Binary(string_to_binary_vec("0x4170746f73436f696e")),
                    ),
                ])),
            )])),
        )]),
    );

    vec![event_0]
}

fn create_calltraces_for_tx_8(events: Vec<Event>) -> Vec<CallTrace> {
    let calltrace = CallTrace::new(
        107809124,
        519,
        0,
        vec![events[0].clone()],
        HashMap::from([
            ("depth".to_string(), Value::UInt8(U256::from(1u8))),
            (
                "module_address".to_string(),
                Value::Binary(string_to_binary_vec("0x1::aptos_account")),
            ),
            (
                "method".to_string(),
                Value::Binary(string_to_binary_vec("create_account")),
            ),
            ("type_arguments".to_string(), Value::Array(Vec::new())),
            (
                "arguments".to_string(),
                Value::Array(vec![Value::Binary(string_to_binary_vec(
                    "0x75b0571ce9b1f9bbb059ec8ac5ef2b0c82904a873bb3241f690b958b57190fb3",
                ))]),
            ),
            ("gas_used".to_string(), Value::UInt64(U256::from(1389u64))),
        ]),
    );
    vec![calltrace]
}

fn create_true_condition_for_tx_fields() -> Expression {
    Expression::Condition(create_condition_with_comparison(
        create_reference_value_comparison(
            "$.block_index",
            Value::UInt64(U256::from(107809124u64)),
            ComparisonOperator::Equal,
        ),
        create_reference_value_comparison(
            "$.time",
            Value::UInt64(U256::from(1665255959u64)),
            ComparisonOperator::GreaterThan,
        ),
        ConditionOperator::And,
    ))
}

fn create_false_events_sequence_expression() -> Vec<Expression> {
    Vec::new()
}

fn create_true_calltraces_sequence_expression() -> Vec<Expression> {
    vec![Expression::Comparison(create_value_reference_comparison(
        "$.block_index",
        Value::UInt128(U256::from(15705350u64)),
        ComparisonOperator::NotEqual,
    ))]
}

fn create_false_comparison_for_tx_extra() -> Comparison {
    let comparison_value = Value::Object(HashMap::from([
        (
            "tx_hash".to_string(),
            Value::Binary(string_to_binary_vec(
                "0xb8acfa10044da2ebca673abb7b9e30c14460d62304937746044de1c18b8569c2",
            )),
        ),
        (
            "sender".to_string(),
            Value::Binary(string_to_binary_vec(
                "0x28579897603d801047315d16f75b646ca97c57007968944b4df94b1f9a4dd1f3",
            )),
        ),
        (
            "expiration_timestamp".to_string(),
            Value::UInt32(U256::from(1665270485u32)),
        ),
        (
            "version".to_string(),
            Value::UInt32(U256::from(107853285u32)),
        ),
        (
            "status".to_string(),
            Value::Binary(string_to_binary_vec("Success")),
        ),
        (
            "state_change_hash".to_string(),
            Value::Binary(string_to_binary_vec(
                "0x371eaf09bd88c929883f15b68143f436ef644eaa67d4b7c87540339a00345949",
            )),
        ),
        (
            "event_root_hash".to_string(),
            Value::Binary(string_to_binary_vec(
                "0x543aef57846cd4954e3b81f4dc743e855d3b3b238b5426a48661987d16ed1bc4",
            )),
        ),
        (
            "gas_limit".to_string(),
            Value::UInt32(U256::from(100000u32)),
        ),
    ]));

    create_reference_value_comparison("$.extra", comparison_value, ComparisonOperator::Equal)
}

fn create_false_condition_with_events_calltraces_extra() -> Expression {
    Expression::Condition(create_condition_with_condition(
        Condition {
            left: Box::new(Expression::EventsSequence(
                create_false_events_sequence_expression(),
            )),
            right: Box::new(Expression::CallTracesSequence(
                create_true_calltraces_sequence_expression(),
            )),
            operator: ConditionOperator::Or,
        },
        create_false_comparison_for_tx_extra(),
        ConditionOperator::And,
    ))
}

#[test]
fn rule_aptos_tx_matching() {
    let events = create_events_for_tx_519();
    let calltraces = create_calltraces_for_tx_8(events.clone());
    let tx = create_tx_for_block_107809124(events.clone(), calltraces);
    let block = create_block_107809124(vec![tx.clone()]);

    let rule_expression_true = Expression::Condition(Condition {
        left: Box::new(create_false_condition_with_events_calltraces_extra()),
        right: Box::new(create_true_condition_for_tx_fields()),
        operator: ConditionOperator::Or,
    });

    let rule = Rule::new(rule_expression_true.clone());
    let rule_verification_context_true = VerificationRuleContext::new(true, rule_expression_true);

    assert_eq!(
        rule.verify(&tx, Some(block.clone())).unwrap(),
        rule_verification_context_true
    );

    let rule_expression_false = Expression::Condition(Condition {
        left: Box::new(create_false_condition_with_events_calltraces_extra()),
        right: Box::new(create_true_condition_for_tx_fields()),
        operator: ConditionOperator::And,
    });
    let rule = Rule::new(rule_expression_false.clone());
    let rule_verification_context_false =
        VerificationRuleContext::new(false, rule_expression_false);

    assert_eq!(
        rule.verify(&tx, Some(block)).unwrap(),
        rule_verification_context_false
    );
}
