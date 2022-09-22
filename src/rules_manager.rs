use crate::rule::*;

pub type RulesList = Vec<Rule>;

trait RulesManager {
    fn create_rule(&mut self, chain: Chain, conditions: Condition, actions: Vec<ActionType>) -> &Rule;
    fn remove_rule(&mut self, rule_id: usize) -> Option<Rule>;
}

impl RulesManager for RulesList {
    fn create_rule(&mut self, chain: Chain, conditions: Condition, actions: Vec<ActionType>) -> &Rule {
        let rule = Rule::new(chain, conditions, actions);
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