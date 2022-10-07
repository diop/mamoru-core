use crate::rule::{Expression, Rule};

pub type RulesList = Vec<Rule>;

trait RulesManager {
    fn create_rule(&mut self, expression: Expression) -> &Rule;
    fn remove_rule(&mut self, rule_id: usize) -> Option<Rule>;
}

impl RulesManager for RulesList {
    fn create_rule(&mut self, expression: Expression) -> &Rule {
        let rule = Rule::new(expression);
        self.push(rule);
        self.last().unwrap()
    }

    fn remove_rule(&mut self, rule_id: usize) -> Option<Rule> {
        match self.get(rule_id) {
            Some(_) => Some(self.remove(rule_id)),
            None => None,
        }
    }
}
