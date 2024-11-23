use super::DayInfo;
use regex::Regex;
use std::sync::LazyLock;

const EXAMPLE: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

pub const INFO: DayInfo = DayInfo {
    name: "Cube Conundrum",
    part1,
    part2,
    example1: EXAMPLE,
    example2: EXAMPLE,
};

const REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"Game (\d+): (.*)").unwrap());

#[derive(Debug)]
struct Game {
    id: u32,
    reveals: Vec<Reveal>,
}

#[derive(Debug)]
struct Reveal {
    red: u32,
    green: u32,
    blue: u32,
}

impl Game {
    fn power(&self) -> u64 {
        let red = self.reveals.iter().map(|r| r.red).max().unwrap() as u64;
        let green = self.reveals.iter().map(|r| r.green).max().unwrap() as u64;
        let blue = self.reveals.iter().map(|r| r.blue).max().unwrap() as u64;
        let power = red * green * blue;
        power
    }
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let groups = REGEX.captures(value).unwrap();
        let id = groups.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let reveals = groups
            .get(2)
            .unwrap()
            .as_str()
            .split("; ")
            .map(|reveal| Reveal::from(reveal))
            .collect();
        Game { id, reveals }
    }
}

impl From<&str> for Reveal {
    fn from(value: &str) -> Self {
        let mut reveal = Reveal {
            red: 0,
            green: 0,
            blue: 0,
        };
        value.split(", ").for_each(|item| {
            let mut item = item.split(' ');
            let count = item.next().unwrap().parse::<u32>().unwrap();
            let color = item.next().unwrap();
            assert_eq!(item.next(), None);
            match color {
                "red" => reveal.red = count,
                "green" => reveal.green = count,
                "blue" => reveal.blue = count,
                _ => panic!("invalid color: {}", color),
            }
        });
        reveal
    }
}

fn part1(input: &str) {
    println!(
        "{:?}",
        input
            .lines()
            .map(Game::from)
            .filter(|game| game
                .reveals
                .iter()
                .all(|reveal| reveal.red <= 12 && reveal.green <= 13 && reveal.blue <= 14))
            .map(|game| game.id)
            .sum::<u32>()
    );
}

fn part2(input: &str) {
    println!(
        "{:?}",
        input
            .lines()
            .map(|line| Game::from(line).power())
            .sum::<u64>()
    );
}
