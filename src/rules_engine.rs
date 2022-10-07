use crate::{
    blockchain_data_types::BlockchainDataType,
    errors::{RetrieveValueError},
    rule::{
        Comparison, ComparisonOperator, ComparisonValue, Condition, ConditionOperator, Expression,
    },
    value,
};
use serde_json;
use std::result::Result;

fn check_vec_expression(
    objects: &Vec<impl BlockchainDataType>,
    expressions: &Vec<Expression>,
) -> Result<bool, RetrieveValueError> {
    if objects.len().ne(&expressions.len()) {
        return Ok(false);
    };
    for (event, expression) in objects.iter().zip(expressions.iter()) {
        if !(check_expression(event, expression)?) {
            return Ok(false);
        };
    }
    Ok(true)
}

pub(crate) fn check_expression(
    object: &impl BlockchainDataType,
    expression: &Expression,
) -> Result<bool, RetrieveValueError> {
    match expression {
        Expression::Comparison(comparison) => check_comparison(object, comparison),
        Expression::Condition(condition) => check_condition(object, condition),
        Expression::EventsSequence(expressions) => {
            check_vec_expression(object.events()?, expressions)
        }
        Expression::CallTracesSequence(expressions) => {
            check_vec_expression(object.calltraces()?, expressions)
        }
    }
}

fn check_condition(
    object: &impl BlockchainDataType,
    condition: &Condition,
) -> Result<bool, RetrieveValueError> {
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
) -> Result<bool, RetrieveValueError> {
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
) -> Result<value::Value, RetrieveValueError> {
    let comparison_value = match comparison_value {
        ComparisonValue::Reference(reference) => {
            let vec: Vec<serde_json::Value> =
                jsonpath_lib::select_as(&serde_json::to_string(object)?, reference)?;
            match vec.get(0) {
                Some(reference_value) => value::Value::try_from(reference_value.clone())?,
                None => return Err(RetrieveValueError::NoPath(reference.to_string())),
            }
        }
        ComparisonValue::Value(value) => value.clone(),
    };
    Ok(comparison_value)
}
