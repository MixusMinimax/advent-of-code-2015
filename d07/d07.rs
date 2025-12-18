#![feature(assert_matches)]

use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Value {
    Lit(u16),
    Var(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lit(i) => write!(f, "{i}"),
            Self::Var(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Unop {
    Not,
}

impl fmt::Display for Unop {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Not => write!(f, "NOT"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Binop {
    And,
    Or,
    LShift,
    RShift,
}

impl fmt::Display for Binop {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::And => write!(f, "AND"),
            Self::Or => write!(f, "OR"),
            Self::LShift => write!(f, "LSHIFT"),
            Self::RShift => write!(f, "RSHIFT"),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Expression {
    Value(Value),
    Unary(Unop, Value),
    Binary(Value, Binop, Value),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Value(v) => write!(f, "{v}"),
            Self::Unary(op, v) => write!(f, "{} {}", op, v),
            Self::Binary(lhs, op, rhs) => write!(f, "{} {} {}", lhs, op, rhs),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Wiring {
    expression: Expression,
    output: String,
}

impl fmt::Display for Wiring {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.expression, self.output)
    }
}

mod grammar {
    use super::*;
    use nom::IResult;
    use nom::Parser;
    use nom::branch::alt;
    use nom::bytes::tag;
    use nom::character::complete::{alpha1, digit1, space0};
    use nom::combinator::{map, map_res, not, peek, value, verify};
    use nom::error::ParseError;
    use nom::sequence::{preceded, terminated};

    const RESERVED: &[&str] = &["AND", "OR", "LSHIFT", "RSHIFT", "NOT"];

    pub fn ws<I, O, E: ParseError<I>, G>(inner: G) -> impl Parser<I, Output = O, Error = E>
    where
        G: Parser<I, Output = O, Error = E>,
        I: nom::Input,
        <I as nom::Input>::Item: nom::AsChar,
    {
        preceded(space0, inner)
    }

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
        if let Ok((i2, op)) = parse_unop(i) {
            return ws(map(parse_value, |v| Expression::Unary(op, v))).parse(i2);
        }
        let (i, lhs) = parse_value(i)?;
        if let Ok((i, op)) = ws(parse_binop).parse(i) {
            let (i, rhs) = ws(parse_value).parse(i)?;
            Ok((i, Expression::Binary(lhs, op, rhs)))
        } else {
            Ok((i, Expression::Value(lhs)))
        }
    }

    pub fn parse_wiring(i: &str) -> IResult<&str, Wiring> {
        map(
            (
                ws(parse_expression),
                ws(tag("->")),
                ws(map(alpha1, String::from)),
                space0,
            ),
            |(e, _, name, _)| Wiring {
                expression: e,
                output: name,
            },
        )
        .parse(i)
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
                parse_expression("NOT   x  "),
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
            );
            assert_eq!(
                parse_expression("123 "),
                Ok((" ", Expression::Value(Value::Lit(123))))
            );
        }
    }
}

fn eval_val<'a>(v: &'a Value, values: &HashMap<&'a str, u16>) -> Result<u16, Vec<&'a str>> {
    match v {
        Value::Lit(i) => Ok(*i),
        Value::Var(s) => {
            if let Some(i) = values.get(s.as_str()) {
                Ok(*i)
            } else {
                Err(vec![s.as_str()])
            }
        }
    }
}

fn eval_expr<'a>(e: &'a Expression, values: &HashMap<&'a str, u16>) -> Result<u16, Vec<&'a str>> {
    match e {
        Expression::Value(v) => eval_val(v, values),
        Expression::Unary(op, v) => eval_val(v, values).map(|i| match op {
            Unop::Not => !i,
        }),
        Expression::Binary(l, op, r) => eval_val(l, values)
            .and_then(|l| eval_val(r, values).map(|r| (l, r)))
            .map(|(l, r)| match op {
                Binop::And => l & r,
                Binop::Or => l | r,
                Binop::LShift => l << r,
                Binop::RShift => l >> r,
            }),
    }
}

fn main() {
    let input = include_str!("input.txt");
    let wirings: HashMap<String, Wiring> = input
        .lines()
        .map(|line| {
            grammar::parse_wiring(line)
                .expect("Failed to parse wiring")
                .1
        })
        .map(|wiring| (wiring.output.clone(), wiring))
        .collect();
    let mut values = HashMap::new();
    let mut evaluation_queue = VecDeque::from(["a"]);
    while let Some(&next) = evaluation_queue.front() {
        if values.contains_key(next) {
            evaluation_queue.pop_front();
            continue;
        }
        let wiring = &wirings[next];
        match eval_expr(&wiring.expression, &values) {
            Ok(i) => {
                println!("{next} evaluated to {i}");
                values.insert(next, i);
                evaluation_queue.pop_front();
            }
            Err(requirements) => {
                println!("{next} requires {requirements:?}");
                for r in requirements {
                    evaluation_queue.push_front(r);
                }
            }
        };
    }

    println!("Result: {}", values["a"]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_expr_lit() {
        let expr = Expression::Value(Value::Lit(123));
        let values = HashMap::new();
        let result = eval_expr(&expr, &values);
        assert_eq!(result, Ok(123));
    }

    #[test]
    fn test_eval_expr_var() {
        let expr = Expression::Value(Value::Var("asd".to_string()));
        let values = HashMap::from([("asd", 69)]);
        let result = eval_expr(&expr, &values);
        assert_eq!(result, Ok(69));
    }

    #[test]
    fn test_eval_expr_var_notfound() {
        let expr = Expression::Value(Value::Var("asd".to_string()));
        let values = HashMap::new();
        let result = eval_expr(&expr, &values);
        assert_eq!(result, Err(vec!["asd"]));
    }
}
