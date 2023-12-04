use std::{fs::File, io::{self, BufRead}, collections::{HashMap, HashSet}};

fn read_file(file_path: &str) -> io::Result<Vec<String>> {
    
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    Ok(lines)
}

fn part1(path_to_puzzle_input: &str) -> u32 {
    let mut sum = 0;
    let mut count = 0;
    match read_file(path_to_puzzle_input) {
        Ok(lines) => {
            for line in lines {
                let parts: Vec<&str> = line.split('|').collect();

                let left_numbers: Vec<u32> = parts[0]
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();

                let right_numbers: HashSet<u32> = parts[1]
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();
                

                
                for &num in &left_numbers {
                    if right_numbers.contains(&num) {
                        if count == 0 {
                            count += 1
                        } else {
                            count *=2;
                        }
                    }
                }
                
            sum += count;
            count = 0
            }
            
        }
        Err(err) => eprintln!("Error reading file: {}", err),
    }

    sum
}

fn part2(path_to_puzzle_input: &str) -> u32 {
    let mut sum = 0;
    
    match read_file(path_to_puzzle_input) {
        Ok(lines) => {
            for line in lines {
                
            }
        }
        Err(err) => eprintln!("Error reading file: {}", err),
    }
    
    sum
}

fn main() {
    println!("Part 1: {}", part1("resources/puzzle_input.txt"));
    println!("Part 2: {}", part2("resources/puzzle_input.txt"));
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
