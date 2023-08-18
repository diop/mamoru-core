use std::collections::BTreeMap;
use std::str::FromStr;

use serde::Serialize;

use crate::error::{ConditionEvalError, ConditionParseError};
use crate::StepOutputs;

mod parser;

/// The parsed playbook condition.
#[derive(Debug, Default, PartialEq, Clone)]
pub enum Condition {
    #[default]
    Success,
    Failure,
    Always,
    Skipped,
    Literal(String),
    Field(String),
    Binary {
        left: Box<Condition>,
        right: Box<Condition>,
        op: BinaryOp,
    },
    Unary {
        condition: Box<Condition>,
        op: UnaryOp,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    And,
    Or,
    Eq,
    Ne,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOp {
    Not,
}

impl FromStr for Condition {
    type Err = ConditionParseError;

    fn from_str(condition: &str) -> Result<Self, Self::Err> {
        parser::parse_condition(condition)
    }
}

impl Condition {
    pub fn evaluate(&self, ctx: &Context) -> Result<bool, ConditionEvalError> {
        match self {
            Condition::Success => Ok(matches!(ctx.pipeline_status, PipelineStatus::Success)),
            Condition::Failure => Ok(matches!(ctx.pipeline_status, PipelineStatus::Failed)),
            Condition::Always => Ok(true),
            Condition::Skipped => Ok(matches!(ctx.pipeline_status, PipelineStatus::Skipped)),
            Condition::Literal(s) => s
                .parse::<bool>()
                .map_err(|_| ConditionEvalError::LiteralEval { literal: s.clone() }),
            Condition::Field(field) => Err(ConditionEvalError::FieldEval {
                field: field.clone(),
            }),
            Condition::Binary { left, right, op } => match op {
                BinaryOp::And => Ok(left.evaluate(ctx)? && right.evaluate(ctx)?),
                BinaryOp::Or => Ok(left.evaluate(ctx)? || right.evaluate(ctx)?),
                BinaryOp::Eq => evaluate_binary_eq(left, right, ctx),
                BinaryOp::Ne => evaluate_binary_eq(left, right, ctx).map(|result| !result),
            },
            Condition::Unary { condition, op } => match op {
                UnaryOp::Not => {
                    let result = condition.evaluate(ctx)?;
                    Ok(!result)
                }
            },
        }
    }
}

fn evaluate_binary_eq(
    left: &Condition,
    right: &Condition,
    ctx: &Context,
) -> Result<bool, ConditionEvalError> {
    let result = match (left, right) {
        (Condition::Field(field), Condition::Literal(literal))
        | (Condition::Literal(literal), Condition::Field(field)) => {
            let field_value = ctx.get_field(field)?;

            field_value == literal
        }
        (Condition::Literal(left), Condition::Literal(right)) => left == right,
        (Condition::Field(left), Condition::Field(right)) => {
            let left_value = ctx.get_field(left)?;
            let right_value = ctx.get_field(right)?;

            left_value == right_value
        }
        (left, right) => left.evaluate(ctx)? == right.evaluate(ctx)?,
    };

    Ok(result)
}

#[derive(Debug, Serialize)]
pub enum PipelineStatus {
    Skipped,
    Success,
    Failed,
}

#[derive(Debug, Serialize)]
pub struct Context {
    pub pipeline_status: PipelineStatus,
    pub incident: Incident,
    pub outputs: BTreeMap<String, StepOutputs>,
}

impl Context {
    pub fn get_field(&self, field: &str) -> Result<&str, ConditionEvalError> {
        match field {
            "incident.id" => Ok(&self.incident.id),
            "incident.level" => Ok(&self.incident.level),
            _ if field.starts_with("outputs.") => self.get_output_field(&field["outputs.".len()..]),
            _ => Err(ConditionEvalError::NoField {
                field: field.to_string(),
            }),
        }
    }

    fn get_output_field(&self, field: &str) -> Result<&str, ConditionEvalError> {
        let mut parts = field.splitn(2, '.');

        let task_name = parts.next().ok_or_else(|| ConditionEvalError::NoField {
            field: field.to_string(),
        })?;
        let field_name = parts.next().ok_or_else(|| ConditionEvalError::NoField {
            field: field.to_string(),
        })?;

        let task_output =
            self.outputs
                .get(task_name)
                .ok_or_else(|| ConditionEvalError::NoTask {
                    task: task_name.to_string(),
                })?;

        let field_value =
            task_output
                .get(field_name)
                .ok_or_else(|| ConditionEvalError::NoField {
                    field: field.to_string(),
                })?;

        Ok(field_value)
    }
}

#[derive(Debug, Serialize)]
pub struct Incident {
    pub id: String,
    pub level: String,
}

#[cfg(test)]
mod evaluate_tests {
    use super::*;

    fn test_context() -> Context {
        Context {
            pipeline_status: PipelineStatus::Success,
            incident: Incident {
                id: "123".to_string(),
                level: "critical".to_string(),
            },
            outputs: BTreeMap::new(),
        }
    }

    #[test]
    fn literals() {
        let condition = Condition::from_str("'true'").unwrap();
        assert!(condition.evaluate(&test_context()).unwrap());

        let condition = Condition::from_str("'false'").unwrap();
        assert!(!condition.evaluate(&test_context()).unwrap());

        let condition = Condition::from_str("'hello'").unwrap();
        assert!(condition.evaluate(&test_context()).is_err());
    }

    #[test]
    fn fields() {
        let condition = Condition::from_str("incident.id == '123'").unwrap();
        assert!(condition.evaluate(&test_context()).unwrap());

        let condition = Condition::from_str("incident.id == '456'").unwrap();
        assert!(!condition.evaluate(&test_context()).unwrap());

        let condition = Condition::from_str("incident.id == incident.id").unwrap();
        assert!(condition.evaluate(&test_context()).unwrap());

        let condition = Condition::from_str("incident.id").unwrap();
        assert!(condition.evaluate(&test_context()).is_err());

        let condition = Condition::from_str("does_not_exist == 'critical'").unwrap();
        assert!(condition.evaluate(&test_context()).is_err());
    }

    #[test]
    fn states() {
        let condition = Condition::from_str("success()").unwrap();
        assert!(condition.evaluate(&test_context()).unwrap());

        let condition = Condition::from_str("!failure()").unwrap();
        assert!(condition.evaluate(&test_context()).unwrap());

        let condition = Condition::from_str("skipped()").unwrap();
        assert!(!condition.evaluate(&test_context()).unwrap());

        let condition = Condition::from_str("always()").unwrap();
        assert!(condition.evaluate(&test_context()).unwrap());

        let condition = Condition::from_str("failure() || always()").unwrap();
        assert!(condition.evaluate(&test_context()).unwrap());

        let condition = Condition::from_str("(failure() && always()) || success()").unwrap();
        assert!(condition.evaluate(&test_context()).unwrap());

        let condition = Condition::from_str("success() && incident.level == 'critical'").unwrap();
        assert!(condition.evaluate(&test_context()).unwrap());
    }
}
