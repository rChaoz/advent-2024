use super::DayInfo;
use crate::api::is_example;
use regex::Regex;

const EXAMPLE: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

pub const INFO: DayInfo = DayInfo {
    name: "Restroom Redoubt",
    part1,
    part2,
    example1: EXAMPLE,
    example2: EXAMPLE,
};

#[derive(Debug, Clone)]
struct Robot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}

fn parse(input: &str) -> Vec<Robot> {
    let regex = Regex::new(r"^p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)$").unwrap();
    input
        .lines()
        .map(|line| {
            let captures = regex.captures(line).unwrap();
            let mut nums = captures
                .iter()
                .skip(1)
                .map(|s| s.unwrap().as_str().parse::<i32>().unwrap());
            Robot {
                px: nums.next().unwrap(),
                py: nums.next().unwrap(),
                vx: nums.next().unwrap(),
                vy: nums.next().unwrap(),
            }
        })
        .collect()
}

fn map_range(mid: bool) -> (i32, i32) {
    let max_x = if is_example() { 11 } else { 101 };
    let max_y = if is_example() { 7 } else { 103 };
    if mid {
        (max_x / 2, max_y / 2)
    } else {
        (max_x, max_y)
    }
}

fn move_robots(robots: &mut Vec<Robot>, seconds: i32) {
    let (max_x, max_y) = map_range(false);

    for robot in robots.iter_mut() {
        robot.px = (robot.px + robot.vx * seconds).rem_euclid(max_x);
        robot.py = (robot.py + robot.vy * seconds).rem_euclid(max_y);
    }
}

fn part1(input: &str) {
    let mut robots = parse(input);
    move_robots(&mut robots, 100);
    // 01
    // 23
    let mut quadrants = [0u32; 4];
    let (mid_x, mid_y) = map_range(true);
    for robot in robots {
        let quadrant = if robot.px < mid_x {
            if robot.py < mid_y {
                0
            } else if robot.py > mid_y {
                2
            } else {
                continue;
            }
        } else if robot.px > mid_x {
            if robot.py < mid_y {
                1
            } else if robot.py > mid_y {
                3
            } else {
                continue;
            }
        } else {
            continue;
        };
        quadrants[quadrant] += 1;
    }

    println!("{}", quadrants.into_iter().reduce(|a, b| a * b).unwrap());
}

fn part2(input: &str) {
    let robots = parse(input);
    let (max_x, max_y) = map_range(false);

    for seconds in 0..max_x * max_y {
        let mut robots = robots.clone();
        move_robots(&mut robots, seconds);

        // Render the map
        let mut map = vec![vec![b'.'; max_x as usize]; max_y as usize];
        for robot in robots.iter() {
            map[robot.py as usize][robot.px as usize] = b'#';
        }
        // Super rudimentary Christmas tree check
        let map: Vec<String> = map
            .into_iter()
            .map(|v| String::from_utf8(v).unwrap())
            .collect();
        if map.iter().any(|row| row.contains("##########")) {
            println!("{seconds}");
            break;
        }
    }
}
