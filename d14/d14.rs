use nom::Parser;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, digit1};
use nom::combinator::{eof, map, map_res};
use std::cmp;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Reindeer {
    name: String,
    speed: i32,
    flight_seconds: i32,
    rest_seconds: i32,
}

impl FromStr for Reindeer {
    type Err = nom::Err<nom::error::Error<String>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        map(
            (
                alphanumeric1,
                tag(" can fly "),
                map_res(digit1, str::parse),
                tag(" km/s for "),
                map_res(digit1, str::parse),
                tag(" seconds, but then must rest for "),
                map_res(digit1, str::parse),
                tag(" seconds."),
                eof,
            ),
            |(name, _, speed, _, flight_seconds, _, rest_seconds, _, _)| Reindeer {
                name: name.to_string(),
                speed,
                flight_seconds,
                rest_seconds,
            },
        )
        .parse(s)
        .map(|(_, r)| r)
        .map_err(|e: nom::Err<nom::error::Error<_>>| e.to_owned())
    }
}

fn reindeer_distance(reindeer: &Reindeer, seconds: i32) -> i32 {
    let cycle_time = reindeer.flight_seconds + reindeer.rest_seconds;
    let full_cycles = seconds / cycle_time;
    let remaining_time = seconds - full_cycles * cycle_time;
    reindeer.speed
        * (full_cycles * reindeer.flight_seconds
            + cmp::min(remaining_time, reindeer.flight_seconds))
}

fn main() {
    let input = include_str!("input.txt");
    let reindeer: Vec<Reindeer> = input
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap();
    let max_dist = reindeer
        .into_iter()
        .map(|r| reindeer_distance(&r, 2503))
        .max()
        .unwrap();
    println!("Part1: {}", max_dist);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.".parse(),
            Ok(Reindeer {
                name: "Comet".to_string(),
                speed: 14,
                flight_seconds: 10,
                rest_seconds: 127,
            })
        );
    }

    #[test]
    fn test_reindeer_distance() {
        assert_eq!(
            reindeer_distance(
                &Reindeer {
                    name: "Comet".to_string(),
                    speed: 14,
                    flight_seconds: 10,
                    rest_seconds: 127,
                },
                1000
            ),
            1120
        );
    }
}
