use super::DayInfo;
use std::iter::Map;
use std::str::Lines;

const EXAMPLE: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

pub const INFO: DayInfo = DayInfo {
    name: "Red-Nosed Reports",
    part1,
    part2,
    example1: EXAMPLE,
    example2: EXAMPLE,
};

fn parse(input: &str) -> Map<Lines, fn(&str) -> Vec<i32>> {
    input.lines().map(|line| {
        line.split_whitespace()
            .map(|word| word.parse::<i32>().unwrap())
            .collect()
    })
}

fn is_safe(nums: &Vec<i32>) -> bool {
    if nums.len() < 2 {
        return true;
    }
    let sign = (nums[1] - nums[0]).signum();
    if sign == 0 {
        return false;
    }
    let mut nums = nums.iter();
    let mut prev = *nums.next().unwrap();
    for &num in nums {
        if (num - prev).signum() != sign || !(1..=3).contains(&num.abs_diff(prev)) {
            return false;
        }
        prev = num;
    }
    true
}

fn part1(input: &str) {
    let count = parse(input)
        .map(|nums| if is_safe(&nums) { 1 } else { 0 })
        .sum::<u32>();
    println!("{count}")
}

fn part2(input: &str) {
    let count = parse(input)
        .map(|nums| {
            if is_safe(&nums) {
                return 1;
            }
            for i in 0..nums.len() {
                let mut nums = nums.clone();
                nums.remove(i);
                if is_safe(&nums) {
                    return 1;
                }
            }
            0
        })
        .sum::<u32>();
    println!("{count}")
}
