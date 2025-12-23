use std::cmp;
use std::collections::HashMap;

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

pub struct CompositionsGenerator {
    stack: Vec<(u32, u32, Vec<u32>)>,
}

impl Iterator for CompositionsGenerator {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((vars_left, remaining, mut current)) = self.stack.pop() {
            if vars_left == 1 {
                current.push(remaining);
                return Some(current);
            } else {
                for i in 0..remaining + 1 {
                    let mut current = current.clone();
                    current.push(i);
                    self.stack.push((vars_left - 1, remaining - i, current))
                }
            }
        }
        None
    }
}

impl CompositionsGenerator {
    pub fn new(n: u32, total: u32) -> Self {
        CompositionsGenerator {
            stack: vec![(n, total, vec![])],
        }
    }
}

pub fn compositions(n: u32, total: u32) -> CompositionsGenerator {
    CompositionsGenerator::new(n, total)
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
