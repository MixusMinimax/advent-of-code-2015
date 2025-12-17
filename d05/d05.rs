fn contains_3vowels(s: &str) -> bool {
    let mut vowel_count = 0;
    for c in s.chars() {
        if "aeiou".contains(c) {
            vowel_count += 1;
        }
        if vowel_count == 3 {
            return true;
        }
    }
    false
}

fn contains_repeating(s: &str) -> bool {
    if s.len() < 2 {
        return false;
    }
    let mut prev = s.chars().next().unwrap();
    for c in s.chars().skip(1) {
        if c == prev {
            return true;
        }
        prev = c;
    }
    false
}

fn contains_mean(s: &str) -> bool {
    s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy")
}

fn contains_repeating_pair(s: &str) -> bool {
    s.len() >= 4 && (0..(s.len() - 2)).any(|i| s[(i + 2)..].contains(&s[i..(i + 2)]))
}

fn contains_separated_repeat(s: &str) -> bool {
    s.len() >= 3 && (0..(s.len() - 2)).any(|i| s[i..i + 1] == s[i + 2..i + 3])
}

fn is_nice1(s: &str) -> bool {
    contains_3vowels(s) && contains_repeating(s) && !contains_mean(s)
}

fn is_nice2(s: &str) -> bool {
    contains_repeating_pair(s) && contains_separated_repeat(s)
}

fn main() {
    let input = include_str!("input.txt");
    let count = input.lines().filter(|l| is_nice1(l)).count();
    println!("Part1: {count}");
    let count = input.lines().filter(|l| is_nice2(l)).count();
    println!("Part2: {count}");
}

#[cfg(test)]
mod tests {
    use super::*;

    //noinspection SpellCheckingInspection
    #[test]
    fn test_vowel() {
        assert!(contains_3vowels("peenar"));
        assert!(!contains_3vowels("qwrtypsdfghjklzxcvbnm"));
        assert!(contains_3vowels("aaa"));
        assert!(!contains_3vowels("aa"));
        assert!(!contains_3vowels(""));
        assert!(contains_3vowels("zjkladopsikeqasfqqsd"));
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn test_repeating() {
        assert!(contains_repeating("slojnikugbrrdevf"));
        assert!(!contains_repeating("sanjoliugrzsedre"));
        assert!(!contains_repeating("x"));
        assert!(!contains_repeating(""));
        assert!(contains_repeating("aa"));
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn test_mean() {
        assert!(contains_mean("xbdfikaboj"));
        assert!(contains_mean("xyageruij"));
        assert!(contains_mean("anjoiglukdsrevcdagjahui"));
        assert!(!contains_mean("asgeurhjio"));
        assert!(contains_mean("ab"));
        assert!(contains_mean("cd"));
        assert!(contains_mean("pq"));
        assert!(contains_mean("xy"));
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn test_repeating_pair() {
        assert!(contains_repeating_pair("aaaa"));
        assert!(!contains_repeating_pair("aaab"));
        assert!(contains_repeating_pair("xababy"));
        assert!(contains_repeating_pair("xyxy"));
        assert!(contains_repeating_pair("aabcdefgaa"));
        assert!(!contains_repeating_pair(""));
        assert!(!contains_repeating_pair("a"));
        assert!(!contains_repeating_pair("aa"));
        assert!(!contains_repeating_pair("aaa"));
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn test_separated_repeat() {
        assert!(contains_separated_repeat("xyx"));
        assert!(contains_separated_repeat("abcdefeghi"));
        assert!(contains_separated_repeat("aaa"));
        assert!(!contains_separated_repeat(""));
        assert!(!contains_separated_repeat("a"));
        assert!(!contains_separated_repeat("aa"));
        assert!(!contains_separated_repeat("aabbcc"));
        assert!(!contains_separated_repeat("ajopigwer"));
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn test_nice1() {
        assert!(is_nice1("ugknbfddgicrmopn"));
        assert!(is_nice1("aaa"));
        assert!(!is_nice1("jchzalrnumimnmhp"));
        assert!(!is_nice1("haegwjzuvuyypxyu"));
        assert!(!is_nice1("dvszwmarrgswjxmb"));
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn test_nice2() {
        assert!(is_nice2("qjhvhtzxzqqjkmpb"));
        assert!(is_nice2("xxyxx"));
        assert!(!is_nice2("uurcxstgmygtbstg"));
        assert!(!is_nice2("ieodomkazucvgmuy"));
    }
}
