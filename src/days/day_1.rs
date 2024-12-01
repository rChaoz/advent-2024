use super::DayInfo;
use std::collections::{BinaryHeap, HashMap};
use std::iter::Map;
use std::str::Lines;

const EXAMPLE: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3";

pub const INFO: DayInfo = DayInfo {
    name: "Historian Hysteria",
    part1,
    part2,
    example1: EXAMPLE,
    example2: EXAMPLE,
};

fn parse(input: &str) -> Map<Lines, fn(&str) -> (u32, u32)> {
    input.lines().map(|line| {
        let mut nums = line.split_whitespace();
        (
            nums.next().unwrap().parse::<u32>().unwrap(),
            nums.next().unwrap().parse::<u32>().unwrap(),
        )
    })
}

fn part1(input: &str) {
    let mut first: BinaryHeap<u32> = BinaryHeap::new();
    let mut second: BinaryHeap<u32> = BinaryHeap::new();
    parse(input).for_each(|(a, b)| {
        first.push(a);
        second.push(b);
    });
    let mut sum: u32 = 0;
    while !first.is_empty() {
        sum += first.pop().unwrap().abs_diff(second.pop().unwrap());
    }
    println!("{sum}")
}

fn part2(input: &str) {
    let mut count: HashMap<u32, u32> = HashMap::new();
    let mut first: Vec<u32> = Vec::new();
    parse(input).for_each(|(a, b)| {
        first.push(a);
        count.entry(b).and_modify(|e| *e += 1).or_insert(1);
    });
    let mut similarity: u64 = 0;
    for num in first {
        similarity += num as u64 * count.get(&num).map(|&n| n).unwrap_or(0) as u64;
    }
    println!("{similarity}")
}
