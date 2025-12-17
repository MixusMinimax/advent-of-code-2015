fn level(s: &str) -> i32 {
    s.chars().fold(0, |acc, c| match c {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc,
    })
}

fn find_basement(s: &str) -> usize {
    s.chars()
        .enumerate()
        .scan(0i32, |acc, (i, c)| {
            match c {
                '(' => *acc += 1,
                ')' => *acc -= 1,
                _ => (),
            }
            Some((i, *acc))
        })
        .find(|&(_, level)| level < 0)
        .map(|(i, _)| i + 1)
        .unwrap_or(0)
}

fn main() {
    println!("Hello World!");
    let input = include_str!("input.txt");
    println!("Part1: {}", level(input));
    println!("Part2: {}", find_basement(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level() {
        assert_eq!(level("(())"), 0);
        assert_eq!(level("()()"), 0);
        assert_eq!(level("((("), 3);
        assert_eq!(level("(()(()("), 3);
        assert_eq!(level("))((((("), 3);
        assert_eq!(level("())"), -1);
        assert_eq!(level("))("), -1);
        assert_eq!(level(")))"), -3);
        assert_eq!(level(")())())"), -3);
    }

    #[test]
    fn test_basement() {
        assert_eq!(find_basement(")"), 1);
        assert_eq!(find_basement("()())"), 5);
    }
}
