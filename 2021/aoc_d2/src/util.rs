use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn get_input_lines_with_buffer() -> Vec<String> {
    let mut vec = Vec::new();
    if let Ok(lines) = read_lines("./puzzle_input.txt") {
        vec = lines.map(|line| line.unwrap()).collect::<Vec<String>>();
    }
    return vec;
}

pub fn get_input_lines_without_buffer() -> Vec<String> {
    read_lines_from_str("./puzzle_input.txt")
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_lines_from_str<P>(filepath: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let input = fs::read_to_string(filepath).unwrap();
    input.lines().map(|line| line.to_string()).collect()
}
