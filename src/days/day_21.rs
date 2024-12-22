use super::DayInfo;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Display, Formatter};
use std::sync::LazyLock;

const EXAMPLE: &str = "\
029A
980A
179A
456A
379A";

pub const INFO: DayInfo = DayInfo {
    name: "Keypad Conundrum",
    part1,
    part2,
    example1: EXAMPLE,
    example2: EXAMPLE,
};

const NUMPAD: &str = "\
XXXXX
X789X
X456X
X123X
XX0AX
XXXXX";

const DIRPAD: &str = "\
XXXXX
XX^AX
X<v>X
XXXXX";

fn calc_paths(keypad: &str) -> HashMap<(u8, u8), Vec<String>> {
    let keypad = keypad
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    let mut paths = HashMap::new();
    for x in 0..keypad[0].len() {
        for y in 0..keypad.len() {
            if keypad[y][x] == b'X' {
                continue;
            }
            // bfs
            let mut queue: VecDeque<(usize, usize, Vec<u8>)> = VecDeque::new();
            queue.push_back((x, y, vec![]));
            let mut min_paths: Vec<Vec<(usize, Vec<String>)>> =
                vec![vec![(usize::MAX, Vec::new()); keypad[0].len()]; keypad.len()];
            min_paths[y][x] = (0, vec![String::from("A")]);
            while let Some((x, y, path)) = queue.pop_front() {
                'outer: for (dx, dy, c) in
                    [(0, -1, b'^'), (1, 0, b'>'), (0, 1, b'v'), (-1, 0, b'<')]
                {
                    let nx = (x as i32 + dx) as usize;
                    let ny = (y as i32 + dy) as usize;
                    if keypad[ny][nx] == b'X' {
                        continue;
                    }
                    // don't do >v> as it's always slower than >>v, v>>
                    {
                        let mut index = path.len();
                        while index > 0 && path[index - 1] == c {
                            index -= 1;
                        }
                        while index > 0 {
                            if path[index - 1] == c {
                                continue 'outer;
                            }
                            index -= 1;
                        }
                    };
                    let distance = path.len() + 1;
                    let (existing_distance, vec) = &mut min_paths[ny][nx];
                    if distance > *existing_distance {
                        continue;
                    } else if distance < *existing_distance {
                        *existing_distance = distance;
                        // as we use BFS, it should never reach the same point with a shorter path
                        assert_eq!(vec.len(), 0);
                    }
                    let mut path = path.clone();
                    path.push(c);
                    queue.push_back((nx, ny, path.clone()));
                    path.push(b'A');
                    vec.push(String::from_utf8(path).unwrap());
                }
            }
            // save paths
            min_paths.into_iter().enumerate().for_each(|(ny, row)| {
                row.into_iter()
                    .enumerate()
                    .for_each(|(nx, (distance, new_paths))| {
                        if distance == usize::MAX {
                            return;
                        }
                        paths.insert((keypad[y][x], keypad[ny][nx]), new_paths);
                    });
            });
        }
    }
    paths
}

static NUMPAD_PATHS: LazyLock<HashMap<(u8, u8), Vec<String>>> =
    LazyLock::new(|| calc_paths(NUMPAD));
static DIRPAD_PATHS: LazyLock<HashMap<(u8, u8), Vec<String>>> =
    LazyLock::new(|| calc_paths(DIRPAD));

#[derive(Debug, Clone)]
enum Code {
    // actual code, whether it's initial code (for numpad)
    Exact(String, bool),
    // any of the codes
    Any(Vec<Code>),
    // sequence of codes
    Seq(Vec<Code>),
}

impl Code {
    fn extract_numeric(code: &str) -> usize {
        code[0..code.len() - 1].parse().unwrap()
    }

    fn shortest_len(&self) -> usize {
        match self {
            Code::Exact(str, _) => str.len(),
            Code::Any(codes) => codes.iter().map(Code::shortest_len).min().unwrap(),
            Code::Seq(codes) => codes.iter().map(Code::shortest_len).sum(),
        }
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Code::Exact(str, _) => f.write_str(str),
            Code::Any(codes) => {
                write!(f, "( ")?;
                let mut iter = codes.iter();
                if let Some(code) = iter.next() {
                    write!(f, "{}", code)?;
                }
                for code in iter {
                    write!(f, " | {}", code)?;
                }
                write!(f, " )")
            }
            Code::Seq(codes) => {
                let mut iter = codes.iter();
                if let Some(code) = iter.next() {
                    write!(f, "{}", code)?;
                }
                for code in iter {
                    write!(f, " {}", code)?;
                }
                Ok(())
            }
        }
    }
}

fn paths_to_codes(paths: &HashMap<(u8, u8), Vec<String>>) -> HashMap<(u8, u8), Code> {
    paths
        .iter()
        .map(|(&xy, paths)| {
            (
                xy,
                match paths.len() {
                    0 => panic!("no paths found"),
                    1 => Code::Exact(paths[0].clone(), false),
                    _ => Code::Any(
                        paths
                            .iter()
                            .map(|s| Code::Exact(s.to_owned(), false))
                            .collect(),
                    ),
                },
            )
        })
        .collect()
}

static NUMPAD_CODES: LazyLock<HashMap<(u8, u8), Code>> =
    LazyLock::new(|| paths_to_codes(&*NUMPAD_PATHS));
static DIRPAD_CODES: LazyLock<HashMap<(u8, u8), Code>> =
    LazyLock::new(|| paths_to_codes(&*DIRPAD_PATHS));

fn part1(input: &str) {
    fn directions(code: &Code) -> Code {
        match code {
            Code::Exact(str, numpad) => {
                let mut current = b'A';
                let codes_src = if *numpad {
                    &*NUMPAD_CODES
                } else {
                    &*DIRPAD_CODES
                };
                let vec: Vec<Code> = str
                    .bytes()
                    .map(|char| {
                        let path = codes_src[&(current, char)].clone();
                        current = char;
                        path
                    })
                    .collect();
                if vec.len() == 1 {
                    vec.into_iter().next().unwrap()
                } else {
                    Code::Seq(vec)
                }
            }
            Code::Any(codes) => Code::Any(codes.iter().map(directions).collect()),
            Code::Seq(codes) => Code::Seq(codes.iter().map(directions).collect()),
        }
    }

    println!(
        "{}",
        input
            .lines()
            .map(|door_code| {
                let robot1_code = directions(&Code::Exact(door_code.to_owned(), true));
                let robot2_code = directions(&robot1_code);
                let robot3_code = directions(&robot2_code);
                Code::extract_numeric(door_code) * robot3_code.shortest_len()
            })
            .sum::<usize>()
    );
}

fn part2(input: &str) {
    fn entry_len(entry: &HashMap<&str, u64>) -> u64 {
        entry
            .iter()
            .map(|(&path, &count)| path.len() as u64 * count)
            .sum()
    }

    fn directions(paths: HashMap<&str, u64>, numpad: bool) -> Vec<HashMap<&str, u64>> {
        let paths_src = if numpad {
            &*NUMPAD_PATHS
        } else {
            &*DIRPAD_PATHS
        };

        let mut base_map: HashMap<&str, u64> = HashMap::new();
        let mut options_map: HashMap<&'static Vec<String>, u64> = HashMap::new();
        for (code, count) in paths {
            let mut current = b'A';
            for char in code.bytes() {
                let paths = &paths_src[&(current, char)];
                match paths.len() {
                    0 => panic!("no paths found for {}, {}", current as char, char as char),
                    1 => *base_map.entry(&paths[0]).or_default() += count,
                    _ => *options_map.entry(paths).or_default() += count,
                };
                current = char;
            }
        }

        if options_map.is_empty() {
            return vec![base_map];
        }

        // Split option entries into multiple base maps
        fn split_entries<'a>(
            map: HashMap<&'a str, u64>,
            entries: &[(&'a Vec<String>, u64)],
        ) -> Vec<HashMap<&'a str, u64>> {
            if let Some(&(paths, count)) = entries.first() {
                paths
                    .iter()
                    .flat_map(|path| {
                        let mut map = map.clone();
                        *map.entry(path).or_default() += count;
                        split_entries(map, &entries[1..]).into_iter()
                    })
                    .collect()
            } else {
                vec![map]
            }
        }

        split_entries(base_map, &options_map.into_iter().collect::<Vec<_>>())
    }

    fn initial_directions(code: &str) -> Vec<HashMap<&str, u64>> {
        directions(HashMap::from([(code, 1)]), true)
    }

    fn next_directions(path_options: Vec<HashMap<&str, u64>>) -> Vec<HashMap<&str, u64>> {
        let entries = path_options
            .into_iter()
            .flat_map(|paths| directions(paths, false))
            .collect::<Vec<_>>();
        let min_len: u64 = entries.iter().map(entry_len).min().unwrap();
        entries
            .into_iter()
            .filter(|entry| entry_len(entry) == min_len)
            .collect()
    }

    println!(
        "{}",
        input
            .lines()
            .map(|door_code| {
                let mut directions = initial_directions(door_code);
                for _ in 0..25 {
                    directions = next_directions(directions);
                }
                Code::extract_numeric(door_code) as u64 * entry_len(&directions[0])
            })
            .sum::<u64>()
    );
}
