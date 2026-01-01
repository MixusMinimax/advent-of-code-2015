use indicatif::ProgressBar;

fn select(nums: &[u32], mask: u64) -> impl Iterator<Item = u32> {
    nums.iter().enumerate().filter_map(move |(i, v)| {
        if 1u64 << i & mask != 0 {
            Some(*v)
        } else {
            None
        }
    })
}

fn lowest_entanglement(nums: &[u32], four: bool) -> (u64, Vec<u32>) {
    let sum: u32 = nums.iter().sum();
    let n = sum / if four { 4 } else { 3 };
    let mut smallest_first_group_size = nums
        .iter()
        .scan(0u32, |acc, v| {
            *acc += *v;
            Some(*acc)
        })
        .take_while(|&s| s <= n)
        .count() as u32
        + 1;
    let mut smallest_first_group = 0u64;
    let mut lowest_quantum_entanglement = u64::MAX;
    let pb = ProgressBar::new(1 << nums.len());
    for mask1 in 0u64..1 << nums.len() {
        pb.set_position(mask1);
        let first_size = mask1.count_ones();
        if first_size > smallest_first_group_size {
            continue;
        }
        if select(nums, mask1).sum::<u32>() != n {
            continue;
        }
        if if four {
            !(0u64..1 << nums.len())
                .filter(|mask2| mask1 & mask2 == 0)
                .filter(|mask2| select(nums, *mask2).sum::<u32>() == n)
                .any(|mask2| {
                    (0u64..1 << nums.len())
                        .filter(|mask3| mask1 & mask3 == 0 && mask2 & mask3 == 0)
                        .any(|mask3| select(nums, mask3).sum::<u32>() == n)
                })
        } else {
            !(0u64..1 << nums.len())
                .filter(|mask2| mask1 & mask2 == 0)
                .any(|mask2| select(nums, mask2).sum::<u32>() == n)
        } {
            continue;
        }
        let quantum_entanglement = select(nums, mask1)
            .map(|v| v as u64)
            .try_fold(1u64, |acc, v| acc.checked_mul(v))
            .unwrap_or(u64::MAX);
        if first_size == smallest_first_group_size
            && quantum_entanglement >= lowest_quantum_entanglement
        {
            continue;
        }
        smallest_first_group_size = first_size;
        smallest_first_group = mask1;
        lowest_quantum_entanglement = quantum_entanglement;
    }
    pb.finish();
    let solution: Vec<_> = select(nums, smallest_first_group).collect();
    (lowest_quantum_entanglement, solution)
}

fn main() {
    let nums = include_str!("input.txt")
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<u32>, _>>()
        .unwrap();

    let (lowest_quantum_entanglement, solution) = lowest_entanglement(&nums, false);
    println!(
        "lowest QE 3: {} for group {:?}",
        lowest_quantum_entanglement, solution
    );

    let (lowest_quantum_entanglement, solution) = lowest_entanglement(&nums, true);
    println!(
        "lowest QE 4: {} for group {:?}",
        lowest_quantum_entanglement, solution
    );
}
