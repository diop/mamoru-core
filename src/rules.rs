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
    In,
    NotIn,
}

pub enum ConditionOperator {
    And,
    Or,
}

pub enum ComparisonValue {
    Reference(String),
    Value(String),
}

pub enum ActionType {
    SubmitTransaction { to: String, payload: String },
    SendNotification { notification_id: String },
}

pub enum ExpressionType {
    Comparison(Comparison),
    Condition(Condition),
}

pub struct Comparison {
    left: ComparisonValue,
    right: ComparisonValue,
    operator: ComparisonOperator,
}

pub struct Condition {
    left: Box<ExpressionType>,
    right: Box<ExpressionType>,
    operator: ConditionOperator,
}

pub struct Rule {
    chain: Chain,
    conditions: Condition,
    actions: Vec<ActionType>,
}