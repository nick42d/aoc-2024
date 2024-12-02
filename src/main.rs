use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    filename: PathBuf,
    day: u8,
    part: u8,
}

mod day_1;
mod day_2;

fn main() {
    let cli = Cli::parse();
    let input = std::fs::read_to_string(cli.filename).unwrap();
    match (cli.day, cli.part) {
        (1, 1) => day_1::part_1(input),
        (1, 2) => day_1::part_2(input),
        (2, 1) => day_2::part_1(input),
        (2, 2) => day_2::part_2(input),
        (day, part) => println!("Day {day} / part {part} not yet implemented"),
    }
}
