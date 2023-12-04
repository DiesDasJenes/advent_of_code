use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn read_file(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(lines)
}

fn process_line(line: &str) -> u32 {
    let parts: Vec<&str> = line.split('|').collect();

    let left_numbers: Vec<u32> = parts[0]
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    let right_numbers: HashSet<u32> = parts[1]
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    let mut count = 0;

    for &num in &left_numbers {
        if right_numbers.contains(&num) {
            if count == 0 {
                count += 1;
            } else {
                count *= 2;
            }
        }
    }

    count
}

fn part1(path_to_puzzle_input: &str) -> u32 {
    match read_file(path_to_puzzle_input) {
        Ok(lines) => lines.into_iter().map(|line| process_line(&line)).sum(),
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            0
        }
    }
}

fn part2(_path_to_puzzle_input: &str) -> u32 {
    // Add your implementation for part 2 here
    // Return the result
    0
}

fn main() {
    let puzzle_input_path = "resources/puzzle_input.txt";
    println!("Part 1: {}", part1(puzzle_input_path));
    // Uncomment the line below when you have implemented part 2
    // println!("Part 2: {}", part2(puzzle_input_path));
}


#[cfg(test)]
mod test {
    
    use rstest::rstest;

    use crate::{part1, part2};

    #[test]
    fn should_return_correct_sum_for_part_1 () {
       
        let sum_of_wins = part1("resources/test_part_1.txt");

        assert_eq!(sum_of_wins, 13);
    }

    // #[test]
    // fn should_return_correct_sum_for_part_2 () {
       
    //     let sum_of_indexes_of_invalid_games = part2("resources/test_part_1.txt");

    //     assert_eq!(sum_of_indexes_of_invalid_games, 2286);
    // }
    

    
}
