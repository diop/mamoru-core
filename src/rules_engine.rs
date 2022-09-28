use crate::{blockchain_data_types::*, rule::*, value};
use anyhow::{self, Error};
use serde_json;
use std::result::Result;

pub fn check_rule_object_match(
    rule: &Rule,
    object: &impl BlockchainDataType,
) -> Result<bool, Error> {
    let matching_result = check_condition(object, rule.rule_conditions())?;
    Ok(matching_result)
}

fn check_expression(
    object: &impl BlockchainDataType,
    expression: &Expression,
) -> Result<bool, Error> {
    match expression {
        Expression::Comparison(comparison) => check_comparison(object, comparison),
        Expression::Condition(condition) => check_condition(object, condition),
    }
}

fn check_condition(object: &impl BlockchainDataType, condition: &Condition) -> Result<bool, Error> {
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
) -> Result<bool, Error> {
    let left = retrieve_comparison_value(&comparison.left, object)?;
    let right = retrieve_comparison_value(&comparison.right, object)?;
    let comparison_result = match comparison.operator {
        ComparisonOperator::Equal => left.eq(&right),
        ComparisonOperator::NotEqual => left.ne(&right),
        ComparisonOperator::GreaterThan => left.gt(&right),
        ComparisonOperator::GreaterThanOrEqual => left.ge(&right),
        ComparisonOperator::LessThan => left.lt(&right),
        ComparisonOperator::LessThanOrEqual => left.le(&right),
        ComparisonOperator::In => left.is_in(&right),
        ComparisonOperator::NotIn => left.not_in(&right),
    };
    Ok(comparison_result)
}

fn retrieve_comparison_value(
    comparison_value: &ComparisonValue,
    object: &impl BlockchainDataType,
) -> Result<value::Value, Error> {
    let comparison_value = match comparison_value {
        ComparisonValue::Reference(reference) => {
            let v: Vec<serde_json::Value> =
                jsonpath_lib::select_as(&serde_json::to_string(object)?, reference)?;
            match v.get(0) {
                Some(reference_value) => value::Value::from(reference_value.clone()),
                None => return Err(anyhow::anyhow!("There is no such path as {}", reference)),
            }
        }
        ComparisonValue::Value(value) => value.clone(),
    };
    Ok(comparison_value)
}
