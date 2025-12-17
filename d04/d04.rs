use md5::{Digest, Md5};

fn find_number1(input: &str) -> u64 {
    for x in 1u64..u64::MAX {
        let mut hasher = Md5::new();
        hasher.update(format!("{}{}", input, x));
        let result = hasher.finalize();
        if result[0] == 0 && result[1] == 0 && ((result[2] & 0xf0) == 0) {
            return x;
        }
    }
    panic!();
}

fn find_number2(input: &str) -> u64 {
    for x in 1u64..u64::MAX {
        let mut hasher = Md5::new();
        hasher.update(format!("{}{}", input, x));
        let result = hasher.finalize();
        if result[0] == 0 && result[1] == 0 && result[2] == 0 {
            return x;
        }
    }
    panic!();
}

fn main() {
    let input = include_str!("input.txt");
    let number = find_number1(input);
    println!("Part1: {number}");
    let number = find_number2(input);
    println!("Part2: {number}");
}
