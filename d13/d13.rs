use itertools::Itertools;
use std::cmp;
use std::collections::HashMap;
use std::str::FromStr;

fn long_tsp(n: u16, dist: impl Fn(u16, u16) -> i32) -> i32 {
    let mut g = HashMap::new();
    for k in 0..n {
        g.insert((1u64 << k, k), dist(0, k));
    }

    for s in 2..=n - 1 {
        for sub in 0u64..(1u64 << n) {
            if sub.count_ones() as u16 == s {
                for k in 0..n {
                    if ((1 << k) & sub) != 0 {
                        let mut result = 0i32;
                        for m in 0..n {
                            if m != k && ((1 << m) & sub) != 0 {
                                result = cmp::max(result, g[&(sub & !(1 << k), m)] + dist(m, k));
                            }
                        }
                        g.insert((sub, k), result);
                    }
                }
            }
        }
    }

    (1..n)
        .map(|k| g[&(((1u64 << n) - 1) & !1u64, k)] + dist(k, 0))
        .max()
        .unwrap()
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Instruction(String, i32, String);

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        || -> Option<_> {
            let mut words = s.split(" ");
            let a = words.next()?.to_string();
            words.next()?;
            let amount = match words.next()? {
                "gain" => 1,
                "lose" => -1,
                _ => return None,
            } * words.next()?.parse::<i32>().ok()?;
            (&mut words).take(6).last()?;
            let b = words.next()?.split(".").next()?.to_string();
            Some(Instruction(a, amount, b))
        }()
        .ok_or(())
    }
}

fn main() {
    // let input = include_str!("sample.txt");
    let input = include_str!("input.txt");
    let instructions: Vec<Instruction> = input
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap();
    let names: Vec<_> = instructions
        .iter()
        .flat_map(|i| [i.0.as_str(), i.2.as_str()])
        .unique()
        .collect();
    let name_to_index: HashMap<_, _> = names.iter().enumerate().map(|(i, &n)| (n, i)).collect();
    let n = names.len();
    let mut matrix = vec![0; n * n];
    let idx = |x: usize, y: usize| y * n + x;
    for ins in &instructions {
        let ia = name_to_index[ins.0.as_str()];
        let ib = name_to_index[ins.2.as_str()];
        matrix[idx(ia, ib)] = ins.1;
    }
    let best_happiness = long_tsp(n as u16, |a, b| {
        matrix[idx(a as usize, b as usize)] + matrix[idx(b as usize, a as usize)]
    });
    println!("Part1: {}", best_happiness);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            "Alice would gain 54 happiness units by sitting next to Bob.".parse(),
            Ok(Instruction("Alice".to_string(), 54, "Bob".to_string()))
        );
        assert_eq!(
            "Alice would lose 79 happiness units by sitting next to Carol.".parse(),
            Ok(Instruction("Alice".to_string(), -79, "Carol".to_string()))
        );
    }
}
