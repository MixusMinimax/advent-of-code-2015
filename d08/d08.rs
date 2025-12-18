use nom::branch::alt;
use nom::bytes::streaming::{is_not, take_while_m_n};
use nom::character::streaming::{char, multispace1};
use nom::combinator::{map, map_res, value, verify};
use nom::error::{FromExternalError, ParseError};
use nom::multi::fold;
use nom::sequence::{delimited, preceded};
use nom::{IResult, Parser};

fn parse_code<'a, E>(input: &'a str) -> IResult<&'a str, u8, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    let parse_hex = take_while_m_n(2, 2, |c: char| c.is_ascii_hexdigit());
    let parse_delimited_hex = preceded(char('x'), parse_hex);
    let mut parse_u8 = map_res(parse_delimited_hex, move |hex| u8::from_str_radix(hex, 16));
    parse_u8.parse(input)
}

fn parse_escaped_char<'a, E>(input: &'a str) -> IResult<&'a str, u8, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    preceded(
        char('\\'),
        alt((
            parse_code,
            value(b'\n', char('n')),
            value(b'\r', char('r')),
            value(b'\t', char('t')),
            value(b'\\', char('\\')),
            value(b'/', char('/')),
            value(b'"', char('"')),
        )),
    )
    .parse(input)
}

fn parse_escaped_whitespace<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, &'a str, E> {
    preceded(char('\\'), multispace1).parse(input)
}

fn parse_literal<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a [u8], E> {
    let not_quote_slash = is_not("\"\\");
    map(
        verify(not_quote_slash, |s: &str| !s.is_empty()),
        str::as_bytes,
    )
    .parse(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StringFragment<'a> {
    Literal(&'a [u8]),
    EscapedChar(u8),
    EscapedWS,
}

fn parse_fragment<'a, E>(input: &'a str) -> IResult<&'a str, StringFragment<'a>, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    alt((
        map(parse_literal, StringFragment::Literal),
        map(parse_escaped_char, StringFragment::EscapedChar),
        value(StringFragment::EscapedWS, parse_escaped_whitespace),
    ))
    .parse(input)
}

fn parse_string<'a, E>(input: &'a str) -> IResult<&'a str, Vec<u8>, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    let build_string = fold(0.., parse_fragment, Vec::new, |mut string, fragment| {
        match fragment {
            StringFragment::Literal(s) => string.extend_from_slice(s),
            StringFragment::EscapedChar(c) => string.push(c),
            StringFragment::EscapedWS => {}
        }
        string
    });
    delimited(char('"'), build_string, char('"')).parse(input)
}

fn main() {
    let input = include_str!("input.txt");
    let mut count = 0usize;
    for l in input.lines() {
        count += l.trim().len() - parse_string::<()>(l.trim()).unwrap().1.len();
    }
    println!("Par1: {}", count);
    let mut count = 0usize;
    for l in input.lines() {
        count += l.trim().replace('\\', "\\\\").replace('"', "\\\"").len() + 2 - l.trim().len();
    }
    println!("Par2: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test2() {
        assert_eq!(parse_string::<()>("\"abc\"").unwrap().1, b"abc");
        assert_eq!(parse_string::<()>("\"aaa\\\"aaa\"").unwrap().1, b"aaa\"aaa");
        assert_eq!(parse_string::<()>("\"\\x27\"").unwrap().1, b"'");
    }
}
