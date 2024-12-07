use super::DayInfo;

const EXAMPLE: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

pub const INFO: DayInfo = DayInfo {
    name: "Guard Gallivant",
    part1,
    part2,
    example1: EXAMPLE,
    example2: EXAMPLE,
};

#[derive(Debug, Clone, Copy)]
struct Direction {
    id: usize,
    x: i32,
    y: i32,
}

const UP: Direction = Direction { id: 0, x: 0, y: -1 };
const RIGHT: Direction = Direction { id: 1, x: 1, y: 0 };
const DOWN: Direction = Direction { id: 2, x: 0, y: 1 };
const LEFT: Direction = Direction { id: 3, x: -1, y: 0 };
const DIRECTIONS: [Direction; 4] = [UP, RIGHT, DOWN, LEFT];

#[derive(Debug, Clone)]
struct Tile {
    wall: bool,
    visited: bool,
    visited_dir: [bool; DIRECTIONS.len()],
}

impl Tile {
    fn new(wall: bool) -> Self {
        Self {
            wall,
            visited: false,
            visited_dir: [false; DIRECTIONS.len()],
        }
    }
}

fn parse(input: &str) -> (i32, i32, Direction, Vec<Vec<Tile>>) {
    let mut x_start = 0;
    let mut y_start = 0;
    let map: Vec<Vec<Tile>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == '^' {
                        x_start = x as i32;
                        y_start = y as i32;
                    }
                    Tile::new(c == '#')
                })
                .collect()
        })
        .collect();
    (x_start, y_start, UP, map)
}

fn run(map: &mut Vec<Vec<Tile>>, mut x: i32, mut y: i32, mut direction: Direction) -> bool {
    let y_range = 0..map.len() as i32;
    let x_range = 0..map[0].len() as i32;

    'outer: loop {
        let current = &mut map[y as usize][x as usize];
        if current.visited_dir[direction.id] {
            break true;
        }
        current.visited = true;
        current.visited_dir[direction.id] = true;

        // Move
        (y, x) = loop {
            let new_y = y + direction.y;
            let new_x = x + direction.x;
            if !x_range.contains(&new_x) || !y_range.contains(&new_y) {
                break 'outer false;
            }
            if !map[new_y as usize][new_x as usize].wall {
                break (new_y, new_x);
            }
            direction = DIRECTIONS[(direction.id + 1) % DIRECTIONS.len()];
        }
    }
}

fn part1(input: &str) {
    let (start_x, start_y, direction, mut map) = parse(input);
    run(&mut map, start_x, start_y, direction);
    let visited = map.iter().flatten().filter(|t| t.visited).count();
    println!("{visited}");
}

fn part2(input: &str) {
    let (start_x, start_y, direction, map) = parse(input);
    let mut count = 0u32;
    for y in 0..map.len() as i32 {
        for x in 0..map[0].len() as i32 {
            if map[y as usize][x as usize].wall || (x == start_x && y == start_y) {
                continue;
            }
            let mut map = map.clone();
            map[y as usize][x as usize].wall = true;
            if run(&mut map, start_x, start_y, direction) {
                count += 1;
            }
        }
    }
    println!("{count}");
}
