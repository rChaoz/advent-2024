mod util;

pub struct DayInfo {
    pub name: &'static str,
    pub part1: fn(&str) -> (),
    pub part2: fn(&str) -> (),
    pub example1: &'static str,
    pub example2: &'static str,
}

pub const DAYS: [DayInfo; 0] = [/* day_1::INFO */];

//mod day_1;
