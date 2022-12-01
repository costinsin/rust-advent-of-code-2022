use clap::Parser;

pub mod solutions;

#[macro_export]
macro_rules! run_day {
    ($module:path, $input_path:expr) => {{
        use $module::*;
        let input = fs::read_to_string($input_path).expect("Sa");

        println!("🎄 Part 1 🎄");
        part1(&input)?;

        println!("🎄 Part 2 🎄");
        part2(&input)?;
    }};
}

#[derive(Parser)]
pub struct Cli {
    pub day: i32,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
