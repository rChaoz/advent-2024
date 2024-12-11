use aoc_2024_rust::*;
use std::io;
use std::io::BufRead;

fn main() {
    println!("Advent of Code 2023: Rust");
    println!("Enter which day/part you would like to run, and in which mode (full/examples).");
    println!("- full mode - run using input/output files");
    println!("- examples mode - run using example input, outputting to stdout");
    println!();
    println!("Format: [<day>] [<mode>]");
    println!("  day - day number (1-25), or \"all\" (default)");
    println!("  mode - one of: f, f1, f2, e, e1, e2 (defaults to f, only f/e allowed if day is \"all\"):");
    println!("    - f1/f2/f - run part(s) 1/2/both in full mode");
    println!("    - e1/e2/e - run part(s) 1/2/both in examples mode");

    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    if line.trim().is_empty() {
        run_all(false);
        return;
    }

    let mut words = line.split(' ');

    let day = match words.next() {
        None | Some("all") => {
            run_all(match words.next().unwrap_or("f") {
                "f" => false,
                "e" => true,
                _ => panic!("invalid mode"),
            });
            return;
        }
        Some(day) => day.parse::<u8>().unwrap(),
    };

    let (part, examples) = match words.next().unwrap_or("f") {
        "f" => (Part::Both, false),
        "f1" => (Part::One, false),
        "f2" => (Part::Two, false),
        "e" => (Part::Both, true),
        "e1" => (Part::One, true),
        "e2" => (Part::Two, true),
        _ => panic!("invalid mode"),
    };

    run_day(day, part, examples);
}
