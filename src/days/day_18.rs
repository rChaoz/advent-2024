use super::DayInfo;
use crate::api::is_example;
use std::collections::VecDeque;

const EXAMPLE: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

pub const INFO: DayInfo = DayInfo {
    name: "RAM Run",
    part1,
    part2,
    example1: EXAMPLE,
    example2: EXAMPLE,
};

// Return map size and byte count, for current mode
fn prepare() -> (Vec<Vec<bool>>, usize) {
    let (size, byte_count) = if is_example() { (7, 12) } else { (71, 1024) };
    let map = vec![vec![false; size]; size];
    (map, byte_count)
}

fn parse<'a>(input: &'a str) -> impl Iterator<Item = (usize, usize)> + 'a {
    input.lines().map(|line| {
        let mut nums = line.split(',').map(|s| s.parse::<usize>().unwrap());
        (nums.next().unwrap(), nums.next().unwrap())
    })
}

const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn bfs(mut map: Vec<Vec<bool>>) -> Option<u32> {
    let size = map.len();
    let mut queue = VecDeque::new();
    queue.push_back((0, 0, 0));
    while let Some((x, y, moves)) = queue.pop_front() {
        if x == size - 1 && y == size - 1 {
            return Some(moves);
        }
        for dir in DIRECTIONS {
            let (nx, ny) = (x as i32 + dir.0, y as i32 + dir.1);
            if nx >= 0
                && nx < size as i32
                && ny >= 0
                && ny < size as i32
                && !map[ny as usize][nx as usize]
            {
                queue.push_back((nx as usize, ny as usize, moves + 1));
                map[ny as usize][nx as usize] = true;
            }
        }
    }
    None
}

fn part1(input: &str) {
    let (mut map, byte_count) = prepare();
    for (x, y) in parse(input).take(byte_count) {
        map[y][x] = true;
    }
    println!("{}", bfs(map).unwrap());
}

fn part2(input: &str) {
    let (mut map, ..) = prepare();
    for (x, y) in parse(input) {
        map[y][x] = true;
        if bfs(map.clone()).is_none() {
            println!("{x},{y}");
            return;
        }
    }
}
