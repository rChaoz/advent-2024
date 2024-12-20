use super::DayInfo;
use crate::api::is_example;
use std::collections::VecDeque;

const EXAMPLE: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

pub const INFO: DayInfo = DayInfo {
    name: "Race Condition",
    part1,
    part2,
    example1: EXAMPLE,
    example2: EXAMPLE,
};

type Point = (usize, usize);

const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn parse(input: &str) -> (Point, Point, Vec<Vec<bool>>) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = (x, y);
                    } else if c == 'E' {
                        end = (x, y);
                    }
                    c == '#'
                })
                .collect()
        })
        .collect();
    (start, end, map)
}

fn bfs(map: &Vec<Vec<bool>>, start: Point) -> Vec<Vec<u32>> {
    let mut score = vec![vec![u32::MAX; map[0].len()]; map.len()];
    let mut queue = VecDeque::new();
    score[start.1][start.0] = 0;
    queue.push_back(start);

    while let Some((x, y)) = queue.pop_front() {
        let new_score = score[y][x] + 1;
        for (dx, dy) in DIRECTIONS {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx >= 0 && ny >= 0 {
                let (nx, ny) = (nx as usize, ny as usize);
                if nx < map[0].len() && ny < map.len() && !map[ny][nx] && new_score < score[ny][nx]
                {
                    score[ny][nx] = new_score;
                    queue.push_back((nx, ny));
                }
            }
        }
    }

    score
}

fn solve(input: &str, part2: bool) {
    let (start, _, map) = parse(input);
    let scores = bfs(&map, start);
    // For main solution
    let mut count = 0u32;
    // For examples
    let mut counts = vec![0u32; 100];

    let cheat_distance = if part2 { 20 } else { 2 };
    let min_cheat_value = if is_example() {
        if part2 {
            50
        } else {
            1
        }
    } else {
        100
    };

    for y in 0..scores.len() as i32 {
        for x in 0..scores[0].len() as i32 {
            if scores[y as usize][x as usize] == u32::MAX {
                // wall or unreachable
                continue;
            }
            // Get all accessible locations within the cheat distance
            for ny in y - cheat_distance..=y + cheat_distance {
                let max_x_delta = cheat_distance - (y - ny).abs();
                for nx in x - max_x_delta..=x + max_x_delta {
                    // Check bounds
                    if nx < 0
                        || ny < 0
                        || nx >= scores[0].len() as i32
                        || ny >= scores.len() as i32
                        || map[ny as usize][nx as usize]
                    {
                        continue;
                    }
                    // Check cheat value
                    if let Some(cheat_value) = scores[ny as usize][nx as usize].checked_sub(
                        scores[y as usize][x as usize] + x.abs_diff(nx) + y.abs_diff(ny),
                    ) {
                        if cheat_value >= 100 {
                            count += 1;
                        } else if cheat_value >= min_cheat_value {
                            counts[cheat_value as usize] += 1;
                        }
                    }
                }
            }
        }
    }

    if is_example() {
        for (i, &count) in counts.iter().enumerate() {
            if count == 0 {
                continue;
            }
            if count == 1 {
                println!("There is one cheat that saves {i} picosecond.",);
            } else {
                println!("There are {count} cheats that save {i} picoseconds.",);
            }
        }
    } else {
        println!("{count}");
    }
}

fn part1(input: &str) {
    solve(input, false);
}

fn part2(input: &str) {
    solve(input, true);
}
