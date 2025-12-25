use indicatif::ProgressBar;

fn present_count(house: u64) -> u64 {
    (1..=house)
        .filter(|e| house.is_multiple_of(*e))
        .map(|e| e * 10)
        .sum()
}

fn present_count_2(house: u64) -> u64 {
    (1..=house)
        .filter(|e| house.is_multiple_of(*e) && house <= 50 * *e)
        .map(|e| e * 11)
        .sum()
}

// fn main() {
//     let input = 36000i64;
//     let mut min_house = u64::MAX;
//     for composition in partial_compositions(input / 10, input / 10) {
//         let house = composition.into_iter().map(|c| c as u64).product();
//         min_house = cmp::min(min_house, house);
//     }
//     println!(
//         "Min House: {} with {} presents",
//         min_house,
//         present_count(min_house)
//     );
// }

fn part1() {
    let input = 36000000u64;
    let bar = ProgressBar::new(input);
    let (house, presents) = (1..)
        .map(|house| (house, present_count(house)))
        .map(|v @ (_, p)| {
            if p > bar.position() {
                bar.set_position(p);
            }
            v
        })
        .find(|(_, p)| *p >= input)
        .unwrap();
    bar.finish();
    println!("Part1: {} gets {}", house, presents);
}

fn part2() {
    let input = 36000000u64;
    let bar = ProgressBar::new(input);
    let (house, presents) = (1..)
        .map(|house| (house, present_count_2(house)))
        .map(|v @ (_, p)| {
            if p > bar.position() {
                bar.set_position(p);
            }
            v
        })
        .find(|(_, p)| *p >= input)
        .unwrap();
    bar.finish();
    println!("Part2: {} gets {}", house, presents);
}

fn main() {
    part1();
    part2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_present_count() {
        assert_eq!(present_count(1), 10);
        assert_eq!(present_count(2), 30);
        assert_eq!(present_count(3), 40);
        assert_eq!(present_count(4), 70);
        assert_eq!(present_count(5), 60);
        assert_eq!(present_count(6), 10 + 20 + 30 + 60);
        assert_eq!(present_count(7), 10 + 70);
        assert_eq!(present_count(8), 10 + 20 + 40 + 80);
        assert_eq!(present_count(9), 10 + 30 + 90);
    }
}
