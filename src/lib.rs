pub mod graph;

use std::collections::HashMap;
use std::{cmp, fmt};

pub fn tsp(n: u16, dist: impl Fn(u16, u16) -> i32) -> i32 {
    let mut g = HashMap::new();
    for k in 0..n {
        g.insert((1u64 << k, k), dist(0, k));
    }

    for s in 2..=n - 1 {
        for sub in 0u64..(1u64 << (n - 1)) {
            let sub = sub << 1;
            if sub.count_ones() as u16 == s {
                for k in 0..n {
                    if ((1 << k) & sub) != 0 {
                        let mut result = i32::MAX;
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

pub fn inv_tsp(n: u16, dist: impl Fn(u16, u16) -> i32) -> i32 {
    -tsp(n, |a, b| -dist(a, b))
}

pub struct CompositionsGenerator<I> {
    stack: Vec<(I, I, Vec<I>)>,
    partial_no_zeroes: bool,
}

impl<I> Iterator for CompositionsGenerator<I>
where
    I: Copy,
    I: std::ops::Add<Output = I>,
    I: PartialEq<I>,
    I: std::ops::Sub<Output = I>,
    std::ops::Range<I>: Iterator<Item = I>,
    I: TryFrom<i32>,
    <I as TryFrom<i32>>::Error: fmt::Debug,
{
    type Item = Vec<I>;

    fn next(&mut self) -> Option<Self::Item> {
        let zero = I::try_from(0).unwrap();
        let one = I::try_from(1).unwrap();
        while let Some((vars_left, remaining, mut current)) = self.stack.pop() {
            if vars_left == one && (!self.partial_no_zeroes || remaining != zero) {
                current.push(remaining);
                return Some(current);
            } else {
                for i in (if self.partial_no_zeroes { one } else { zero })..remaining + one {
                    let mut current = current.clone();
                    current.push(i);
                    self.stack.push((vars_left - one, remaining - i, current))
                }
                if self.partial_no_zeroes && remaining == zero {
                    return Some(current);
                }
            }
        }
        None
    }
}

impl<I> CompositionsGenerator<I> {
    pub fn new(n: I, total: I) -> Self {
        CompositionsGenerator {
            stack: vec![(n, total, vec![])],
            partial_no_zeroes: false,
        }
    }

    pub fn new_partial(n: I, total: I) -> Self {
        CompositionsGenerator {
            stack: vec![(n, total, vec![])],
            partial_no_zeroes: true,
        }
    }
}

pub fn compositions<I>(n: I, total: I) -> CompositionsGenerator<I>
where
    I: Copy,
    I: std::ops::Add<Output = I>,
    I: PartialEq<I>,
    I: std::ops::Sub<Output = I>,
    std::ops::Range<I>: Iterator<Item = I>,
    I: TryFrom<i32>,
    <I as TryFrom<i32>>::Error: fmt::Debug,
{
    CompositionsGenerator::<I>::new(n, total)
}

pub fn partial_compositions<I>(n: I, total: I) -> CompositionsGenerator<I>
where
    I: Copy,
    I: std::ops::Add<Output = I>,
    I: PartialEq<I>,
    I: std::ops::Sub<Output = I>,
    std::ops::Range<I>: Iterator<Item = I>,
    I: TryFrom<i32>,
    <I as TryFrom<i32>>::Error: fmt::Debug,
{
    CompositionsGenerator::<I>::new_partial(n, total)
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use std::collections::HashSet;

    #[test]
    fn test_compositions() {
        fn direct_compositions(n: u32, total: u32) -> Vec<Vec<u32>> {
            let mut result = vec![];
            let mut stack = vec![(n, total, vec![])];

            while let Some((vars_left, remaining, mut current)) = stack.pop() {
                if vars_left == 1 {
                    current.push(remaining);
                    result.push(current);
                } else {
                    for i in 0..remaining + 1 {
                        let mut current = current.clone();
                        current.push(i);
                        stack.push((vars_left - 1, remaining - i, current))
                    }
                }
            }

            result
        }

        let direct = direct_compositions(4, 100);
        let generated: Vec<_> = CompositionsGenerator::new(4, 100).collect();
        assert_eq!(direct, generated);
        assert_eq!(generated.len(), 176851);
    }

    #[test]
    fn test_partial_compositions() {
        let expected = HashSet::from(
            [
                &[6] as &[i32],
                &[1, 5],
                &[2, 4],
                &[3, 3],
                &[1, 1, 4],
                &[1, 2, 3],
                &[2, 2, 2],
                &[1, 1, 1, 3],
                &[1, 1, 2, 2],
                &[1, 1, 1, 1, 2],
                &[1, 1, 1, 1, 1, 1],
            ]
            .map(|v| v.iter().copied().sorted().join("+")),
        );
        let actual = partial_compositions(6, 6)
            .map(|v| v.iter().copied().sorted().join("+"))
            .collect::<HashSet<_>>();
        assert_eq!(actual, expected);
    }
}
