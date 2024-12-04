mod util;

pub struct DayInfo {
    pub name: &'static str,
    pub part1: fn(&str) -> (),
    pub part2: fn(&str) -> (),
    pub example1: &'static str,
    pub example2: &'static str,
}

pub const DAYS: [DayInfo; 4] = [day_1::INFO, day_2::INFO, day_3::INFO, day_4::INFO];

mod day_1;
mod day_2;
mod day_3;
mod day_4;