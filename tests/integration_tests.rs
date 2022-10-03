use std::collections::HashMap;

use mamoru_core::{
    blockchain_data_types::{Event, Transaction},
    rule::*,
    rules_engine::check_rule_object_match,
    value::Value,
};

const BLOCK_HASH: &str = "0xa79b2d2f1d4f0ec73a9a107525b6d08738e01f6c55b44d3fbb04246707d22309";
const TX_HASH: &str = "0x489a7cecb58805e24407722b30e9f4c23827b6d1b63254dbe91654b32c8acf64";
const BLOCK_INDEX: u64 = 12546;
const EVENT_ID: u64 = 9876;

fn create_transaction(block_hash: &str, block_index: u64, tx_hash: &str) -> Transaction {
    Transaction::new(
        String::from(block_hash),
        block_index,
        String::from(tx_hash),
        HashMap::new(),
    )
}

fn create_default_transaction() -> Transaction {
    create_transaction(BLOCK_HASH, BLOCK_INDEX, TX_HASH)
}

fn create_event(block_hash: &str, block_index: u64, tx_hash: &str, event_id: u64) -> Event {
    Event::new(
        String::from(block_hash),
        block_index,
        String::from(tx_hash),
        event_id,
        HashMap::new(),
    )
}

fn create_default_event() -> Event {
    create_event(BLOCK_HASH, BLOCK_INDEX, TX_HASH, EVENT_ID)
}

fn create_reference_value_comparison(
    reference: &str,
    value: Value,
    operator: ComparisonOperator,
) -> Comparison {
    Comparison {
        left: ComparisonValue::Reference(String::from(reference)),
        right: ComparisonValue::Value(value),
        operator,
    }
}

fn create_default_true_comparison() -> Comparison {
    create_reference_value_comparison(
        "$.transaction_hash",
        Value::String(String::from(BLOCK_HASH)),
        ComparisonOperator::NotEqual,
    )
}

fn create_default_false_comparison() -> Comparison {
    create_reference_value_comparison(
        "$.transaction_hash",
        Value::String(String::from(TX_HASH)),
        ComparisonOperator::Equal,
    )
}

fn create_simple_condition(
    left: Comparison,
    right: Comparison,
    operator: ConditionOperator,
) -> Condition {
    Condition {
        left: Box::new(Expression::Comparison(left)),
        right: Box::new(Expression::Comparison(right)),
        operator,
    }
}

fn create_complex_condition(
    left: Condition,
    right: Comparison,
    operator: ConditionOperator,
) -> Condition {
    Condition {
        left: Box::new(Expression::Condition(left)),
        right: Box::new(Expression::Comparison(right)),
        operator,
    }
}

fn create_rule(condition: Condition) -> Rule {
    Rule::new(
        Chain::AptosTestnet,
        condition,
        vec![ActionType::SendNotification {
            notification_id: String::from("Notification ID"),
        }],
    )
}

#[test]
fn rule_object_matching_happy_flow() {
    let comparison_true = create_reference_value_comparison(
        "$.block_hash",
        Value::String(String::from(BLOCK_HASH)),
        ComparisonOperator::Equal,
    );

    let condition_simple_true = create_simple_condition(
        create_default_true_comparison(),
        create_default_false_comparison(),
        ConditionOperator::Or,
    );

    let condition_complex_true = create_complex_condition(
        condition_simple_true,
        comparison_true,
        ConditionOperator::And,
    );

    let rule = create_rule(condition_complex_true);

    assert!(check_rule_object_match(&rule, &create_default_transaction()).unwrap());
    assert!(check_rule_object_match(&rule, &create_default_event()).unwrap());
}

#[test]
#[should_panic(expected = "NoPath(\"$.event_id\")")]
fn rule_transaction_matching_non_existent_reference() {
    let comparison_non_existent_tx_reference = create_reference_value_comparison(
        "$.event_id",
        Value::Number(125),
        ComparisonOperator::LessThan,
    );

    let condition = create_simple_condition(
        create_default_true_comparison(),
        comparison_non_existent_tx_reference,
        ConditionOperator::And,
    );

    let rule = create_rule(condition);

    check_rule_object_match(&rule, &create_default_transaction()).unwrap();
}

#[test]
fn rule_transaction_matching_different_value_types() {
    let comparison_different_value_types = create_reference_value_comparison(
        "$.block_index",
        Value::Bool(true),
        ComparisonOperator::LessThan,
    );

    let condition = create_simple_condition(
        create_default_true_comparison(),
        comparison_different_value_types,
        ConditionOperator::And,
    );

    let rule = create_rule(condition);

    assert!(!check_rule_object_match(&rule, &create_default_transaction()).unwrap());
}
