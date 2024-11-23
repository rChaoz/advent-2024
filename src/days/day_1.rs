use super::DayInfo;
use std::cell::LazyCell;
use std::collections::HashMap;

pub const INFO: DayInfo = DayInfo {
    name: "Trebuchet?!",
    part1,
    part2,

    example1: "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",

    example2: "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
};

const DIGITS: LazyCell<HashMap<&str, u32>> = LazyCell::new(|| {
    HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ])
});

fn part1(input: &str) {
    fn get_simple_digit(str: &str, right: bool) -> u32 {
        if right {
            str.chars().rev().find_map(|c| c.to_digit(10)).unwrap()
        } else {
            str.chars().find_map(|c| c.to_digit(10)).unwrap()
        }
    }

    part(input, get_simple_digit);
}

fn part2(input: &str) {
    fn get_complex_digit(str: &str, right: bool) -> u32 {
        let digit_at_index = |index: usize| -> Option<u32> {
            for &pattern in DIGITS.keys() {
                if pattern.len() > (str.len() - index) {
                    continue;
                }
                if pattern == &str[index..index + pattern.len()] {
                    return Some(*DIGITS.get(pattern).unwrap());
                }
            }
            None
        };
        if right {
            for index in (0..str.len()).rev() {
                if let Some(digit) = digit_at_index(index) {
                    return digit;
                }
            }
        } else {
            for index in 0..str.len() {
                if let Some(digit) = digit_at_index(index) {
                    return digit;
                }
            }
        }
        panic!("digit not found")
    }

    part(input, get_complex_digit);
}

fn part(input: &str, get_digit: fn(&str, bool) -> u32) {
    println!(
        "{}",
        input
            .lines()
            .map(|line| {
                let d1 = get_digit(&line, false);
                let d2 = get_digit(&line, true);
                10 * d1 + d2
            })
            .sum::<u32>()
    );
}
