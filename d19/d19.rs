use nom::Parser;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::combinator::{eof, map};
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Replacement {
    from: String,
    to: String,
}

impl FromStr for Replacement {
    type Err = nom::Err<nom::error::Error<String>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        map((alpha1, tag(" => "), alpha1, eof), |(from, _, to, ..)| {
            Replacement {
                from: str::to_string(from),
                to: str::to_string(to),
            }
        })
        .parse(s)
        .map(|(_, r)| r)
        .map_err(<nom::Err<nom::error::Error<&str>>>::to_owned)
    }
}

fn parse_input(
    input: &str,
) -> Result<(Vec<Replacement>, String), nom::Err<nom::error::Error<String>>> {
    enum State {
        Replacements,
        Molecule,
    }
    let mut state = State::Replacements;
    let mut vec = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            state = State::Molecule;
        } else if let State::Molecule = state {
            return Ok((vec, line.to_string()));
        } else {
            vec.push(line.parse()?);
        }
    }
    Err(nom::Err::Incomplete(nom::Needed::Unknown))
}

fn main() {
    let (replacements, molecule) = parse_input(include_str!("input.txt")).unwrap();
    println!("{:?}, {}", replacements, molecule);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            "Al => ThF".parse(),
            Ok(Replacement {
                from: "Al".to_string(),
                to: "ThF".to_string()
            })
        );
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input("H => HO\nH => OH\nO => HH\n\nHOH"),
            Ok((
                vec![
                    Replacement {
                        from: "H".to_string(),
                        to: "HO".to_string()
                    },
                    Replacement {
                        from: "H".to_string(),
                        to: "OH".to_string()
                    },
                    Replacement {
                        from: "O".to_string(),
                        to: "HH".to_string()
                    },
                ],
                "HOH".to_string()
            ))
        );
    }
}
