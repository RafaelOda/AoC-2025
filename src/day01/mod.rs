use crate::utils::{load_input, AoCSolution};

pub fn solve() -> AoCSolution {
    
    let input = load_input(1);

    let numbers: Vec<i32> = input
        .lines()
        .filter_map(|line| line.trim().parse::<i32>().ok())
        .collect();

    let part1_result = format!("{}", numbers.len());
    let part2_result = "Result 2".to_string();

    (part1_result, part2_result)
}