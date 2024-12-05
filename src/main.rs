use clap::Parser;
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
mod day_3;
mod day_4;
mod day_5;

fn main() {
    let cli = Cli::parse();
    let input = std::fs::read_to_string(cli.filename).unwrap();
    let time_before = std::time::Instant::now();
    match (cli.day, cli.part) {
        (1, 1) => day_1::part_1(input),
        (1, 2) => day_1::part_2(input),
        (2, 1) => day_2::part_1(input),
        (2, 2) => day_2::part_2(input),
        (3, 1) => day_3::part_1(input),
        (3, 2) => day_3::part_2(input),
        (4, 1) => day_4::part_1(input),
        (4, 2) => day_4::part_2(input),
        (5, 1) => day_5::part_1(input),
        (5, 2) => day_5::part_2(input),
        (day, part) => println!("Day {day} / part {part} not yet implemented"),
    }
    let time_after = std::time::Instant::now();
    let duration = time_after.duration_since(time_before);
    println!(
        "Processed day {}, part {} in {:?}",
        cli.day, cli.part, duration
    );
}
