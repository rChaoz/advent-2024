use super::DayInfo;
use std::collections::{HashMap, HashSet};

pub const INFO: DayInfo = DayInfo {
    name: "Monkey Market",
    part1,
    part2,

    example1: "\
1
10
100
2024",

    example2: "\
1
2
3
2024",
};

const STEPS: u32 = 2000;

fn parse<'a>(input: &'a str) -> impl Iterator<Item = u64> + 'a {
    input.lines().map(|s| s.parse().unwrap())
}

fn next(mut secret: u64) -> u64 {
    // Helpers
    macro_rules! mix_prune {
        ($num:expr) => {
            secret = (secret ^ $num) % 16777216
        };
    }

    mix_prune!(secret * 64);
    mix_prune!(secret / 32);
    mix_prune!(secret * 2048);

    secret
}

fn part1(input: &str) {
    println!(
        "{}",
        parse(input)
            .map(|mut num| {
                for _ in 0..STEPS {
                    num = next(num);
                }
                num
            })
            .sum::<u64>()
    )
}

fn part2(input: &str) {
    // Create map of diff-sequence to price locked for each num
    let maps = parse(input)
        .map(|mut num| {
            // Init first 3 changes
            let mut seq = [0i8; 4];
            for i in 0..3 {
                let new = next(num);
                seq[i] = (new % 10) as i8 - (num % 10) as i8;
                num = new;
            }
            // Remember the first value for each possible sequence
            let mut map: HashMap<[i8; 4], u8> = HashMap::new();
            for _ in 0..(STEPS - 3) {
                let new = next(num);
                let digit = (new % 10) as u8;
                seq[3] = digit as i8 - (num % 10) as i8;
                map.entry(seq).or_insert(digit);
                // Shift the sequence to the left and move to the next number
                seq[0] = seq[1];
                seq[1] = seq[2];
                seq[2] = seq[3];
                num = new;
            }
            map
        })
        .collect::<Vec<_>>();
    // Collect all possible sequences
    let mut sequences: HashSet<[i8; 4]> = HashSet::new();
    for map in &maps {
        sequences.extend(map.keys());
    }
    // Find the total score for every possible sequence
    let mut best_total = 0;
    for seq in sequences {
        let total: u32 = maps
            .iter()
            .map(|map| map.get(&seq).copied().unwrap_or_default() as u32)
            .sum();
        if total > best_total {
            best_total = total;
        }
    }
    println!("{best_total}");
}
