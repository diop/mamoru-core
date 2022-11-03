use crate::{
    blockchain_data_types::{Block, Transaction},
    errors::RetrieveValueError,
    rules_engine,
    value::Value,
};
use ethnum::U256;

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

#[derive(Clone, Debug)]
pub struct Rule {
    id: String,
    activate_since: Value,
    inactivate_since: Value,
    expression: Expression,
}

impl Rule {
    pub fn new(
        id: String,
        activate_since: u64,
        inactivate_since: u64,
        expression: Expression,
    ) -> Self {
        let activate_since = Value::UInt64(U256::from(activate_since));
        let inactivate_since = Value::UInt64(U256::from(inactivate_since));

        Rule {
            id,
            activate_since,
            inactivate_since,
            expression,
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn rule_expression(&self) -> &Expression {
        &self.expression
    }

    pub fn verify(
        &self,
        transaction: &Transaction,
        _block: Option<Block>,
    ) -> Result<VerificationRuleContext, RetrieveValueError> {
        let matched = self.active(transaction.time());
        let matched =
            matched && rules_engine::check_expression(transaction, self.rule_expression())?;

        Ok(VerificationRuleContext {
            matched,
            evaluated_expression: self.rule_expression().clone(),
        })
    }

    fn active(&self, time: &Value) -> bool {
        time >= &self.activate_since && time < &self.inactivate_since
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn not_yet_active_rule_does_not_match() {
        let rule = rule();
        let transaction = transaction(ACTIVE_SINCE - 1);
        let result = rule.verify(&transaction, None).unwrap();

        assert!(!result.matched);
    }

    #[test]
    fn already_inactive_rule_does_not_match() {
        let rule = rule();
        let transaction = transaction(INACTIVE_SINCE);
        let result = rule.verify(&transaction, None).unwrap();

        assert!(!result.matched);
    }

    #[test]
    fn active_rule_does_match() {
        let rule = rule();
        let transaction = transaction(ACTIVE_SINCE);
        let result = rule.verify(&transaction, None).unwrap();

        assert!(result.matched);
    }

    const ACTIVE_SINCE: u64 = 10;
    const INACTIVE_SINCE: u64 = ACTIVE_SINCE + 10;

    fn transaction(time: u64) -> Transaction {
        Transaction::new(42, 43, time, vec![], vec![], HashMap::new())
    }

    fn rule() -> Rule {
        Rule::new(
            "test".to_string(),
            ACTIVE_SINCE,
            INACTIVE_SINCE,
            Expression::Comparison(Comparison {
                left: ComparisonValue::Reference("$.block_index".to_string()),
                right: ComparisonValue::Value(Value::UInt128(U256::from(42u64))),
                operator: ComparisonOperator::Equal,
            }),
        )
    }

}
