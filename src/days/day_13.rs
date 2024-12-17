use super::DayInfo;
use regex::Regex;

/*
For each step, we name the claw movements for the buttons x1 and y1 (for the first button - A)
and x2 and y2 (for the second button - B). The prize position is X and Y.

For example, the first step is:

    Button A: X+94, Y+34
    Button B: X+22, Y+67
    Prize: X=8400, Y=5400

This results in the following values:

    x1 = 94, y1 = 34  ;  x2 = 22, y2 = 67
    X = 8400, Y = 5400

We need to find the number of button presses for each button to reach the prize position,
we'll name these unknown values b1 and b2, for the two buttons, respectively. To find b1:

b1*x1 + b2*x2 = X | y2
b1*y1 + b2*y2 = Y | x2

b1*x1*y2 + b2*x2*y2 = X*y2
b1*x2*y1 + b2*x2*y2 = Y*x2
--------------------------  -
b1*x1*y2 - b1*x2*y1 = X*y2 - Y*x2
b1 (x1*y2 - x2*y1)  = X*y2 - Y*x2
b1 = (X*y2 - Y*x2) / (x1*y2 - x2*y1)

To find b2, we use b1 in the first equation:

b1*x1 + b2*x2 = X
b2*x2 = X - b1*x1
b2 = (X - b1*x1) / x2
 */

const EXAMPLE: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

pub const INFO: DayInfo = DayInfo {
    name: "Claw Contraption",
    part1,
    part2,
    example1: EXAMPLE,
    example2: EXAMPLE,
};

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Game {
    button_a: Point,
    button_b: Point,
    prize: Point,
}

fn parse(input: &str, extra: bool) -> Vec<Game> {
    let pattern = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();
    let mut s = String::new();
    let mut games = Vec::new();
    let extra: i64 = if extra { 10000000000000 } else { 0 };

    let mut process = |s: &mut String| {
        let captures = pattern.captures(s).unwrap();
        let mut nums = captures
            .iter()
            .skip(1)
            .map(|g| g.unwrap().as_str().parse::<i64>().unwrap());
        games.push(Game {
            button_a: Point {
                x: nums.next().unwrap(),
                y: nums.next().unwrap(),
            },
            button_b: Point {
                x: nums.next().unwrap(),
                y: nums.next().unwrap(),
            },
            prize: Point {
                x: nums.next().unwrap() + extra,
                y: nums.next().unwrap() + extra,
            },
        });
        s.clear();
    };

    for line in input.lines() {
        if line.is_empty() {
            process(&mut s);
        } else {
            s.push_str(line);
            s.push('\n');
        }
    }
    process(&mut s);
    games
}

fn solve(input: &str, extra: bool) {
    let games = parse(input, extra);
    let sum: i64 = games
        .iter()
        .map(|game| {
            let lhs = game.prize.x * game.button_b.y - game.prize.y * game.button_b.x;
            let rhs = game.button_a.x * game.button_b.y - game.button_b.x * game.button_a.y;
            if lhs % rhs == 0 {
                let b1 = lhs / rhs;
                let lhs = game.prize.x - b1 * game.button_a.x;
                let rhs = game.button_b.x;
                if lhs % rhs == 0 {
                    let b2 = lhs / rhs;
                    if b1 >= 0 && b2 >= 0 {
                        b1 * 3 + b2
                    } else {
                        0
                    }
                } else {
                    0
                }
            } else {
                0
            }
        })
        .sum();
    println!("{sum}");
}

fn part1(input: &str) {
    solve(input, false);
}

fn part2(input: &str) {
    solve(input, true);
}
