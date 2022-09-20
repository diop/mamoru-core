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

impl Rule {
    pub fn new_rule(chain: Chain, conditions: Condition, actions: Vec<ActionType>) -> Self {
        Rule { chain, conditions, actions }
    }

    pub fn get_rule_chain(&self) -> &Chain {
        &self.chain
    }

    pub fn get_rule_conditions(&self) -> &Condition {
        &self.conditions
    }

    pub fn get_rule_actions(&self) -> &Vec<ActionType> {
        &self.actions
    }
}

pub type RulesList = Vec<Rule>;

trait RulesManager {
    fn create_rule(&mut self, chain: Chain, conditions: Condition, actions: Vec<ActionType>) -> &Rule;
    fn remove_rule(&mut self, rule_id: usize) -> Option<Rule>;
}

impl RulesManager for RulesList {
    fn create_rule(&mut self, chain: Chain, conditions: Condition, actions: Vec<ActionType>) -> &Rule {
        let rule = Rule::new_rule(chain, conditions, actions);
        self.push(rule);
        self.last().unwrap()
    }

    fn remove_rule(&mut self, rule_id: usize) -> Option<Rule> {
        match self.get(rule_id) {
            Some(_) => Some(self.remove(rule_id)),
            None => None
        }
    }
}
