use super::DayInfo;
use regex::Regex;

pub const INFO: DayInfo = DayInfo {
    name: "Mull It Over",
    part1,
    part2,

    example1: "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
    example2: "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
};

fn solve(input: &str, ignore_conditionals: bool) -> u32 {
    let mut sum: u32 = 0;
    let mut enabled = true;
    for c in Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)")
        .unwrap()
        .captures_iter(input)
    {
        match &c[0] {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ if ignore_conditionals || enabled => {
                let mut nums = c
                    .iter()
                    .skip(1)
                    .map(|m| m.unwrap().as_str().parse::<u32>().unwrap());
                sum += nums.next().unwrap() * nums.next().unwrap();
            }
            _ => (),
        }
    }
    sum
}

fn part1(input: &str) {
    println!("{}", solve(input, true));
}

fn part2(input: &str) {
    println!("{}", solve(input, false));
}
