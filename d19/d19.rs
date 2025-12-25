use nom::Parser;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::combinator::{eof, map};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Replacement {
    from: String,
    to: String,
}

impl fmt::Display for Replacement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let content = format!("{} => {}", self.from, self.to);
        <String as fmt::Display>::fmt(&content, f)
    }
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

fn apply_replacements(molecule: &str, replacements: &[Replacement]) -> HashSet<String> {
    let mut result = HashSet::new();
    for replacement in replacements {
        for (index, _) in molecule.match_indices(&replacement.from) {
            let mut s = molecule.to_string();
            s.replace_range(index..index + replacement.from.len(), &replacement.to);
            result.insert(s);
        }
    }
    result
}

fn apply_replacements_reverse_verbose<'r>(
    molecule: &str,
    replacements: &'r [Replacement],
) -> Vec<(String, usize, &'r Replacement)> {
    let mut result = Vec::new();
    for replacement in replacements {
        for (index, _) in molecule.match_indices(&replacement.to) {
            let mut s = molecule.to_string();
            s.replace_range(index..index + replacement.to.len(), &replacement.from);
            result.push((s, index, replacement));
        }
    }
    result
}

fn synthesize<'r>(
    molecule: &str,
    goal: &str,
    replacements: &'r [Replacement],
) -> Result<Vec<(String, usize, &'r Replacement)>, String> {
    let h = |a: &str| strsim::levenshtein(a, goal) as i64;

    let mut open_set = HashSet::from([molecule.to_string()]);
    let mut came_from = HashMap::<_, (String, usize, &'r Replacement)>::new();
    let mut g_score = HashMap::from([(molecule.to_string(), 0i64)]);
    let mut f_score = HashMap::from([(molecule.to_string(), h(molecule))]);

    while let Some(current) = open_set
        .iter()
        .min_by_key(|s| f_score.get(s.as_str()).copied().unwrap_or(i64::MAX))
    {
        if current == goal {
            let mut total_path = Vec::new();
            let mut current = current;
            while came_from.contains_key(current) {
                let prev = came_from.get(current).unwrap();
                current = &prev.0;
                total_path.push(prev.clone());
            }
            return Ok(total_path);
        }

        let current = current.clone();
        open_set.remove(&current);

        for (neighbor, index, replacement) in
            apply_replacements_reverse_verbose(&current, replacements)
        {
            let tentative_g_score = g_score.get(&current).copied().unwrap_or(i64::MAX);
            if tentative_g_score < g_score.get(&neighbor).copied().unwrap_or(i64::MAX) {
                came_from.insert(neighbor.clone(), (current.clone(), index, replacement));
                g_score.insert(neighbor.clone(), tentative_g_score);
                f_score.insert(neighbor.clone(), tentative_g_score + h(&neighbor));
                open_set.insert(neighbor);
            }
        }
    }

    Err("Didn't work".to_string())
}

fn main() {
    let (replacements, molecule) = parse_input(include_str!("input.txt")).unwrap();

    let result = apply_replacements(&molecule, &replacements);
    println!("Part1: {}", result.len());

    let result = synthesize(&molecule, "e", &replacements).unwrap();
    println!("Part2: {}", result.len());

    // verbose printout because it's very cool:
    println!("Replacements to get from 'e' to '{}':", molecule);
    println!("{: <23} | e", "BEGIN");
    for (s, i, r) in result {
        println!("{: <16} at {: >3} | {}", r, i, s);
    }
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

    #[test]
    fn test_apply_replacements() {
        let (replacements, molecule) = parse_input("H => HO\nH => OH\nO => HH\n\nHOH").unwrap();
        let expected = HashSet::from(["HOOH", "HOHO", "OHOH", "HHHH"].map(str::to_string));
        let actual = apply_replacements(&molecule, &replacements);
        assert_eq!(actual, expected);
    }
}
