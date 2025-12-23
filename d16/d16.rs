use itertools::Itertools;
use nom::Parser;
use nom::bytes::tag;
use nom::character::complete::{alphanumeric1, char, space0};
use nom::combinator::map;
use nom::multi::separated_list0;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq, Default)]
struct Sue {
    number: u32,
    properties: HashMap<String, u32>,
}

impl FromStr for Sue {
    type Err = nom::Err<nom::error::Error<String>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        map(
            (
                tag("Sue"),
                space0,
                nom::character::complete::u32,
                char(':'),
                space0,
                separated_list0(
                    (char(','), space0),
                    (
                        alphanumeric1,
                        char(':'),
                        space0,
                        nom::character::complete::u32,
                    ),
                ),
            ),
            |(_, _, number, _, _, props)| Sue {
                number,
                properties: props
                    .into_iter()
                    .map(|(name, _, _, v)| (str::to_string(name), v))
                    .collect(),
            },
        )
        .parse(s)
        .map(|(_, o)| o)
        .map_err(<nom::Err<nom::error::Error<&str>>>::to_owned)
    }
}

fn main() {
    let input = include_str!("input.txt");
    let sues: Vec<Sue> = input
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap();
    let expected = HashMap::from([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]);

    let sue = sues
        .iter()
        .filter(|sue| {
            sue.properties
                .iter()
                .all(|(prop, &v)| expected[prop.as_str()] == v)
        })
        .exactly_one()
        .unwrap();
    println!("Part1: {}", sue.number);

    let sue = sues
        .iter()
        .filter(|sue| {
            sue.properties.iter().all(
                |(prop, &v)| match (prop.as_str(), expected[prop.as_str()]) {
                    ("cats" | "trees", expected) => v > expected,
                    ("pomeranians" | "goldfish", expected) => v < expected,
                    (_, expected) => v == expected,
                },
            )
        })
        .exactly_one()
        .unwrap();
    println!("Part2: {}", sue.number);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            "Sue 69:".parse(),
            Ok(Sue {
                number: 69,
                properties: HashMap::from([])
            })
        );
        assert_eq!(
            "Sue 69: perfumes: 7, vizslas: 9, akitas: 1".parse(),
            Ok(Sue {
                number: 69,
                properties: HashMap::from([
                    ("perfumes".to_string(), 7),
                    ("vizslas".to_string(), 9),
                    ("akitas".to_string(), 1),
                ])
            })
        );
    }
}
