use crate::{blockchain_data_types::*, rule::*};
use jsonpath_lib::Selector;

pub fn check_expression(object: &impl BlockchainDataType, expression: &Expression) -> bool {
    match expression {
        Expression::Comparison(comparison) => check_comparison(object, comparison),
        Expression::Condition(condition) => check_condition(object, condition),
    }
}

fn check_condition(object: &impl BlockchainDataType, condition: &Condition) -> bool {
    let left = check_expression(object, &condition.left);
    let right = check_expression(object, &condition.right);
    match condition.operator {
        ConditionOperator::And => return left && right,
        _ => return left || right,
    }
}

fn check_comparison(object: &impl BlockchainDataType, comparison: &Comparison) -> bool {
    let left = retrieve_comparison_value(&comparison.left, object);
    let right = retrieve_comparison_value(&comparison.right, object);
    match comparison.operator {
        ComparisonOperator::Equal => return left.eq(&right),
        ComparisonOperator::NotEqual => return left.ne(&right),
        ComparisonOperator::GreaterThan => return left.gt(&right),
        ComparisonOperator::GreaterThanOrEqual => return left.ge(&right),
        ComparisonOperator::LessThan => return left.lt(&right),
        ComparisonOperator::LessThanOrEqual => return left.le(&right),
        // TODO: implement ComparisonOperator::In and ComparisonOperator::NotIn
    }
}

fn retrieve_comparison_value(
    comparison_value: &ComparisonValue,
    object: &impl BlockchainDataType,
) -> String {
    match comparison_value {
        ComparisonValue::Reference(reference) => Selector::new()
            .path(&reference)
            .unwrap()
            .value_from(object)
            .unwrap()
            .select_as_str()
            .unwrap(),
        ComparisonValue::Value(value) => value.to_string(),
    }
}

// TODO: Add a loop over rules to check if any of the rules is matched