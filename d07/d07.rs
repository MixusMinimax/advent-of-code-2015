#![feature(assert_matches)]

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Value {
    Lit(u16),
    Var(String),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Unop {
    Not,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Binop {
    And,
    Or,
    LShift,
    RShift,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Expression {
    Value(Value),
    Unary(Unop, Value),
    Binary(Value, Binop, Value),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Wiring {
    expression: Expression,
    output: String,
}

mod grammar {
    use super::*;
    use nom::IResult;
    use nom::Parser;
    use nom::branch::alt;
    use nom::bytes::tag;
    use nom::character::complete::{alpha1, digit1, space0, space1};
    use nom::combinator::{map, map_res, not, peek, value, verify};
    use nom::sequence::terminated;

    const RESERVED: &[&str] = &["AND", "OR", "LSHIFT", "RSHIFT", "NOT"];

    fn parse_unop(i: &str) -> IResult<&str, Unop> {
        terminated(value(Unop::Not, tag("NOT")), not(peek(alpha1))).parse(i)
    }

    fn parse_binop(i: &str) -> IResult<&str, Binop> {
        terminated(
            alt((
                value(Binop::And, tag("AND")),
                value(Binop::Or, tag("OR")),
                value(Binop::LShift, tag("LSHIFT")),
                value(Binop::RShift, tag("RSHIFT")),
            )),
            not(peek(alpha1)),
        )
        .parse(i)
    }

    fn parse_value(i: &str) -> IResult<&str, Value> {
        alt((
            map(map_res(digit1, str::parse), Value::Lit),
            map(
                map(
                    verify(alpha1, |ident| !RESERVED.contains(ident)),
                    String::from,
                ),
                Value::Var,
            ),
        ))
        .parse(i)
    }

    fn parse_expression(i: &str) -> IResult<&str, Expression> {
        alt((
            map((parse_unop, space1, parse_value), |(a, _, b)| {
                Expression::Unary(a, b)
            }),
            map(
                (parse_value, space1, parse_binop, space1, parse_value),
                |(a, _, op, _, b)| Expression::Binary(a, op, b),
            ),
            map(parse_value, Expression::Value),
        ))
        .parse(space0(i)?.0)
    }

    fn parse_wiring(i: &str) {
        alt(());
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::assert_matches::assert_matches;

        #[test]
        fn test_unop() {
            assert_eq!(parse_unop("NOT"), Ok(("", Unop::Not)));
            assert_eq!(parse_unop("NOT t"), Ok((" t", Unop::Not)));
            assert_matches!(parse_unop("NOTt"), Err(nom::Err::Error(_)));
            assert_matches!(parse_unop("NO"), Err(nom::Err::Incomplete(_)));
        }

        #[test]
        fn test_binop() {
            assert_matches!(parse_binop("asd"), Err(nom::Err::Error(_)));
            assert_matches!(parse_binop("NOT"), Err(nom::Err::Error(_)));
            assert_matches!(parse_binop("AND"), Ok(("", Binop::And)));
            assert_matches!(parse_binop("AND d"), Ok((" d", Binop::And)));
            assert_matches!(parse_binop("ANDd"), Err(nom::Err::Error(_)));
            assert_matches!(parse_binop("OR"), Ok(("", Binop::Or)));
            assert_matches!(parse_binop("LSHIFT"), Ok(("", Binop::LShift)));
            assert_matches!(parse_binop("RSHIFT"), Ok(("", Binop::RShift)));
        }

        #[test]
        fn test_value() {
            assert_eq!(parse_value("123"), Ok(("", Value::Lit(123))));
            assert_eq!(parse_value("123 "), Ok((" ", Value::Lit(123))));
            assert_eq!(parse_value("123asd"), Ok(("asd", Value::Lit(123))));
            assert_eq!(parse_value("asd"), Ok(("", Value::Var("asd".to_string()))));
            assert_eq!(
                parse_value("NOT"),
                Err(nom::Err::Error(nom::error::Error::new(
                    "NOT",
                    nom::error::ErrorKind::Verify
                )))
            );
            assert_eq!(
                parse_value("NOTt"),
                Ok(("", Value::Var("NOTt".to_string())))
            );
        }

        #[test]
        fn test_parse_expression() {
            assert_eq!(
                parse_expression("NOT x"),
                Ok((
                    "",
                    Expression::Unary(Unop::Not, Value::Var("x".to_string()))
                ))
            );
            assert_eq!(
                parse_expression("  NOT   x  "),
                Ok((
                    "  ",
                    Expression::Unary(Unop::Not, Value::Var("x".to_string()))
                ))
            );
            assert_eq!(
                parse_expression("123 AND y"),
                Ok((
                    "",
                    Expression::Binary(Value::Lit(123), Binop::And, Value::Var("y".to_string()))
                ))
            )
        }
    }
}

fn main() {}
