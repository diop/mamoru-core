pub use ethnum::U256;
pub use mamoru_core::{
    blockchain_data_types::{Block, CallTrace, Event, Transaction},
    rule::*,
    value::Value,
};
pub use std::collections::HashMap;

pub fn create_reference_value_comparison(
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

pub fn create_value_reference_comparison(
    reference: &str,
    value: Value,
    operator: ComparisonOperator,
) -> Comparison {
    Comparison {
        left: ComparisonValue::Value(value),
        right: ComparisonValue::Reference(String::from(reference)),
        operator,
    }
}

pub fn create_condition_with_comparison(
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

pub fn create_condition_with_condition(
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

pub fn string_to_binary_vec(string: &str) -> Vec<u8> {
    String::from(string).into_bytes()
}

pub fn test_active_rule(expression: Expression) -> Rule {
    Rule::new("testing".to_string(), 0, u64::MAX, expression)
}
