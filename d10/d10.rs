use itertools::Itertools;

fn look_say(i: &str) -> String {
    i.chars()
        .chunk_by(|&c| c)
        .into_iter()
        .map(|(k, c)| format!("{}{}", c.count(), k))
        .collect()
}

fn main() {
    let result = (0..40).fold("1113222113".to_string(), |s, _| look_say(&s));
    println!("Part1: {}", result.len());
    let result = (0..10).fold(result, |s, _| look_say(&s));
    println!("Part2: {}", result.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(look_say("1"), "11");
        assert_eq!(look_say("11"), "21");
        assert_eq!(look_say("21"), "1211");
        assert_eq!(look_say("1211"), "111221");
        assert_eq!(look_say("111221"), "312211");
    }
}
