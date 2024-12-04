use super::DayInfo;

const EXAMPLE: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

pub const INFO: DayInfo = DayInfo {
    name: "Ceres Search",
    part1,
    part2,
    example1: EXAMPLE,
    example2: EXAMPLE,
};

const WORD: &[u8] = "XMAS".as_bytes();

type Direction = (i32, i32);
#[rustfmt::skip]
const DIRECTIONS: [Direction; 8] = [
    ( 1,  0), // up
    ( 1,  1), // up-right
    ( 0,  1), // right
    (-1,  1), // down-right
    (-1,  0), // down
    (-1, -1), // down-left
    ( 0, -1), // left
    ( 1, -1), // up-left
];

fn part1(input: &str) {
    let map: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();
    let i_range = 0..map.len() as i32;
    let j_range = 0..map[0].len() as i32;
    let mut count = 0u32;
    for i in i_range.clone() {
        for j in j_range.clone() {
            for direction in DIRECTIONS {
                let mut index = 0;
                loop {
                    let i = i + direction.0 * index;
                    let j = j + direction.1 * index;
                    if !i_range.contains(&i)
                        || !j_range.contains(&j)
                        || map[i as usize][j as usize] != WORD[index as usize]
                    {
                        break;
                    }
                    index += 1;
                    if index as usize == WORD.len() {
                        count += 1;
                        break;
                    }
                }
            }
        }
    }

    println!("{count}");
}

fn part2(input: &str) {
    let map: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();
    let mut count = 0u32;
    for i in 1..(map.len() - 1) as i32 {
        for j in 1..(map[0].len() - 1) as i32 {
            if map[i as usize][j as usize] != b'A' {
                continue;
            }
            for dir in 0..2 {
                // dir is / (1) or \ (2)
                // Find a MAS
                let (dia, dja) = DIRECTIONS[dir * 2 + 1];
                let (dib, djb) = DIRECTIONS[dir * 2 + 5];
                let a = map[(i + dia) as usize][(j + dja) as usize];
                let b = map[(i + dib) as usize][(j + djb) as usize];
                if !(a == b'M' && b == b'S') && !(a == b'S' && b == b'M') {
                    continue;
                }
                // Find another MAS at 90 deg angle
                let (dia, dja) = DIRECTIONS[dir * 2 + 3];
                let (dib, djb) = DIRECTIONS[(dir * 2 + 7) % 8];
                let a = map[(i + dia) as usize][(j + dja) as usize];
                let b = map[(i + dib) as usize][(j + djb) as usize];
                if !(a == b'M' && b == b'S') && !(a == b'S' && b == b'M') {
                    continue;
                }
                count += 1;
                break;
            }
        }
    }

    println!("{count}");
}
