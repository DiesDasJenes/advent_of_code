use std::{fs};

type Input<'a> = Vec<&'a str>; 

fn parse_input(puzzle_input: &str) -> Input {
    let parsed_input = puzzle_input
    .split("\n")
    .collect();

    return parsed_input
}

fn part1(input: &Input) -> u32 {
    let mut sum = 0;
    let mut tmp = 0;

    for s in input {
        if s.is_empty() {
            tmp = 0
        } else {
            tmp += s.parse::<u32>().unwrap();
            if tmp >= sum {
                sum = tmp
            }
        }
    }

    return sum;
}

fn part2(input: &Input) -> u32 {
    let mut tmp = 0;
    let mut calories_per_elf: Vec<u32> = Vec::new();

    for (i,s) in input.iter().enumerate() {
        if s.is_empty() {
            calories_per_elf.push(tmp);
            tmp = 0
        } else {
            tmp += s.parse::<u32>().unwrap();
            if i == input.len() - 1 {
               calories_per_elf.push(tmp)
            }
        }
    }
    
    return get_sum_of_top_three_highest_numbers(&calories_per_elf);
}

fn get_sum_of_top_three_highest_numbers(vector: &Vec<u32>) -> u32 {
    let mut unique_numbers = get_unique_number(vector);
    unique_numbers.sort();
    unique_numbers.reverse();

    let highest_numbers: Vec<&u32> = unique_numbers.iter().take(3).collect();

    
    println!("{:?}", highest_numbers);
    highest_numbers.into_iter().sum()
}

fn get_unique_number(vector: &Vec<u32>) -> Vec<u32> {
    let mut unqiue_numbers: Vec<u32> = vec![];

    for element in vector {
        if !unqiue_numbers.contains(&element) {
            unqiue_numbers.push(*element)
        }
    }
    
    println!("{:?}", unqiue_numbers);
    return unqiue_numbers;
}

fn main() {
    let puzzle_input = fs::read_to_string("resources/puzzle_input.txt").unwrap();
    let parsed_input = parse_input(&puzzle_input);
    println!("Part 1: {}", part1(&parsed_input));
    println!("Part 2: {}", part2(&parsed_input));
}

#[cfg(test)]
mod test {
    use crate::{parse_input, part1, part2};

    #[test]
    fn should_return_parsed_input () {
        let example = "1
        1
        
        2
        2";

        let parsed_example = parse_input(&example);

        assert_eq!(parsed_example.len(),5);
    }

    #[test]
    fn should_calculate_highest_count () {
        let strings = vec!["1", "2", "", "3", "", "4", "5"];

        assert_eq!(part1(&strings), 9)
    }

    #[test]
    fn should_calculate_sum_of_top_three () {
        let strings = vec!["1", "2", "", "3", "", "4", "5","","6","7","","3","3","","6","7"];

        assert_eq!(part2(&strings), 28)
    }
}
