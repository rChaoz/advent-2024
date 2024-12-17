use super::DayInfo;

pub const INFO: DayInfo = DayInfo {
    name: "Warehouse Woes",
    part1,
    part2,

    example1: "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",

    example2: "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
};

mod util {
    // Common
    use std::fmt::{Display, Formatter, Write};

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Direction(i32, i32);

    impl From<char> for Direction {
        fn from(value: char) -> Self {
            match value {
                '^' => DIRECTIONS[0],
                '>' => DIRECTIONS[1],
                'v' => DIRECTIONS[2],
                '<' => DIRECTIONS[3],
                _ => panic!("invalid direction: {}", value),
            }
        }
    }

    impl Display for Direction {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            f.write_char(match self {
                Direction(0, -1) => '^',
                Direction(1, 0) => '>',
                Direction(0, 1) => 'v',
                Direction(-1, 0) => '<',
                _ => panic!("invalid direction: {:?}", self),
            })
        }
    }

    pub const DIRECTIONS: [Direction; 4] = [
        Direction(0, -1), // up
        Direction(1, 0),  // right
        Direction(0, 1),  // down
        Direction(-1, 0), // left
    ];

    // Map commons
    pub trait Map: Display {
        fn move_robot(&mut self, direction: Direction);
        fn calc_gps(&self) -> u32;
    }

    fn map_display<T: Display>(
        f: &mut Formatter,
        robot_x: i32,
        robot_y: i32,
        map: &Vec<Vec<T>>,
    ) -> std::fmt::Result {
        for (y, row) in map.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if x == robot_x as usize && y == robot_y as usize {
                    f.write_char('@')?;
                } else {
                    tile.fmt(f)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }

    fn map_calc_gps<T, F: Fn(&T) -> bool>(map: &Vec<Vec<T>>, matcher: F) -> u32 {
        map.iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, tile)| {
                        if matcher(tile) {
                            100 * y as u32 + x as u32
                        } else {
                            0
                        }
                    })
                    .sum::<u32>()
            })
            .sum()
    }

    // Part 1
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Tile1 {
        Empty,
        Wall,
        /// whether the box is stuck (cannot be moved anymore by robot)
        Box(bool),
    }

    impl Display for Tile1 {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_char(match self {
                Tile1::Empty => '.',
                Tile1::Wall => '#',
                Tile1::Box(false) => 'O',
                Tile1::Box(true) => 'o',
            })
        }
    }

    #[derive(Debug)]
    pub struct Map1 {
        map: Vec<Vec<Tile1>>,
        robot_x: i32,
        robot_y: i32,
    }

    impl Map1 {
        pub fn create(map: Vec<Vec<char>>) -> Map1 {
            let mut robot_x = 0;
            let mut robot_y = 0;
            Map1 {
                map: map
                    .into_iter()
                    .enumerate()
                    .map(|(y, row)| {
                        row.into_iter()
                            .enumerate()
                            .map(|(x, c)| match c {
                                '#' => Tile1::Wall,
                                '.' => Tile1::Empty,
                                'O' => Tile1::Box(false),
                                '@' => {
                                    robot_x = x as i32;
                                    robot_y = y as i32;
                                    Tile1::Empty
                                }
                                _ => panic!("invalid tile: {}", c),
                            })
                            .collect()
                    })
                    .collect(),
                robot_x,
                robot_y,
            }
        }
    }

    impl Map for Map1 {
        fn move_robot(&mut self, direction: Direction) {
            let robot_x = self.robot_x + direction.0;
            let robot_y = self.robot_y + direction.1;
            match self.map[robot_y as usize][robot_x as usize] {
                Tile1::Empty => {
                    self.robot_x = robot_x;
                    self.robot_y = robot_y;
                }
                Tile1::Wall | Tile1::Box(true) => (),
                Tile1::Box(false) => {
                    let mut next_x = robot_x + direction.0;
                    let mut next_y = robot_y + direction.1;
                    while let Tile1::Box(false) = self.map[next_y as usize][next_x as usize] {
                        next_x += direction.0;
                        next_y += direction.1;
                    }
                    if let Tile1::Empty = self.map[next_y as usize][next_x as usize] {
                        self.map[next_y as usize][next_x as usize] = Tile1::Box({
                            // Box is stuck if it has two adjacent non-opposite walls
                            fn is_fixed(tile: Tile1) -> bool {
                                match tile {
                                    Tile1::Wall | Tile1::Box(true) => true,
                                    _ => false,
                                }
                            }

                            (is_fixed(self.map[(next_y - 1) as usize][next_x as usize])
                                || is_fixed(self.map[(next_y + 1) as usize][next_x as usize]))
                                && (is_fixed(self.map[next_y as usize][(next_x - 1) as usize])
                                    || is_fixed(self.map[next_y as usize][(next_x + 1) as usize]))
                        });
                        self.map[robot_y as usize][robot_x as usize] = Tile1::Empty;
                        self.robot_x = robot_x;
                        self.robot_y = robot_y;
                    }
                }
            }
        }

        fn calc_gps(&self) -> u32 {
            map_calc_gps(&self.map, |tile| match tile {
                Tile1::Box(_) => true,
                _ => false,
            })
        }
    }

    impl Display for Map1 {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            map_display(f, self.robot_x, self.robot_y, &self.map)
        }
    }

    // Part 2
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Tile2 {
        Empty,
        Wall,
        BoxLeft,
        BoxRight,
    }

    impl Display for Tile2 {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            f.write_char(match self {
                Tile2::Empty => '.',
                Tile2::Wall => '#',
                Tile2::BoxLeft => '[',
                Tile2::BoxRight => ']',
            })
        }
    }

    #[derive(Debug)]
    pub struct Map2 {
        map: Vec<Vec<Tile2>>,
        robot_x: i32,
        robot_y: i32,
    }

    impl Map2 {
        pub fn create(map: Vec<Vec<char>>) -> Map2 {
            let mut robot_x = 0;
            let mut robot_y = 0;
            Map2 {
                map: map
                    .into_iter()
                    .enumerate()
                    .map(|(y, row)| {
                        row.into_iter()
                            .enumerate()
                            .flat_map(|(x, c)| match c {
                                '#' => [Tile2::Wall; 2],
                                '.' => [Tile2::Empty; 2],
                                'O' => [Tile2::BoxLeft, Tile2::BoxRight],
                                '@' => {
                                    robot_x = x as i32 * 2;
                                    robot_y = y as i32;
                                    [Tile2::Empty; 2]
                                }
                                _ => panic!("invalid tile: {}", c),
                            })
                            .collect()
                    })
                    .collect(),
                robot_x,
                robot_y,
            }
        }
    }

    impl Map for Map2 {
        fn move_robot(&mut self, direction: Direction) {
            //noinspection RsUnnecessaryParentheses
            fn can_move_box(map: &mut Map2, x: i32, y: i32, direction: Direction) -> bool {
                let next_x = x + direction.0;
                let next_y = y + direction.1;
                // Special case - moving to the right
                let move_x = if direction.0 == 1 { next_x + 1 } else { next_x };
                if match map.map[next_y as usize][move_x as usize] {
                    Tile2::Wall => false,
                    Tile2::Empty => true,
                    Tile2::BoxLeft => can_move_box(map, move_x, next_y, direction),
                    Tile2::BoxRight => can_move_box(map, move_x - 1, next_y, direction),
                } {
                    // check right side of box when moving up/down
                    if direction.1 != 0 {
                        match map.map[next_y as usize][(next_x + 1) as usize] {
                            Tile2::Wall => false,
                            Tile2::Empty => true,
                            Tile2::BoxLeft => can_move_box(map, next_x + 1, next_y, direction),
                            Tile2::BoxRight => true, // already checked by BoxLeft in the match earlier
                        }
                    } else {
                        true
                    }
                } else {
                    false
                }
            }

            fn move_box(map: &mut Map2, x: i32, y: i32, direction: Direction) {
                let next_x = x + direction.0;
                let next_y = y + direction.1;
                // Special case - moving to the right
                let move_x = if direction.0 == 1 { next_x + 1 } else { next_x };
                match map.map[next_y as usize][move_x as usize] {
                    Tile2::BoxLeft => move_box(map, move_x, next_y, direction),
                    Tile2::BoxRight => move_box(map, move_x - 1, next_y, direction),
                    _ => (),
                }
                if direction.1 != 0 {
                    match map.map[next_y as usize][(next_x + 1) as usize] {
                        Tile2::BoxLeft => move_box(map, next_x + 1, next_y, direction),
                        _ => (),
                    }
                }
                map.map[y as usize][x as usize] = Tile2::Empty;
                map.map[y as usize][(x + 1) as usize] = Tile2::Empty;
                map.map[next_y as usize][next_x as usize] = Tile2::BoxLeft;
                map.map[next_y as usize][(next_x + 1) as usize] = Tile2::BoxRight;
            }

            let x = self.robot_x + direction.0;
            let y = self.robot_y + direction.1;
            match self.map[y as usize][x as usize] {
                Tile2::Empty => {
                    self.robot_x = x;
                    self.robot_y = y;
                }
                Tile2::Wall => (),
                Tile2::BoxLeft => {
                    if can_move_box(self, x, y, direction) {
                        move_box(self, x, y, direction);
                        self.robot_x = x;
                        self.robot_y = y;
                    }
                }
                Tile2::BoxRight => {
                    if can_move_box(self, x - 1, y, direction) {
                        move_box(self, x - 1, y, direction);
                        self.robot_x = x;
                        self.robot_y = y;
                    }
                }
            }
        }

        fn calc_gps(&self) -> u32 {
            map_calc_gps(&self.map, |tile| match tile {
                Tile2::BoxLeft => true,
                _ => false,
            })
        }
    }

    impl Display for Map2 {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            map_display(f, self.robot_x, self.robot_y, &self.map)
        }
    }
}

use util::*;

fn parse(input: &str, part2: bool) -> (Box<dyn Map>, Vec<Direction>) {
    let mut map = Vec::new();
    let mut moves = Vec::new();
    let mut lines = input.lines();

    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        map.push(line.chars().collect());
    }

    while let Some(line) = lines.next() {
        moves.extend(line.chars().map(Direction::from));
    }

    (
        if part2 {
            Box::new(Map2::create(map))
        } else {
            Box::new(Map1::create(map))
        },
        moves,
    )
}

fn run(input: &str, part2: bool) {
    let (mut map, moves) = parse(input, part2);
    for mv in moves {
        map.move_robot(mv);
    }
    println!("{}", map.calc_gps());
}

fn part1(input: &str) {
    run(input, false);
}

fn part2(input: &str) {
    run(input, true);
}
