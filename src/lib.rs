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
