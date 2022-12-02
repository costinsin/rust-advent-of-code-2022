use advent_of_code_2022::{parse_args, run_day, solutions};
use solutions::*;
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let args = parse_args();
    let path = format!("src/inputs/day{}.in", args.day);

    match args.day {
        1 => run_day!(day1, path),
        2 => run_day!(day2, path),
        _ => println!("Not found"),
    }

    Ok(())
}
