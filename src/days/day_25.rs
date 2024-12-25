use super::DayInfo;

pub const INFO: DayInfo = DayInfo {
    name: "Code Chronicle",
    part1,
    part2,

    example1: "\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####",

    example2: "\
...",
};

const WIDTH: usize = 5;
const HEIGHT: usize = 7;

fn parse(input: &str) -> (Vec<[i32; WIDTH]>, Vec<[i32; WIDTH]>) {
    let mut lines = input.lines().peekable();
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    while let Some(&first_line) = lines.peek() {
        let is_lock = first_line == "#".repeat(WIDTH);
        let mut new_item = [-1; WIDTH];
        for i in 1..=HEIGHT {
            let line = lines.next().unwrap();
            assert_eq!(line.len(), WIDTH);
            if i == HEIGHT {
                assert_eq!(line, if is_lock { "." } else { "#" }.repeat(WIDTH));
            }
            for (index, char) in line.chars().enumerate() {
                new_item[index] += (char == '#') as i32;
            }
        }
        if is_lock { &mut locks } else { &mut keys }.push(new_item);

        if let Some(blank_line) = lines.next() {
            assert_eq!(blank_line, "");
        }
    }

    (keys, locks)
}

fn part1(input: &str) {
    let (keys, locks) = parse(input);
    let max = HEIGHT as i32 - 2;
    let mut count = 0u32;
    for key in &keys {
        for lock in &locks {
            if (0..WIDTH).all(|i| key[i] + lock[i] <= max) {
                count += 1;
            }
        }
    }
    println!("{count}");
}

fn part2(_: &str) {
    println!("There is no part 2 for day 25 - it's over! Merry Christmas :)")
}
