use std::str::FromStr;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Gift {
    l: i32,
    w: i32,
    h: i32,
}

impl Gift {
    fn new(l: i32, w: i32, h: i32) -> Self {
        Self { l, w, h }
    }
}

impl FromStr for Gift {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        || -> Option<Self> {
            let mut it = s.split("x");
            Some(Gift::new(
                it.next()?.parse().ok()?,
                it.next()?.parse().ok()?,
                it.next()?.parse().ok()?,
            ))
        }()
        .ok_or(())
    }
}

fn cuboid_surface(Gift { l, w, h }: Gift) -> i32 {
    2 * l * w + 2 * l * h + 2 * w * h
}

fn cuboid_volume(Gift { l, w, h }: Gift) -> i32 {
    l * w * h
}

fn slack(Gift { l, w, h }: Gift) -> i32 {
    [l * w, l * h, w * h].into_iter().min().unwrap_or(0)
}

fn required_wrapping(gift: Gift) -> i32 {
    cuboid_surface(gift) + slack(gift)
}

fn shortest_circumference(Gift { l, w, h }: Gift) -> i32 {
    [2 * (l + w), 2 * (l + h), 2 * (w + h)]
        .into_iter()
        .min()
        .unwrap_or(0)
}

fn required_ribbon(gift: Gift) -> i32 {
    shortest_circumference(gift) + cuboid_volume(gift)
}

fn main() {
    let input = include_str!("input.txt");
    let gifts: Vec<Gift> = input.lines().map(str::parse).map(Result::unwrap).collect();
    let wrapping: i32 = gifts.iter().copied().map(required_wrapping).sum();
    println!("Part1: {wrapping}");
    let ribbon: i32 = gifts.iter().copied().map(required_ribbon).sum();
    println!("Part2: {ribbon}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!("1x2x3".parse(), Ok(Gift::new(1, 2, 3)));
        assert_eq!("1x2x3x4".parse(), Ok(Gift::new(1, 2, 3)));
        assert_eq!("1x2".parse(), Err::<Gift, ()>(()));
        assert_eq!("xxx".parse(), Err::<Gift, ()>(()));
        assert_eq!("".parse(), Err::<Gift, ()>(()));
    }

    #[test]
    fn test_cuboid_surface() {
        assert_eq!(cuboid_surface(Gift::new(1, 1, 1)), 6);
        assert_eq!(cuboid_surface(Gift::new(1, 1, 2)), 10);
        assert_eq!(cuboid_surface(Gift::new(2, 2, 2)), 24);
    }

    #[test]
    fn test_paper() {
        assert_eq!(required_wrapping("2x3x4".parse().unwrap()), 58);
        assert_eq!(required_wrapping("1x1x10".parse().unwrap()), 43);
    }

    #[test]
    fn test_shortest_circumference() {
        assert_eq!(shortest_circumference("2x3x4".parse().unwrap()), 10);
        assert_eq!(shortest_circumference("1x1x10".parse().unwrap()), 4);
    }
}
