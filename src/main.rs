use std::env;
use crate::utils::AoCSolution;

mod utils;
mod day01;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: u8 = args
        .get(1)
        .expect("Please provide a day number (1-25) as a command-line argument, e.g., 'cargo run 1'")
        .parse()
        .expect("Day number must be an integer.");

    println!("--- Advent of Code 2025 | Day {:02} ---", day);

    let solve_fn: fn() -> AoCSolution = match day {
        1 => day01::solve,
        _ => {
            println!("Solution for Day {:02} not yet implemented.", day);
            return;
        }
    };

    let (part1, part2) = solve_fn();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}