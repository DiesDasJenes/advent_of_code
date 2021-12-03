use std::fs;
use std::path::Path;

pub fn get_input_lines() -> Vec<String> {
    read_lines_from_str("./puzzle_input.txt")
}

fn read_lines_from_str<P>(filepath: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let input = fs::read_to_string(filepath).unwrap();
    input.lines().map(|line| line.to_string()).collect()
}
