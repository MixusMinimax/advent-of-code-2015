use aoc2015::{inv_tsp, tsp};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Edge(String, String, i32);

impl FromStr for Edge {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        || -> Option<Self> {
            let mut words = s.split(' ');
            let a = words.next()?.to_string();
            words.next()?;
            let b = words.next()?.to_string();
            words.next()?;
            let d: i32 = words.next()?.parse().ok()?;
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

    let mut edge_weights = vec![i32::MAX; n * n];
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

    let longest = inv_tsp(n as u16, |a, b| edge_weights[idx(a as usize, b as usize)]);
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
