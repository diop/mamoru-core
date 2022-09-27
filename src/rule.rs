use crate::value::Value;

pub enum Chain {
    AptosTestnet,
    AptosMainnet,
    MistenTestnet,
    MistenMainnet,
    SolanaTestnet,
    SolanaMainnet,
    NearTestnet,
    NearMainnet,
}

pub enum ComparisonOperator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    // In,
    // NotIn,
}

pub enum ConditionOperator {
    And,
    Or,
}

pub enum ComparisonValue {
    Reference(String),
    Value(Value),
}

pub enum ActionType {
    SubmitTransaction { to: String, payload: String },
    SendNotification { notification_id: String },
}

pub enum Expression {
    Comparison(Comparison),
    Condition(Condition),
}

pub struct Comparison {
    pub left: ComparisonValue,
    pub right: ComparisonValue,
    pub operator: ComparisonOperator,
}

pub struct Condition {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub operator: ConditionOperator,
}

pub struct Rule {
    chain: Chain,
    conditions: Condition,
    actions: Vec<ActionType>,
}

impl Rule {
    pub fn new(chain: Chain, conditions: Condition, actions: Vec<ActionType>) -> Self {
        Rule {
            chain,
            conditions,
            actions,
        }
    }

    pub fn rule_chain(&self) -> &Chain {
        &self.chain
    }

    pub fn rule_conditions(&self) -> &Condition {
        &self.conditions
    }

    pub fn rule_actions(&self) -> &Vec<ActionType> {
        &self.actions
    }
}
