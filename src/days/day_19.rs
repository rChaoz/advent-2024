use super::DayInfo;

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
    let mut count = 0u64;

    'outer: for pattern in patterns {
        // index and number of ways to reach it
        let mut v = vec![0u64; pattern.len()];
        v[0] = 1;
        for index in 0..v.iter().len() {
            if v[index] == 0 {
                continue;
            }
            let remaining = &pattern[index..];
            for &towel in &towels {
                if remaining.starts_with(towel) {
                    let new_index = index + towel.len();
                    if new_index == pattern.len() {
                        if part2 {
                            count += v[index];
                        } else {
                            count += 1;
                            continue 'outer;
                        }
                    } else if new_index < pattern.len() {
                        v[new_index] += v[index];
                    }
                }
            }
        }
    }

    println!("{count}");
}

fn part1(input: &str) {
    solve(input, false);
}

fn part2(input: &str) {
    solve(input, true);
}
