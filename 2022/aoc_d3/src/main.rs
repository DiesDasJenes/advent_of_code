use std::{fs, ops::Index, collections::HashSet};

type Input<'a> = Vec<(&'a str,&'a str)>; 

static ASCII_LOWER: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

fn parse_input(puzzle_input: &str) -> Input {
    let parsed_input = puzzle_input
    .split('\n')
    .map(|line| line.trim())
    .map(|line| line.split_at(line.len()/2))
    .collect();

    parsed_input
}

fn find_intersection_character(input: &(&str,&str)) -> char {
    let leftside: HashSet<char> = input.0.chars().collect();
    let intersetion_char = input.1.chars().find(|c| leftside.contains(&c)).unwrap();
    // println!("{}", input.0);
    // println!("{}", input.1);
    return intersetion_char;
}

fn calculate_priority_for(input: &(&str,&str)) -> u32 {
    let char: char = find_intersection_character(input);
    let index: u32 = ASCII_LOWER.iter().position(|character| char.to_ascii_lowercase().eq(character)).unwrap().try_into().unwrap();
    
    if char.is_ascii_uppercase() {  
        return index + 27;
    }
    index + 1
}

fn part1(input: &Input) -> u32 {
    let mut total_score: u32 = 0;

    for line in input  {
        total_score += calculate_priority_for(line);
    }
    total_score
}

fn part2(input: &Input) -> u32 {
    let mut total_score: u32 = 0;
    total_score
}


fn main() {
    let puzzle_input = fs::read_to_string("resources/puzzle_input.txt").unwrap();
    let parsed_input = parse_input(&puzzle_input);

    println!("Part 1: {}", part1(&parsed_input));
    println!("Part 2: {}", part2(&parsed_input));
}

#[cfg(test)]
mod test {
    use crate::{part1, part2,parse_input, calculate_priority_for, find_intersection_character};

    #[test]
    fn should_solve_part_1 () {
        let example = "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";
        let parsed_input = parse_input(&example);
        let result = part1(&parsed_input);

        assert_eq!(result,157);
    }

    #[test]
    fn should_return_parsed_input () {
        let example = "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";

        let parsed_example = parse_input(example);

        assert_eq!(parsed_example.len(),6);
        assert_eq!(parsed_example.get(0).unwrap().0,"vJrwpWtwJgWr");
        assert_eq!(parsed_example.get(0).unwrap().1,"hcsFMMfFFhFp");
        assert_eq!(parsed_example.get(1).unwrap().0,"jqHRNqRjqzjGDLGL");
        assert_eq!(parsed_example.get(1).unwrap().1,"rsFMfFZSrLrFZsSL");
    }
    
    #[test]
    fn should_calculate_priority_of_intersection_lowercase () {
        let example =("p","p");

        let result = calculate_priority_for(&example);

        assert_eq!(result,16);
    }

    #[test]
    fn should_calculate_priority_of_intersection_uppercase () {
        let example = ("P","P");

        let result = calculate_priority_for(&example);

        assert_eq!(result,42);
    }

    #[test]
    fn should_find_intersection_in_strings_lowercase () {
        let example = ("vJrwpWtwJgWr","hcsFMMfFFhFp");

        let result = find_intersection_character(&example);

        assert_eq!(result,'p');
    }

    #[test]
    fn should_find_intersection_in_strings_uppercase () {
        let example = ("jqHRNqRjqzjGDLGL","rsFMfFZSrLrFZsSL");

        let result = find_intersection_character(&example);

        assert_eq!(result,'L');
    }
}
