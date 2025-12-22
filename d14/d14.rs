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

#[derive(Clone, Debug, Eq, PartialEq)]
enum ReindeerState {
    Flying(i32),
    Resting(i32),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ReindeerData {
    reindeer: Reindeer,
    state: ReindeerState,
    distance_covered: i32,
    points: i32,
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

fn advance_reindeer(reindeer: &Reindeer, state: &ReindeerState) -> ReindeerState {
    use ReindeerState::*;
    match *state {
        Flying(time) if time >= reindeer.flight_seconds - 1 => Resting(0),
        Flying(time) => Flying(time + 1),
        Resting(time) if time >= reindeer.rest_seconds - 1 => Flying(0),
        Resting(time) => Resting(time + 1),
    }
}

fn main() {
    let input = include_str!("input.txt");
    let reindeer: Vec<Reindeer> = input
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap();

    const SECONDS: i32 = 2503;

    // part 1
    let max_dist = reindeer
        .iter()
        .map(|r| reindeer_distance(r, SECONDS))
        .max()
        .unwrap();
    println!("Part1: {}", max_dist);

    // part 2
    let mut reindeer: Vec<_> = reindeer
        .into_iter()
        .map(|r| ReindeerData {
            reindeer: r,
            state: ReindeerState::Flying(0),
            distance_covered: 0i32,
            points: 0i32,
        })
        .collect();
    for _ in 0..SECONDS {
        let max_dist = reindeer
            .iter_mut()
            .map(|data| {
                if let ReindeerState::Flying(_) = data.state {
                    data.distance_covered += data.reindeer.speed;
                }
                data.state = advance_reindeer(&data.reindeer, &data.state);
                data.distance_covered
            })
            .max()
            .unwrap();
        reindeer.iter_mut().for_each(|data| {
            if data.distance_covered == max_dist {
                data.points += 1;
            }
        });
    }
    let winner = reindeer.iter().map(|data| data.points).max().unwrap();
    println!("Part2: {}", winner);
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

    #[test]
    fn test_advance_reindeer() {
        use ReindeerState::*;
        let comet = Reindeer {
            name: "Comet".to_string(),
            speed: 14,
            flight_seconds: 10,
            rest_seconds: 127,
        };
        assert_eq!(advance_reindeer(&comet, &Flying(0)), Flying(1));
        assert_eq!(advance_reindeer(&comet, &Flying(8)), Flying(9));
        assert_eq!(advance_reindeer(&comet, &Flying(9)), Resting(0));
        assert_eq!(advance_reindeer(&comet, &Resting(0)), Resting(1));
        assert_eq!(advance_reindeer(&comet, &Resting(125)), Resting(126));
        assert_eq!(advance_reindeer(&comet, &Resting(126)), Flying(0));
    }
}
