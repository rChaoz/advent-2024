mod days;

use days::DayInfo;
use days::DAYS;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::RwLock;
use std::time::Instant;

pub mod api;

pub enum Part {
    One = 1,
    Two = 2,
    Both = 3,
}

static IS_EXAMPLE: RwLock<bool> = RwLock::new(false);

/// Runs the solution for the given 1-indexed day number (1-25).
/// * `day` - day number (1-25)
/// * `part` - which part to run (1, 2 or both - 3)
/// * `examples` - whether to run on the example code or full input
pub fn run_day(day: u8, part: Part, examples: bool) {
    if day == 0 {
        panic!("day cannot be 0")
    }
    if let Some(day_info) = DAYS.get(day as usize - 1) {
        println!("\nRunning single day {}: {}\n\n", day, day_info.name);
        if examples {
            run_examples(day_info, part);
        } else {
            run_full(day, day_info, part);
        }
    } else {
        panic!("day {} not found, max day is {}", day, DAYS.len())
    }
}

/// Runs the solution for all days.
/// * `examples` - whether to run on the example code or full input
pub fn run_all(examples: bool) {
    println!("\nRunning all days\n\n");
    for (day, day_info) in DAYS.iter().enumerate() {
        let day = (day + 1) as u8;
        println!("# Day {}: {}", day, day_info.name);
        if examples {
            run_examples(day_info, Part::Both);
        } else {
            run_full(day, day_info, Part::Both);
        }
    }
}

fn run_part<T: FnOnce()>(part: u8, func: T) {
    println!("## Part {part}");
    let time = Instant::now();
    func();
    println!("done in {:?}\n", time.elapsed());
}

fn run_examples(day: &DayInfo, part: Part) {
    *IS_EXAMPLE.write().unwrap() = true;
    match part {
        Part::One => run_part(1, || (day.part1)(day.example1)),
        Part::Two => run_part(2, || (day.part2)(day.example2)),
        Part::Both => {
            run_part(1, || (day.part1)(day.example1));
            run_part(2, || (day.part2)(day.example2));
        }
    }
}

fn run_full(day_num: u8, day: &DayInfo, part: Part) {
    *IS_EXAMPLE.write().unwrap() = false;
    // Files: data/{day}/{input,output-1,output-2}.txt
    let path = Path::new("data");
    // Read input file
    let mut input = String::new();
    File::open(path.join(day_num.to_string() + ".txt"))
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    let input = &input;

    match part {
        Part::One => run_part(1, || (day.part1)(input)),
        Part::Two => run_part(2, || (day.part2)(input)),
        Part::Both => {
            run_part(1, || (day.part1)(input));
            run_part(2, || (day.part2)(input));
        }
    }
}
