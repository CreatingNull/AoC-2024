use std::path::PathBuf;

use clap::{Parser, value_parser};

pub mod day01;

/// Advent of Code Runner CLI
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// Which day to run
    #[arg(short, long, value_parser = value_parser!(u8).range(1..=25))]
    day: u8,

    /// Which part to run
    #[arg(short, long, value_parser = value_parser!(u8).range(1..=2))]
    part: u8,

    /// Path to the input file
    #[arg(
        short = 'f',  // Stop it clashing with day.
        long,
        value_parser = |s: &str| {
            let path = PathBuf::from(s);
            match path.file_name().and_then(|f| f.to_str()) {
                Some(name) if path.exists() && matches!(name, "example.txt" | "data.txt") => Ok(path),
                _ => Err(String::from("Input must be an existing file named example.txt, part1.txt, or part2.txt")),
            }
        }
    )]
    data: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    match (cli.day, cli.part) {
        (1, 1) => println!("🧩 Output: {}", day01::part_one(&cli.data)),
        (1, 2) => println!("🧩 Output: {}", day01::part_two(&cli.data)),
        _ => eprintln!("❌ Day {} Part {} not implemented", cli.day, cli.part),
    }
}
