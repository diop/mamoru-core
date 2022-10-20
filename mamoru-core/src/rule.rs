use crate::{
    blockchain_data_types::{Block, Transaction},
    errors::RetrieveValueError,
    rules_engine,
    value::Value,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ComparisonOperator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    In,
    NotIn,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConditionOperator {
    And,
    Or,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ComparisonValue {
    Reference(String),
    Value(Value),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    Comparison(Comparison),
    Condition(Condition),
    EventsSequence(Vec<Expression>),
    CallTracesSequence(Vec<Expression>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Comparison {
    pub left: ComparisonValue,
    pub right: ComparisonValue,
    pub operator: ComparisonOperator,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Condition {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub operator: ConditionOperator,
}

#[derive(Debug, Eq, PartialEq)]
pub struct VerificationRuleContext {
    matched: bool,
    evaluated_expression: Expression,
}

impl VerificationRuleContext {
    pub fn new(matched: bool, evaluated_expression: Expression) -> VerificationRuleContext {
        VerificationRuleContext {
            matched,
            evaluated_expression,
        }
    }

    pub fn matched(&self) -> bool {
        self.matched
    }

    pub fn evaluated_expression(&self) -> &Expression {
        &self.evaluated_expression
    }
}

pub struct Rule {
    expression: Expression,
}

impl Rule {
    pub fn new(expression: Expression) -> Self {
        Rule { expression }
    }

    pub fn rule_expression(&self) -> &Expression {
        &self.expression
    }

    pub fn verify(
        &self,
        transaction: &Transaction,
        _block: Option<Block>,
    ) -> Result<VerificationRuleContext, RetrieveValueError> {
        let matched = rules_engine::check_expression(transaction, self.rule_expression())?;
        Ok(VerificationRuleContext {
            matched,
            evaluated_expression: self.rule_expression().clone(),
        })
    }
}
