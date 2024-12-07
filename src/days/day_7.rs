use super::DayInfo;

const EXAMPLE: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

pub const INFO: DayInfo = DayInfo {
    name: "Bridge Repair",
    part1,
    part2,
    example1: EXAMPLE,
    example2: EXAMPLE,
};

fn parse_line(line: &str) -> (u64, Vec<u64>) {
    let mut parts = line.split(": ");
    let result = parts.next().unwrap().parse().unwrap();
    let nums = parts
        .next()
        .unwrap()
        .split(' ')
        .map(|v| v.parse().unwrap())
        .collect();
    (result, nums)
}

fn can_solve(result: u64, first: u64, rest: &[u64], allow_concat: bool) -> bool {
    if rest.len() == 0 {
        result == first
    } else {
        can_solve(result, first + rest[0], &rest[1..], allow_concat)
            || can_solve(result, first * rest[0], &rest[1..], allow_concat)
            || if allow_concat {
                can_solve(
                    result,
                    first * 10u64.pow(rest[0].ilog10() + 1) + rest[0],
                    &rest[1..],
                    allow_concat,
                )
            } else {
                false
            }
    }
}

fn run(input: &str, allow_concat: bool) {
    let sum: u64 = input
        .lines()
        .map(parse_line)
        .filter_map(|(result, nums)| {
            if can_solve(result, nums[0], &nums[1..], allow_concat) {
                Some(result)
            } else {
                None
            }
        })
        .sum();
    println!("{sum}");
}

fn part1(input: &str) {
    run(input, false);
}

fn part2(input: &str) {
    run(input, true);
}
