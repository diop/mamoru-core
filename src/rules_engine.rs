use crate::{blockchain_data_types::*, rule::*};
use jsonpath_lib::Selector;
use std::result::Result;

pub fn check_rule_object_match(
    rule: &Rule,
    object: &impl BlockchainDataType,
) -> Result<bool, String> {
    let matching_result = check_condition(object, rule.rule_conditions())?;
    Ok(matching_result)
}

pub fn check_expression(
    object: &impl BlockchainDataType,
    expression: &Expression,
) -> Result<bool, String> {
    match expression {
        Expression::Comparison(comparison) => check_comparison(object, comparison),
        Expression::Condition(condition) => check_condition(object, condition),
    }
}

fn check_condition(
    object: &impl BlockchainDataType,
    condition: &Condition,
) -> Result<bool, String> {
    let left = check_expression(object, &condition.left)?;
    let right = check_expression(object, &condition.right)?;
    let condition_result = match condition.operator {
        ConditionOperator::And => left && right,
        ConditionOperator::Or => left || right,
    };
    Ok(condition_result)
}

fn check_comparison(
    object: &impl BlockchainDataType,
    comparison: &Comparison,
) -> Result<bool, String> {
    let left = retrieve_comparison_value(&comparison.left, object)?;
    let right = retrieve_comparison_value(&comparison.right, object)?;
    let comparison_result = match comparison.operator {
        ComparisonOperator::Equal => left.eq(&right),
        ComparisonOperator::NotEqual => left.ne(&right),
        ComparisonOperator::GreaterThan => left.gt(&right),
        ComparisonOperator::GreaterThanOrEqual => left.ge(&right),
        ComparisonOperator::LessThan => left.lt(&right),
        ComparisonOperator::LessThanOrEqual => left.le(&right),
        // TODO: implement ComparisonOperator::In and ComparisonOperator::NotIn
    };
    Ok(comparison_result)
}

fn retrieve_comparison_value(
    comparison_value: &ComparisonValue,
    object: &impl BlockchainDataType,
) -> Result<String, String> {
    let comparison_value = match comparison_value {
        ComparisonValue::Reference(reference) => Selector::new()
            .path(&reference)?
            .value_from(object)?
            .select_as_str()?,
        ComparisonValue::Value(value) => value.to_string(),
    };
    Ok(comparison_value)
}
