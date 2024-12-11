use super::DayInfo;
use std::array;
use std::collections::HashMap;

pub const INFO: DayInfo = DayInfo {
    name: "Plutonian Pebbles",
    part1,
    part2,
    example1: "125 17",
    example2: "125 17",
};

fn part1(input: &str) {
    const MAX_STEP: u32 = 25;

    fn calc(num: u64, step: u32) -> u32 {
        if step == MAX_STEP {
            1
        } else if num == 0 {
            calc(1, step + 1)
        } else {
            let digits = num.ilog10() + 1;
            if digits % 2 == 0 {
                let t = 10u64.pow(digits / 2);
                calc(num / t, step + 1) + calc(num % t, step + 1)
            } else {
                calc(num * 2024, step + 1)
            }
        }
    }

    println!(
        "{}",
        input
            .split_whitespace()
            .map(|s| calc(s.parse().unwrap(), 0))
            .sum::<u32>()
    );
}

fn part2(input: &str) {
    const MAX_STEP: usize = 75;

    let mut steps: [HashMap<u64, u128>; MAX_STEP + 1] = array::from_fn(|_| HashMap::new());

    for num in input.split_whitespace().map(|s| s.parse::<u64>().unwrap()) {
        *steps[0].entry(num).or_default() += 1;
    }

    for step in 1..=MAX_STEP {
        let (s1, s2) = steps.split_at_mut(step);
        let current = s2.first_mut().unwrap();
        for (&num, &count) in s1.last().unwrap() {
            if num == 0 {
                *current.entry(1).or_default() += count;
                continue;
            }
            let digits = num.ilog10() + 1;
            if digits % 2 == 0 {
                let t = 10u64.pow(digits / 2);
                *current.entry(num / t).or_default() += count;
                *current.entry(num % t).or_default() += count;
            } else {
                *current.entry(num * 2024).or_default() += count;
            }
        }
    }

    println!("{}", steps.last().unwrap().values().sum::<u128>());
}
