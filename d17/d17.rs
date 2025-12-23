use std::str::FromStr;

fn main() {
    let containers = include_str!("input.txt")
        .lines()
        .map(i32::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let working_combinations = || {
        (0..1u64 << containers.len()).filter(|&mask| {
            (0..containers.len())
                .map(|i| if 1 << i & mask != 0 { containers[i] } else { 0 })
                .sum::<i32>()
                == 150
        })
    };

    let count = working_combinations().count();
    println!("Part1: {}", count);

    let min_amount = working_combinations()
        .map(|mask| mask.count_ones())
        .min()
        .unwrap();

    let count = working_combinations()
        .filter(|mask| mask.count_ones() == min_amount)
        .count();
    println!("Part2: {}", count);
}
