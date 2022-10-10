mod common;
use common::*;

const BLOCK_INDEX: u128 = 15671840;
const TX_INDEX: u128 = 12546;
const EVENT_INDEX: u128 = 9876;
const CALLTRACE_INDEX: u128 = 9876;
const TIME: u64 = 1664851463;
const EVENT_ID: &str = "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef";

fn create_transaction(events_number: usize, calltraces_number: usize) -> Transaction {
    Transaction::new(
        BLOCK_INDEX,
        TX_INDEX,
        TIME,
        vec![create_default_event(); events_number],
        vec![create_default_calltrace(); calltraces_number],
        HashMap::new(),
    )
}

fn create_default_transaction() -> Transaction {
    create_transaction(1, 1)
}

fn create_default_event() -> Event {
    Event::new(
        BLOCK_INDEX,
        TX_INDEX,
        EVENT_INDEX,
        string_to_binary_vec(EVENT_ID),
        HashMap::new(),
    )
}

fn create_default_calltrace() -> CallTrace {
    CallTrace::new(
        BLOCK_INDEX,
        TX_INDEX,
        CALLTRACE_INDEX,
        vec![create_default_event()],
        HashMap::new(),
    )
}

fn create_default_true_comparison() -> Comparison {
    create_reference_value_comparison(
        "$.block_index",
        Value::UInt128(U256::from(BLOCK_INDEX)),
        ComparisonOperator::Equal,
    )
}

fn create_default_false_comparison() -> Comparison {
    create_reference_value_comparison(
        "$.block_index",
        Value::UInt128(U256::from(BLOCK_INDEX)),
        ComparisonOperator::NotEqual,
    )
}

fn create_default_true_events_sequence_expression() -> Vec<Expression> {
    vec![
        Expression::Comparison(create_value_reference_comparison(
            "$.block_index",
            Value::UInt128(U256::from(BLOCK_INDEX)),
            ComparisonOperator::Equal,
        )),
        Expression::Comparison(create_reference_value_comparison(
            "$.tx_index",
            Value::UInt128(U256::from(TX_INDEX)),
            ComparisonOperator::Equal,
        )),
        Expression::Comparison(create_value_reference_comparison(
            "$.event_index",
            Value::UInt128(U256::from(EVENT_INDEX)),
            ComparisonOperator::Equal,
        )),
        Expression::Comparison(create_reference_value_comparison(
            "$.event_id",
            Value::UInt128(U256::from(TX_INDEX)),
            ComparisonOperator::NotEqual,
        )),
        Expression::Comparison(create_reference_value_comparison(
            "$.event_id",
            Value::Binary(string_to_binary_vec(EVENT_ID)),
            ComparisonOperator::Equal,
        )),
    ]
}

fn create_default_true_calltraces_sequence_expression() -> Vec<Expression> {
    vec![
        Expression::Comparison(create_value_reference_comparison(
            "$.block_index",
            Value::UInt128(U256::from(BLOCK_INDEX)),
            ComparisonOperator::Equal,
        )),
        Expression::Comparison(create_reference_value_comparison(
            "$.tx_index",
            Value::UInt128(U256::from(TX_INDEX)),
            ComparisonOperator::Equal,
        )),
        Expression::Comparison(create_value_reference_comparison(
            "$.calltrace_index",
            Value::UInt128(U256::from(EVENT_INDEX)),
            ComparisonOperator::Equal,
        )),
    ]
}

fn create_default_false_calltraces_sequence_expression() -> Vec<Expression> {
    vec![
        Expression::Comparison(create_value_reference_comparison(
            "$.block_index",
            Value::UInt128(U256::from(BLOCK_INDEX)),
            ComparisonOperator::Equal,
        )),
        Expression::Comparison(create_reference_value_comparison(
            "$.tx_index",
            Value::UInt128(U256::from(TX_INDEX)),
            ComparisonOperator::NotEqual,
        )),
        Expression::Comparison(create_value_reference_comparison(
            "$.calltrace_index",
            Value::UInt128(U256::from(EVENT_INDEX)),
            ComparisonOperator::Equal,
        )),
    ]
}

#[test]
fn rule_tx_matching_happy_flow() {
    let comparison_true = create_reference_value_comparison(
        "$.tx_index",
        Value::UInt128(U256::from(TX_INDEX)),
        ComparisonOperator::Equal,
    );

    let condition_simple_true = create_condition_with_comparison(
        create_default_true_comparison(),
        create_default_false_comparison(),
        ConditionOperator::Or,
    );

    let condition_complex_true = create_condition_with_condition(
        condition_simple_true,
        comparison_true,
        ConditionOperator::And,
    );

    let rule = Rule::new(Expression::Condition(condition_complex_true.clone()));
    let rule_verification_context =
        VerificationRuleContext::new(true, Expression::Condition(condition_complex_true));

    assert_eq!(
        rule.verify(&create_default_transaction(), None).unwrap(),
        rule_verification_context
    );
}

#[test]
#[should_panic(expected = "NoPath(\"$.event_index\")")]
fn rule_tx_matching_non_existent_reference() {
    let comparison_non_existent_tx_reference = create_reference_value_comparison(
        "$.event_index",
        Value::UInt128(U256::from(125u8)),
        ComparisonOperator::LessThan,
    );

    let condition = create_condition_with_comparison(
        create_default_true_comparison(),
        comparison_non_existent_tx_reference,
        ConditionOperator::And,
    );

    let rule = Rule::new(Expression::Condition(condition));

    rule.verify(&create_default_transaction(), None).unwrap();
}

#[test]
#[should_panic(expected = "SerializationError(Error(\"expected value\", line: 1, column: 1))")]
fn rule_tx_matching_reference_to_calltraces_vector() {
    let comparison_wrong_reference = create_reference_value_comparison(
        "$.calltraces",
        Value::UInt128(U256::from(125u8)),
        ComparisonOperator::LessThan,
    );

    let rule = Rule::new(Expression::Comparison(comparison_wrong_reference));

    rule.verify(&create_default_transaction(), None).unwrap();
}

#[test]
fn rule_tx_matching_different_value_types() {
    let comparison_different_value_types = create_reference_value_comparison(
        "$.block_index",
        Value::Binary(string_to_binary_vec(EVENT_ID)),
        ComparisonOperator::LessThan,
    );

    let condition = create_condition_with_comparison(
        create_default_true_comparison(),
        comparison_different_value_types,
        ConditionOperator::And,
    );

    let rule = Rule::new(Expression::Condition(condition.clone()));
    let rule_verification_context =
        VerificationRuleContext::new(false, Expression::Condition(condition));

    assert_eq!(
        rule.verify(&create_default_transaction(), None).unwrap(),
        rule_verification_context
    );
}

#[test]
fn rule_tx_matching_events_sequence() {
    let expression = create_default_true_events_sequence_expression();

    let rule = Rule::new(Expression::EventsSequence(expression.clone()));
    let rule_verification_context =
        VerificationRuleContext::new(true, Expression::EventsSequence(expression.clone()));

    let tx_pass_rule_verification = create_transaction(expression.len(), expression.len());

    let tx_fail_rule_verification = create_transaction(expression.len() + 1, expression.len());

    assert_eq!(
        rule.verify(&tx_pass_rule_verification, None).unwrap(),
        rule_verification_context
    );

    assert_ne!(
        rule.verify(&tx_fail_rule_verification, None).unwrap(),
        rule_verification_context
    );
}

#[test]
fn rule_tx_matching_true_calltrace_sequence_expression() {
    let expression = create_default_true_calltraces_sequence_expression();

    let rule = Rule::new(Expression::CallTracesSequence(expression.clone()));
    let rule_verification_context =
        VerificationRuleContext::new(true, Expression::CallTracesSequence(expression.clone()));

    let tx_pass_rule_verification = create_transaction(expression.len(), expression.len());

    let tx_fail_rule_verification = create_transaction(expression.len(), expression.len() + 1);

    assert_eq!(
        rule.verify(&tx_pass_rule_verification, None).unwrap(),
        rule_verification_context
    );

    assert_ne!(
        rule.verify(&tx_fail_rule_verification, None).unwrap(),
        rule_verification_context
    );
}

#[test]
fn rule_tx_matching_false_calltrace_sequence_expression() {
    let expression = create_default_false_calltraces_sequence_expression();

    let rule = Rule::new(Expression::CallTracesSequence(expression.clone()));
    let rule_verification_context =
        VerificationRuleContext::new(false, Expression::CallTracesSequence(expression.clone()));

    let tx_pass_rule_verification = create_transaction(expression.len(), expression.len());

    assert_eq!(
        rule.verify(&tx_pass_rule_verification, None).unwrap(),
        rule_verification_context
    );
}
