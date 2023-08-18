use nom::branch::alt;
use nom::bytes::complete::{tag, take_while, take_while1};
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::sequence::{delimited, preceded, tuple};

use crate::condition::{BinaryOp, Condition, UnaryOp};
use crate::error::ConditionParseError;

pub fn parse_condition(input: &str) -> Result<Condition, ConditionParseError> {
    let (_, condition) =
        parse_expression(input).map_err(|e| ConditionParseError::InvalidCondition {
            condition: input.to_string(),
            error: e.to_string(),
        })?;

    Ok(condition)
}

fn parse_expression(input: &str) -> nom::IResult<&str, Condition> {
    let (input, left) = alt((
        map(
            preceded(parse_unary_op, parse_primary_expression),
            |condition| Condition::Unary {
                condition: Box::new(condition),
                op: UnaryOp::Not,
            },
        ),
        parse_comparison_expression,
    ))(input)?;

    if let Ok((input, (op, right))) = tuple((parse_binary_op, parse_expression))(input) {
        return Ok((
            input,
            Condition::Binary {
                left: Box::new(left),
                right: Box::new(right),
                op,
            },
        ));
    }

    Ok((input, left))
}

fn parse_primary_expression(input: &str) -> nom::IResult<&str, Condition> {
    alt((
        map(tag("success()"), |_| Condition::Success),
        map(tag("failure()"), |_| Condition::Failure),
        map(tag("always()"), |_| Condition::Always),
        map(tag("skipped()"), |_| Condition::Skipped),
        delimited(tag("("), parse_expression, tag(")")), // handle brackets
        parse_literal,
        parse_field,
    ))(input)
}

fn parse_binary_op(input: &str) -> nom::IResult<&str, BinaryOp> {
    delimited(
        multispace0,
        alt((
            map(tag("&&"), |_| BinaryOp::And),
            map(tag("||"), |_| BinaryOp::Or),
        )),
        multispace0,
    )(input)
}

fn parse_comparison_expression(input: &str) -> nom::IResult<&str, Condition> {
    let (input, left) = parse_primary_expression(input)?;

    if let Ok((input, (op, right))) = tuple((parse_comparison_op, parse_primary_expression))(input)
    {
        Ok((
            input,
            Condition::Binary {
                left: Box::new(left),
                right: Box::new(right),
                op,
            },
        ))
    } else {
        Ok((input, left))
    }
}

fn parse_comparison_op(input: &str) -> nom::IResult<&str, BinaryOp> {
    delimited(
        multispace0,
        alt((
            map(tag("=="), |_| BinaryOp::Eq),
            map(tag("!="), |_| BinaryOp::Ne),
        )),
        multispace0,
    )(input)
}

fn parse_unary_op(input: &str) -> nom::IResult<&str, UnaryOp> {
    map(tag("!"), |_| UnaryOp::Not)(input)
}

fn parse_literal(input: &str) -> nom::IResult<&str, Condition> {
    alt((
        // Parse literals enclosed in "
        map(
            delimited(tag("\""), take_while(is_valid_char_for_literal), tag("\"")),
            |s: &str| Condition::Literal(s.into()),
        ),
        // Parse literals enclosed in '
        map(
            delimited(tag("'"), take_while(is_valid_char_for_literal), tag("'")),
            |s: &str| Condition::Literal(s.into()),
        ),
    ))(input)
}

fn is_valid_char_for_literal(c: char) -> bool {
    // Literals don't support escape sequences.
    c != '\"' && c != '\'' && c != '\\'
}

fn parse_field(input: &str) -> nom::IResult<&str, Condition> {
    // Replace with a more detailed field parser if necessary
    map(
        take_while1(|c: char| c.is_alphanumeric() || c == '.' || c == '_' || c == '-'),
        |s: &str| Condition::Field(s.into()),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn functions() {
        assert_eq!(parse_condition("success()").unwrap(), Condition::Success);
        assert_eq!(parse_condition("failure()").unwrap(), Condition::Failure);
        assert_eq!(parse_condition("always()").unwrap(), Condition::Always);
        assert_eq!(parse_condition("skipped()").unwrap(), Condition::Skipped);
    }

    #[test]
    fn literal() {
        assert_eq!(
            parse_condition("'hello'").unwrap(),
            Condition::Literal("hello".into())
        );
        assert_eq!(
            parse_condition("\"world\"").unwrap(),
            Condition::Literal("world".into())
        );
    }

    #[test]
    fn field() {
        assert_eq!(
            parse_condition("incident.id").unwrap(),
            Condition::Field("incident.id".into())
        );
        assert_eq!(
            parse_condition("incident.level").unwrap(),
            Condition::Field("incident.level".into())
        );
    }

    #[test]
    fn binary() {
        assert_eq!(
            parse_condition("success() && failure()").unwrap(),
            Condition::Binary {
                left: Box::new(Condition::Success),
                right: Box::new(Condition::Failure),
                op: BinaryOp::And,
            }
        );
        assert_eq!(
            parse_condition("success() || failure()").unwrap(),
            Condition::Binary {
                left: Box::new(Condition::Success),
                right: Box::new(Condition::Failure),
                op: BinaryOp::Or,
            }
        );
        assert_eq!(
            parse_condition("success() == failure()").unwrap(),
            Condition::Binary {
                left: Box::new(Condition::Success),
                right: Box::new(Condition::Failure),
                op: BinaryOp::Eq,
            }
        );
        assert_eq!(
            parse_condition("success() != failure()").unwrap(),
            Condition::Binary {
                left: Box::new(Condition::Success),
                right: Box::new(Condition::Failure),
                op: BinaryOp::Ne,
            }
        );
    }

    #[test]
    fn binary_nested() {
        assert_eq!(
            parse_condition("success() && failure() || always()").unwrap(),
            Condition::Binary {
                left: Box::new(Condition::Success),
                right: Box::new(Condition::Binary {
                    left: Box::new(Condition::Failure),
                    right: Box::new(Condition::Always),
                    op: BinaryOp::Or,
                }),
                op: BinaryOp::And,
            }
        );

        assert_eq!(
            parse_condition("(success() && failure()) || always()").unwrap(),
            Condition::Binary {
                left: Box::new(Condition::Binary {
                    left: Box::new(Condition::Success),
                    right: Box::new(Condition::Failure),
                    op: BinaryOp::And,
                }),
                right: Box::new(Condition::Always),
                op: BinaryOp::Or,
            }
        );

        assert_eq!(
            parse_condition("success() && (failure() || (always() && skipped()))").unwrap(),
            Condition::Binary {
                left: Box::new(Condition::Success),
                right: Box::new(Condition::Binary {
                    left: Box::new(Condition::Failure),
                    right: Box::new(Condition::Binary {
                        left: Box::new(Condition::Always),
                        right: Box::new(Condition::Skipped),
                        op: BinaryOp::And,
                    }),
                    op: BinaryOp::Or,
                }),
                op: BinaryOp::And,
            }
        );
    }

    #[test]
    fn unary() {
        assert_eq!(
            parse_condition("!success()").unwrap(),
            Condition::Unary {
                condition: Box::new(Condition::Success),
                op: UnaryOp::Not,
            }
        );

        assert_eq!(
            parse_condition("!success() && failure()").unwrap(),
            Condition::Binary {
                left: Box::new(Condition::Unary {
                    condition: Box::new(Condition::Success),
                    op: UnaryOp::Not,
                }),
                right: Box::new(Condition::Failure),
                op: BinaryOp::And,
            }
        );

        assert_eq!(
            parse_condition("!(!success() && failure())").unwrap(),
            Condition::Unary {
                condition: Box::new(Condition::Binary {
                    left: Box::new(Condition::Unary {
                        condition: Box::new(Condition::Success),
                        op: UnaryOp::Not,
                    }),
                    right: Box::new(Condition::Failure),
                    op: BinaryOp::And,
                }),
                op: UnaryOp::Not,
            }
        );
    }

    #[test]
    fn mixed() {
        assert_eq!(
            parse_condition("success() && incident.id == '123'").unwrap(),
            Condition::Binary {
                left: Box::new(Condition::Success),
                right: Box::new(Condition::Binary {
                    left: Box::new(Condition::Field("incident.id".into())),
                    right: Box::new(Condition::Literal("123".into())),
                    op: BinaryOp::Eq,
                }),
                op: BinaryOp::And,
            }
        );

        assert_eq!(
            parse_condition("incident.id == '123' && success()").unwrap(),
            Condition::Binary {
                left: Box::new(Condition::Binary {
                    left: Box::new(Condition::Field("incident.id".into())),
                    right: Box::new(Condition::Literal("123".into())),
                    op: BinaryOp::Eq,
                }),
                right: Box::new(Condition::Success),
                op: BinaryOp::And,
            }
        );

        assert_eq!(
            parse_condition("success() && incident.id == '123' && incident.level != 'high'")
                .unwrap(),
            Condition::Binary {
                left: Box::new(Condition::Success),
                right: Box::new(Condition::Binary {
                    left: Box::new(Condition::Binary {
                        left: Box::new(Condition::Field("incident.id".into())),
                        right: Box::new(Condition::Literal("123".into())),
                        op: BinaryOp::Eq,
                    }),
                    right: Box::new(Condition::Binary {
                        left: Box::new(Condition::Field("incident.level".into())),
                        right: Box::new(Condition::Literal("high".into())),
                        op: BinaryOp::Ne,
                    }),
                    op: BinaryOp::And,
                }),
                op: BinaryOp::And,
            }
        );
    }
}
