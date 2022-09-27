use std::collections::HashMap;

use mamoru_core::{
    blockchain_data_types::{Event, Transaction},
    rule::*,
    rules_engine::check_rule_object_match,
    value::Value,
};

#[test]
fn rule_object_matching_happy_flow() {
    //true
    let comparison1 = Comparison {
        left: ComparisonValue::Value(Value::String(String::from(
            "0xf499bd6a4171ba03afca2cf56518e5aa557db9866d002109b3adb31bf184646a",
        ))),
        right: ComparisonValue::Reference(String::from("$.transaction_hash")),
        operator: ComparisonOperator::NotEqual,
    };

    //true
    let comparison2 = Comparison {
        left: ComparisonValue::Reference(String::from("$.block_hash")),
        right: ComparisonValue::Value(Value::String(String::from(
            "0xa79b2d2f1d4f0ec73a9a107525b6d08738e01f6c55b44d3fbb04246707d22309",
        ))),
        operator: ComparisonOperator::Equal,
    };

    //false
    let comparison3 = Comparison {
        left: ComparisonValue::Value(Value::String(String::from(
            "0x2de857b3eb22ad4f758e6639c7ba4a5f744db465902e25ba9d66da5c7f5efaeb",
        ))),
        right: ComparisonValue::Reference(String::from("$.transaction_hash")),
        operator: ComparisonOperator::Equal,
    };

    //true
    let comparison4 = Comparison {
        left: ComparisonValue::Reference(String::from("$.block_index")),
        right: ComparisonValue::Value(Value::Number(125)),
        operator: ComparisonOperator::GreaterThan,
    };

    let condition1 = Condition {
        left: Box::new(Expression::Comparison(comparison1)),
        right: Box::new(Expression::Comparison(comparison2)),
        operator: ConditionOperator::And,
    };
    let condition2 = Condition {
        left: Box::new(Expression::Comparison(comparison3)),
        right: Box::new(Expression::Comparison(comparison4)),
        operator: ConditionOperator::Or,
    };
    let condition3 = Condition {
        left: Box::new(Expression::Condition(condition1)),
        right: Box::new(Expression::Condition(condition2)),
        operator: ConditionOperator::And,
    };

    let rule = Rule::new(
        Chain::AptosTestnet,
        condition3,
        vec![ActionType::SendNotification {
            notification_id: String::from("Notification ID"),
        }],
    );

    let transaction = Transaction::new(
        String::from("0xa79b2d2f1d4f0ec73a9a107525b6d08738e01f6c55b44d3fbb04246707d22309"),
        12546,
        String::from("0x123..."),
        HashMap::new(),
    );

    let event = Event::new(
        String::from("0xa79b2d2f1d4f0ec73a9a107525b6d08738e01f6c55b44d3fbb04246707d22309"),
        12546,
        String::from("0x123..."),
        9876,
        HashMap::new(),
    );

    assert_eq!(check_rule_object_match(&rule, &transaction).unwrap(), true);
    assert_eq!(check_rule_object_match(&rule, &event).unwrap(), true);
}

#[test]
#[should_panic(expected = "There is no such path as $.event_id")]
fn rule_transaction_matching_non_existent_reference() {
    //true
    let comparison1 = Comparison {
        left: ComparisonValue::Value(Value::String(String::from(
            "0xf499bd6a4171ba03afca2cf56518e5aa557db9866d002109b3adb31bf184646a",
        ))),
        right: ComparisonValue::Reference(String::from("$.transaction_hash")),
        operator: ComparisonOperator::NotEqual,
    };

    //Error
    let comparison2 = Comparison {
        left: ComparisonValue::Reference(String::from("$.event_id")),
        right: ComparisonValue::Value(Value::Number(125)),
        operator: ComparisonOperator::LessThan,
    };

    let condition = Condition {
        left: Box::new(Expression::Comparison(comparison1)),
        right: Box::new(Expression::Comparison(comparison2)),
        operator: ConditionOperator::And,
    };

    let rule = Rule::new(
        Chain::AptosTestnet,
        condition,
        vec![ActionType::SendNotification {
            notification_id: String::from("Notification ID"),
        }],
    );

    let transaction = Transaction::new(
        String::from("0xa79b2d2f1d4f0ec73a9a107525b6d08738e01f6c55b44d3fbb04246707d22309"),
        12546,
        String::from("0x123..."),
        HashMap::new(),
    );

    check_rule_object_match(&rule, &transaction).unwrap();
}

#[test]
fn rule_transaction_matching_different_value_types() {
    //true
    let comparison1 = Comparison {
        left: ComparisonValue::Value(Value::String(String::from(
            "0xa79b2d2f1d4f0ec73a9a107525b6d08738e01f6c55b44d3fbb04246707d22309",
        ))),
        right: ComparisonValue::Reference(String::from("$.block_hash")),
        operator: ComparisonOperator::Equal,
    };

    //false
    let comparison2 = Comparison {
        left: ComparisonValue::Reference(String::from("$.block_index")),
        right: ComparisonValue::Value(Value::Bool(true)),
        operator: ComparisonOperator::LessThan,
    };

    let condition = Condition {
        left: Box::new(Expression::Comparison(comparison1)),
        right: Box::new(Expression::Comparison(comparison2)),
        operator: ConditionOperator::And,
    };

    let rule = Rule::new(
        Chain::AptosTestnet,
        condition,
        vec![ActionType::SendNotification {
            notification_id: String::from("Notification ID"),
        }],
    );

    let transaction = Transaction::new(
        String::from("0xa79b2d2f1d4f0ec73a9a107525b6d08738e01f6c55b44d3fbb04246707d22309"),
        12546,
        String::from("0x123..."),
        HashMap::new(),
    );

    assert_eq!(check_rule_object_match(&rule, &transaction).unwrap(), false);
}
