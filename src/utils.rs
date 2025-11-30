use std::fs;

pub type AoCSolution = (String, String);

pub fn load_input(day_num: u8) -> String {
    let day_str = format!("{:02}", day_num);
    let path = format!("src/day{}/input.txt", day_str);
    fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Failed to read input file: {}", path))
}