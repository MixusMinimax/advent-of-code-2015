use std::cmp;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

fn tsp(n: u16, dist: impl Fn(u16, u16) -> u32) -> u32 {
    let mut g = HashMap::new();
    for k in 0..n {
        g.insert((1u64 << k, k), dist(0, k));
    }

    for s in 2..=n - 1 {
        for sub in 0u64..(1u64 << n) {
            if sub.count_ones() as u16 == s {
                for k in 0..n {
                    if ((1 << k) & sub) != 0 {
                        let mut result = u32::MAX;
                        for m in 0..n {
                            if m != k && ((1 << m) & sub) != 0 {
                                result = cmp::min(result, g[&(sub & !(1 << k), m)] + dist(m, k));
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
        .min()
        .unwrap()
}

fn long_tsp(n: u16, dist: impl Fn(u16, u16) -> u32) -> u32 {
    let mut g = HashMap::new();
    for k in 0..n {
        g.insert((1u64 << k, k), dist(0, k));
    }

    for s in 2..=n - 1 {
        for sub in 0u64..(1u64 << n) {
            if sub.count_ones() as u16 == s {
                for k in 0..n {
                    if ((1 << k) & sub) != 0 {
                        let mut result = 0u32;
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Edge(String, String, u32);

impl FromStr for Edge {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        || -> Option<Self> {
            let mut words = s.split(' ');
            let a = words.next()?.to_string();
            words.next()?;
            let b = words.next()?.to_string();
            words.next()?;
            let d: u32 = words.next()?.parse().ok()?;
            Some(Edge(a, b, d))
        }()
        .ok_or(())
    }
}

fn main() {
    // let input = include_str!("sample.txt");
    let input = include_str!("input.txt");
    let edges: Vec<Edge> = input
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap();
    let cities = edges
        .iter()
        .flat_map(|e| [e.0.as_str(), e.1.as_str()])
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let city_index = cities
        .iter()
        .enumerate()
        .map(|(i, &n)| (n, i))
        .collect::<HashMap<_, _>>();

    let n = cities.len() + 1;

    let mut edge_weights = vec![u32::MAX; n * n];
    let idx = |x: usize, y: usize| y * n + x;
    for i in 0..n {
        edge_weights[idx(0, i)] = 0;
        edge_weights[idx(i, 0)] = 0;
    }

    for e in &edges {
        let i0 = city_index[e.0.as_str()] + 1;
        let i1 = city_index[e.1.as_str()] + 1;
        edge_weights[idx(i0, i1)] = e.2;
        edge_weights[idx(i1, i0)] = e.2;
    }

    println!("{:?}", cities);
    println!("{:?}", city_index);
    println!("{:?}", edge_weights);

    let shortest = tsp(n as u16, |a, b| edge_weights[idx(a as usize, b as usize)]);
    println!("Shortest: {}", shortest);

    let longest = long_tsp(n as u16, |a, b| edge_weights[idx(a as usize, b as usize)]);
    println!("Longest: {}", longest);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            "London to Dublin = 464".parse(),
            Ok(Edge("London".to_string(), "Dublin".to_string(), 464))
        );
        assert_eq!(
            "London to Dublin = 464 ignored extra stuff".parse(),
            Ok(Edge("London".to_string(), "Dublin".to_string(), 464))
        );
        assert_eq!("London to Dublin = asd".parse(), Err::<Edge, ()>(()));
        assert_eq!("London to Dublin = ".parse(), Err::<Edge, ()>(()));
        assert_eq!("London to Dublin =".parse(), Err::<Edge, ()>(()));
        assert_eq!("".parse(), Err::<Edge, ()>(()));
    }

    #[test]
    fn test_simple1() {
        assert_eq!(
            tsp(3, |a, b| {
                [
                    // .
                    [0, 1, 10],
                    [1, 0, 1],
                    [10, 1, 0], // .
                ][b as usize][a as usize]
            }),
            12
        );
    }

    #[test]
    fn test_simple2() {
        assert_eq!(
            tsp(4, |a, b| {
                [
                    // 1   2    3    4
                    [0, 1, 100, 100], // 1
                    [1, 0, 1, 100],   // 2
                    [100, 1, 0, 1],   // 3
                    [100, 100, 1, 0], // 4
                ][b as usize][a as usize]
            }),
            103
        );
    }

    #[test]
    fn test_simple3() {
        assert_eq!(
            tsp(4, |a, b| {
                [
                    // 1   2    3    4
                    [0, 0, 0, 0],  // 1
                    [0, 0, 1, 10], // 2
                    [0, 1, 0, 1],  // 3
                    [0, 10, 1, 0], // 4
                ][b as usize][a as usize]
            }),
            2
        );
    }
}
