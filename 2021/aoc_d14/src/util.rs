use std::fs;
use std::path::Path;

pub fn get_input_lines_without_buffer<P>(file_path: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    read_lines_from_str(file_path)
}

fn read_lines_from_str<P>(filepath: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let input = fs::read_to_string(filepath).unwrap();
    input.lines().map(|line| line.to_string()).collect()
}
