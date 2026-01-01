/// [v] is \[row, col\]
fn n2ton(v: [u64; 2]) -> u64 {
    let row = v[0] + v[1] - 1;
    (row - 1) * row / 2 + 1 + v[1] - 1
}

fn hash(i: u64) -> u64 {
    (i * 252533) % 33554393
}

fn get_code(v: [u64; 2]) -> u64 {
    (1..n2ton(v)).fold(20151125u64, |acc, _| hash(acc))
}

fn main() {
    println!("Part1: {}", get_code([2978, 3083]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n2ton() {
        assert_eq!(n2ton([1, 1]), 1);
        assert_eq!(n2ton([1, 2]), 3);
        assert_eq!(n2ton([1, 3]), 6);
        assert_eq!(n2ton([2, 1]), 2);
        assert_eq!(n2ton([2, 2]), 5);
        assert_eq!(n2ton([2, 3]), 9);
        assert_eq!(n2ton([3, 1]), 4);
        assert_eq!(n2ton([3, 2]), 8);
        assert_eq!(n2ton([3, 3]), 13);
    }

    #[test]
    fn test_hash() {
        assert_eq!(hash(20151125), 31916031);
        assert_eq!(hash(31916031), 18749137);
        assert_eq!(hash(18749137), 16080970);
        assert_eq!(hash(16080970), 21629792);
        assert_eq!(hash(21629792), 17289845);
    }

    #[test]
    fn test_code() {
        assert_eq!(get_code([1, 1]), 20151125);
        assert_eq!(get_code([3, 2]), 8057251);
        assert_eq!(get_code([5, 6]), 31663883);
        assert_eq!(get_code([6, 6]), 27995004);
    }
}
