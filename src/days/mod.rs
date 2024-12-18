mod util;

pub struct DayInfo {
    pub name: &'static str,
    pub part1: fn(&str) -> (),
    pub part2: fn(&str) -> (),
    pub example1: &'static str,
    pub example2: &'static str,
}

pub const DAYS: [DayInfo; 16] = [
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
    day_11::INFO,
    day_12::INFO,
    day_13::INFO,
    day_14::INFO,
    day_15::INFO,
    day_16::INFO,
];

mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
