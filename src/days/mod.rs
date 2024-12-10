mod util;

pub struct DayInfo {
    pub name: &'static str,
    pub part1: fn(&str) -> (),
    pub part2: fn(&str) -> (),
    pub example1: &'static str,
    pub example2: &'static str,
}

pub const DAYS: [DayInfo; 10] = [
    day_1::INFO,
    day_2::INFO,
    day_3::INFO,
    day_4::INFO,
    day_5::INFO,
    day_6::INFO,
    day_7::INFO,
    day_8::INFO,
    day_9::INFO,
    day_10::INFO,
];

mod day_1;
mod day_10;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
