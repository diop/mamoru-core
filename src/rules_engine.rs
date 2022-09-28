use crate::{blockchain_data_types::BlockchainDataType, errors::RetrieveValueError, rule, value};
use serde_json;
use std::result::Result;

pub fn check_rule_object_match(
    rule: &rule::Rule,
    object: &impl BlockchainDataType,
) -> Result<bool, RetrieveValueError> {
    let matching_result = check_condition(object, rule.rule_conditions())?;
    Ok(matching_result)
}

fn check_expression(
    object: &impl BlockchainDataType,
    expression: &rule::Expression,
) -> Result<bool, RetrieveValueError> {
    match expression {
        rule::Expression::Comparison(comparison) => check_comparison(object, comparison),
        rule::Expression::Condition(condition) => check_condition(object, condition),
    }
}

fn check_condition(
    object: &impl BlockchainDataType,
    condition: &rule::Condition,
) -> Result<bool, RetrieveValueError> {
    let left = check_expression(object, &condition.left)?;
    let right = check_expression(object, &condition.right)?;
    let condition_result = match condition.operator {
        rule::ConditionOperator::And => left && right,
        rule::ConditionOperator::Or => left || right,
    };
    Ok(condition_result)
}

fn check_comparison(
    object: &impl BlockchainDataType,
    comparison: &rule::Comparison,
) -> Result<bool, RetrieveValueError> {
    let left = retrieve_comparison_value(&comparison.left, object)?;
    let right = retrieve_comparison_value(&comparison.right, object)?;
    let comparison_result = match comparison.operator {
        rule::ComparisonOperator::Equal => left.eq(&right),
        rule::ComparisonOperator::NotEqual => left.ne(&right),
        rule::ComparisonOperator::GreaterThan => left.gt(&right),
        rule::ComparisonOperator::GreaterThanOrEqual => left.ge(&right),
        rule::ComparisonOperator::LessThan => left.lt(&right),
        rule::ComparisonOperator::LessThanOrEqual => left.le(&right),
        rule::ComparisonOperator::In => left.is_in(&right),
        rule::ComparisonOperator::NotIn => left.not_in(&right),
    };
    Ok(comparison_result)
}

fn retrieve_comparison_value(
    comparison_value: &rule::ComparisonValue,
    object: &impl BlockchainDataType,
) -> Result<value::Value, RetrieveValueError> {
    let comparison_value = match comparison_value {
        rule::ComparisonValue::Reference(reference) => {
            let v: Vec<serde_json::Value> =
                jsonpath_lib::select_as(&serde_json::to_string(object)?, reference)?;
            match v.get(0) {
                Some(reference_value) => value::Value::from(reference_value.clone()),
                None => return Err(RetrieveValueError::NoPath(reference.to_string())),
            }
        }
        rule::ComparisonValue::Value(value) => value.clone(),
    };
    Ok(comparison_value)
}
