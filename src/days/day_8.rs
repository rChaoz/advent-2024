use super::DayInfo;
use std::collections::{HashMap, HashSet};
use std::ops::Range;

const EXAMPLE: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

pub const INFO: DayInfo = DayInfo {
    name: "...",
    part1,
    part2,
    example1: EXAMPLE,
    example2: EXAMPLE,
};

fn parse(input: &str) -> (Range<i32>, Range<i32>, HashMap<char, Vec<(i32, i32)>>) {
    let mut x_len = 0;
    let mut y_len = 0;
    let mut map = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        x_len = line.len() as i32;
        y_len = y as i32 + 1;
        for (x, char) in line.chars().enumerate() {
            if char != '.' {
                map.entry(char)
                    .or_insert_with(Vec::new)
                    .push((x as i32, y as i32));
            }
        }
    }

    (0..x_len, 0..y_len, map)
}

fn find_anti_nodes(input: &str, resonant: bool) -> usize {
    let mut anti_nodes = HashSet::new();
    let (x_range, y_range, map) = parse(input);
    for (_, antennas) in map {
        // Go through every possible combination of antennas
        for i in 0..antennas.len() - 1 {
            for j in i + 1..antennas.len() {
                // Calculate the two anti-nodes
                let (x1, y1) = antennas[i];
                let (x2, y2) = antennas[j];
                if resonant {
                    anti_nodes.insert((x1, y1));
                    anti_nodes.insert((x2, y2));
                }
                let mut x3 = x1;
                let mut y3 = y1;
                let mut x4 = x2;
                let mut y4 = y2;
                let dx = x1 - x2;
                let dy = y1 - y2;
                loop {
                    x3 += dx;
                    y3 += dy;
                    x4 -= dx;
                    y4 -= dy;
                    // Save in set as we need to find unique locations
                    let has3 = x_range.contains(&x3) && y_range.contains(&y3);
                    let has4 = x_range.contains(&x4) && y_range.contains(&y4);
                    if has3 {
                        anti_nodes.insert((x3, y3));
                    }
                    if has4 {
                        anti_nodes.insert((x4, y4));
                    }
                    if !resonant || (!has3 && !has4) {
                        break;
                    }
                }
            }
        }
    }
    anti_nodes.len()
}

fn part1(input: &str) {
    println!("{}", find_anti_nodes(input, false));
}

fn part2(input: &str) {
    println!("{}", find_anti_nodes(input, true));
}
