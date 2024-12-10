use super::DayInfo;
use std::collections::HashSet;

const EXAMPLE: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

pub const INFO: DayInfo = DayInfo {
    name: "Hoof It",
    part1,
    part2,
    example1: EXAMPLE,
    example2: EXAMPLE,
};

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn run<F: FnMut(&Vec<Vec<u32>>, usize, usize) -> u32>(input: &str, mut dfs: F) -> u32 {
    let map: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut score = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            score += dfs(&map, i, j);
        }
    }
    score
}

fn part1(input: &str) {
    fn dfs(
        map: &Vec<Vec<u32>>,
        visited: &mut Vec<Vec<bool>>,
        y: usize,
        x: usize,
        expected: u32,
    ) -> u32 {
        if visited[y][x] || map[y][x] != expected {
            return 0;
        }
        visited[y][x] = true;

        if expected == 9 {
            return 1;
        }
        let mut score = 0;
        for (dx, dy) in DIRECTIONS {
            let x2 = x as i32 + dx;
            let y2 = y as i32 + dy;
            if x2 < 0 || y2 < 0 || x2 >= map[0].len() as i32 || y2 >= map.len() as i32 {
                continue;
            }
            score += dfs(map, visited, y2 as usize, x2 as usize, expected + 1);
        }
        score
    }

    println!(
        "{}",
        run(input, |map, y, x| {
            let mut vis = vec![vec![false; map[0].len()]; map.len()];
            dfs(map, &mut vis, y, x, 0)
        })
    );
}

fn part2(input: &str) {
    fn dfs(
        map: &Vec<Vec<u32>>,
        paths: &mut HashSet<Vec<(usize, usize)>>,
        mut path: Vec<(usize, usize)>,
        y: usize,
        x: usize,
        expected: u32,
    ) {
        if map[y][x] != expected {
            return;
        }
        path.push((x, y));

        if expected == 9 {
            paths.insert(path);
            return;
        }
        for (dx, dy) in DIRECTIONS {
            let x2 = x as i32 + dx;
            let y2 = y as i32 + dy;
            if x2 < 0 || y2 < 0 || x2 >= map[0].len() as i32 || y2 >= map.len() as i32 {
                continue;
            }
            dfs(
                map,
                paths,
                path.clone(),
                y2 as usize,
                x2 as usize,
                expected + 1,
            );
        }
    }

    println!(
        "{}",
        run(input, |map, y, x| {
            let mut paths: HashSet<Vec<(usize, usize)>> = HashSet::new();
            dfs(map, &mut paths, Vec::new(), y, x, 0);
            paths.len() as u32
        })
    );
}
