use super::DayInfo;
use std::collections::{HashMap, HashSet};
use std::ops::Index;

const EXAMPLE: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

pub const INFO: DayInfo = DayInfo {
    name: "Print Queue",
    part1,
    part2,
    example1: EXAMPLE,
    example2: EXAMPLE,
};

fn parse_rules<'a>(input: &'a str) -> (Vec<(u32, u32)>, Box<dyn 'a + Iterator<Item = Vec<u32>>>) {
    let mut lines = input.lines();
    let mut rules: Vec<(u32, u32)> = Vec::new();

    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let mut parts = line.split('|').map(|s| s.parse::<u32>().unwrap());
        rules.push((parts.next().unwrap(), parts.next().unwrap()));
    }

    let updates = lines.map(|line| {
        line.split(',')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<u32>>()
    });

    (rules, Box::new(updates))
}

fn check_update(update: &Vec<u32>, rules: &Vec<(u32, u32)>) -> bool {
    // Map of numbers to their position in the queue
    let map: HashMap<u32, u32> = HashMap::from_iter(update.iter().copied().zip(0..));

    for (first, second) in rules {
        if let Some(first_index) = map.get(first) {
            if let Some(second_index) = map.get(second) {
                if first_index > second_index {
                    return false;
                }
            }
        }
    }
    true
}

fn part1(input: &str) {
    let (rules, updates) = parse_rules(input);

    let count: u32 = updates
        .map(|update| {
            if check_update(&update, &rules) {
                update[update.len() / 2]
            } else {
                0
            }
        })
        .sum();

    println!("{count}");
}

fn part2(input: &str) {
    let (rules, updates) = parse_rules(input);

    let count: u32 = updates
        .filter(|update| !check_update(&update, &rules))
        .map(|mut update| {
            let all_numbers: HashSet<_> = update.iter().copied().collect();
            let mut numbers: HashSet<u32> = HashSet::new();
            let stop = update.len() / 2;
            let mut index = 0;
            loop {
                // Find the next number that can go at the current index
                let next_index = update
                    .iter()
                    .position(|&num| {
                        rules.iter().all(|(first, second)| {
                            *second != num
                                || !all_numbers.contains(&first)
                                || numbers.contains(&first)
                        })
                    })
                    .unwrap();
                if index == stop {
                    break update[next_index];
                }
                index += 1;
                let found_num = update.remove(next_index);
                numbers.insert(found_num);
            }
        })
        .sum();

    println!("{count}");
}
