#![feature(iter_map_windows)]

use itertools::Itertools;

fn increment(s: impl Into<Vec<u8>>) -> Vec<u8> {
    let mut s = s.into();
    for c in s.iter_mut().rev() {
        if *c == b'z' {
            *c = b'a';
        } else {
            *c += 1;
            return s;
        }
    }
    s.insert(0, b'a');
    s
}

fn contains_straight(s: impl AsRef<[u8]>) -> bool {
    s.as_ref()
        .iter()
        .copied()
        .tuple_windows()
        .any(|(a, b, c)| b == a + 1 && c == b + 1)
}

fn contains_illegal(s: impl AsRef<[u8]>) -> bool {
    s.as_ref().iter().any(|c| b"iol".contains(c))
}

fn distinct_pair_count(s: impl AsRef<[u8]>) -> usize {
    s.as_ref()
        .iter()
        .copied()
        .map_windows(|&[a, b]| if a == b { Some(a) } else { None })
        .flatten()
        .unique()
        .count()
}

fn password_is_safe(s: impl AsRef<[u8]>) -> bool {
    let s = s.as_ref();
    contains_straight(s) && !contains_illegal(s) && distinct_pair_count(s) >= 2
}

fn next_safe_password(s: impl AsRef<[u8]>) -> Vec<u8> {
    let mut s = s.as_ref().to_vec();
    loop {
        if password_is_safe(&s) {
            return s;
        }
        s = increment(s);
    }
}

fn main() {
    let result: String = String::from_utf8(next_safe_password(b"hxbxwxba")).unwrap();
    println!("Part1: {}", result);
    let result: String = String::from_utf8(next_safe_password(increment(result))).unwrap();
    println!("Part2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment() {
        assert_eq!(increment(b"aaa"), b"aab");
        assert_eq!(increment(b"xyz"), b"xza");
        assert_eq!(increment(b""), b"a");
        assert_eq!(increment(b"z"), b"aa");
        assert_eq!(increment(b"az"), b"ba");
        assert_eq!(increment(b"zz"), b"aaa");
        assert_eq!(increment(b"yz"), b"za");
    }

    #[test]
    fn test_contains_straight() {
        assert!(contains_straight(b"abc"));
        assert!(contains_straight(b"aaxyzaa"));
        assert!(!contains_straight(b""));
        assert!(!contains_straight(b"a"));
        assert!(!contains_straight(b"ab"));
        assert!(!contains_straight(b"aaaaa"));
    }

    #[test]
    fn test_distinct_pair_count() {
        assert_eq!(distinct_pair_count(b"aa"), 1);
        assert_eq!(distinct_pair_count(b"aaa"), 1);
        assert_eq!(distinct_pair_count(b"aaaa"), 1);
        assert_eq!(distinct_pair_count(b"aabb"), 2);
        assert_eq!(distinct_pair_count(b"abcdefghij"), 0);
        assert_eq!(distinct_pair_count(b"abcddeffghiij"), 3);
    }

    #[test]
    fn test_safe_passwords() {
        assert!(!password_is_safe(b"hijklmmn"));
        assert!(!password_is_safe(b"abbceffg"));
        assert!(!password_is_safe(b"abbcegjk"));
        assert!(password_is_safe(b"abcdffaa"));
        assert!(password_is_safe(b"ghjaabcc"));
    }

    #[test]
    fn test_next_safe_password() {
        assert_eq!(next_safe_password(b"abcdefgh"), b"abcdffaa");
        assert_eq!(next_safe_password(b"ghijklmn"), b"ghjaabcc");
    }
}
