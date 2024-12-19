use super::DayInfo;
use std::collections::BTreeMap;

const EXAMPLE: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

pub const INFO: DayInfo = DayInfo {
    name: "Linen Layout",
    part1,
    part2,
    example1: EXAMPLE,
    example2: EXAMPLE,
};

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut lines = input.lines();
    let towels = lines.next().unwrap().split(", ").collect();
    assert_eq!(lines.next().unwrap(), "");
    let patterns = lines.collect();
    (towels, patterns)
}

fn solve(input: &str, part2: bool) {
    let (towels, patterns) = parse(input);
    let mut solved = 0u64;
    let mut arrangements = 0u64;

    for pattern in patterns {
        // index and number of ways to reach it
        let mut map = BTreeMap::new();
        map.insert(0, 1);

        'outer: while let Some((index, ways)) = map.pop_first() {
            let remaining = &pattern[index..];
            for &towel in &towels {
                if remaining.starts_with(towel) {
                    let new_index = index + towel.len();
                    if new_index == pattern.len() {
                        solved += 1;
                        if !part2 {
                            break 'outer;
                        }
                        arrangements += ways;
                    } else {
                        *map.entry(new_index).or_insert(0) += ways;
                    }
                }
            }
        }
    }

    println!("{}", if part2 { arrangements } else { solved });
}

fn part1(input: &str) {
    solve(input, false);
}

fn part2(input: &str) {
    solve(input, true);
}
