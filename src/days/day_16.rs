use super::DayInfo;
use fix_fn::fix_fn;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

const EXAMPLE: &str = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

pub const INFO: DayInfo = DayInfo {
    name: "Reindeer Maze",
    part1,
    part2,

    example1: EXAMPLE,
    example2: EXAMPLE,
};

const START_DIR: usize = 1;
const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

#[derive(Debug, Eq)]
struct Entry {
    x: usize,
    y: usize,
    dir: usize,
    score: u32,
}

impl Entry {
    fn new(x: usize, y: usize, dir: usize, score: u32) -> Self {
        Self { x, y, dir, score }
    }
}

// Reverse order by score
impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.score.partial_cmp(&self.score)
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse(input: &str) -> (usize, usize, usize, usize, Vec<Vec<bool>>) {
    let (mut start_x, mut start_y) = (0, 0);
    let (mut end_x, mut end_y) = (0, 0);
    let vec = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start_x = x;
                        start_y = y;
                    } else if c == 'E' {
                        end_x = x;
                        end_y = y;
                    }
                    c == '#'
                })
                .collect()
        })
        .collect();
    (start_x, start_y, end_x, end_y, vec)
}

fn dijkstra(start_x: usize, start_y: usize, map: &Vec<Vec<bool>>) -> Vec<Vec<[u32; 4]>> {
    // Best scores for each direction
    let mut best = vec![vec![[u32::MAX; 4]; map[0].len()]; map.len()];

    // Dijkstra :3 - queue has x, y, direction, score
    let mut queue = BinaryHeap::new();
    best[start_y][start_x][START_DIR] = 0;
    queue.push(Entry::new(start_x, start_y, START_DIR, 0));

    while let Some(Entry {
        x,
        y,
        dir,
        score: old_score,
    }) = queue.pop()
    {
        let score = best[y][x][dir];
        if old_score != score {
            continue;
        }

        // Try going forward
        let new_x = x as i32 + DIRECTIONS[dir].0;
        let new_y = y as i32 + DIRECTIONS[dir].1;
        if new_x >= 0
            && new_x < map[0].len() as i32
            && new_y >= 0
            && new_y < map.len() as i32
            && !map[new_y as usize][new_x as usize]
        {
            let new_score = score + 1;
            if new_score < best[new_y as usize][new_x as usize][dir] {
                best[new_y as usize][new_x as usize][dir] = new_score;
                queue.push(Entry::new(new_x as usize, new_y as usize, dir, new_score));
            }
        }

        // Or turning
        let new_score = score + 1000;
        for dir in [(dir + 1) % 4, (dir + 3) % 4] {
            if new_score < best[y][x][dir] {
                best[y][x][dir] = new_score;
                queue.push(Entry::new(x, y, dir, new_score));
            }
        }
    }

    best
}

fn part1(input: &str) {
    let (start_x, start_y, end_x, end_y, map) = parse(input);
    let best = dijkstra(start_x, start_y, &map);

    println!("{}", best[end_y][end_x].into_iter().min().unwrap());
}

fn part2(input: &str) {
    let (start_x, start_y, end_x, end_y, map) = parse(input);
    let best = dijkstra(start_x, start_y, &map);
    let mut best_tiles = HashSet::new();

    let reconstruct_path = fix_fn!(|reconstruct_path,
                                    best_tiles: &mut HashSet<(usize, usize)>,
                                    x: usize,
                                    y: usize,
                                    dir: usize,
                                    score: u32|
     -> Option<()> {
        best_tiles.insert((x, y));
        // Stop condition
        if x == start_x && y == start_y {
            return None;
        }
        // Step that led here
        let prev_x = (x as i32 - DIRECTIONS[dir].0) as usize;
        let prev_y = (y as i32 - DIRECTIONS[dir].1) as usize;
        let prev_score = score.checked_sub(1)?;
        if best[prev_y][prev_x][dir] == prev_score {
            reconstruct_path(best_tiles, prev_x, prev_y, dir, prev_score);
        }
        // Rotation that led here
        let prev_score = score.checked_sub(1000)?;
        for prev_dir in [(dir + 1) % 4, (dir + 3) % 4] {
            if best[y][x][prev_dir] == prev_score {
                reconstruct_path(best_tiles, x, y, prev_dir, prev_score);
            }
        }
        None
    });

    let best_score = best[end_y][end_x].into_iter().min().unwrap();
    for dir in 0..DIRECTIONS.len() {
        if best[end_y][end_x][dir] == best_score {
            reconstruct_path(&mut best_tiles, end_x, end_y, dir, best_score);
        }
    }

    println!("{}", best_tiles.len());
}
