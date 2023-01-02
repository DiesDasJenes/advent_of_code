use std::{fs, collections::HashSet};

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
    let intersetion_char = input.1.chars().find(|c| leftside.contains(c)).unwrap();
    
    intersetion_char
}

fn find_intersection_character_in_list(input: Vec<String>) -> char {
    let first_line: HashSet<char> = input.get(0).unwrap().chars().collect();
    let mut intersection_chars: Vec<Vec<char>>= Vec::new();

    for (index, line) in input.iter().enumerate() {
        if index != 0 {
        let chars: Vec<char> = line.chars().into_iter().filter(|c| first_line.contains(c)).collect();
        intersection_chars.push(chars);
        }
    }
    
    let intersection_char = intersection_chars.get(0).unwrap().iter().find(|c| intersection_chars.get(1).unwrap().contains(c)).unwrap();
    intersection_char.to_owned()
}

fn calculate_priority_for(char: char) -> u32 {
    let index: u32 = ASCII_LOWER.iter().position(|character| char.to_ascii_lowercase().eq(character)).unwrap().try_into().unwrap();
    
    if char.is_ascii_uppercase() {  
        return index + 27;
    }
    index + 1
}

fn get_chunks_of_three<'a>(input: &'a Input) -> Vec<Vec<String>> {
    let chuks = input.chunks(3);
    let mut chunks = Vec::new();
    for chunk in chuks {
        let chunk_lines: Vec<String> = chunk.iter().map(|tuple| format!("{}{}",tuple.0, tuple.1)).collect();
        chunks.push(chunk_lines);
    }
    
    chunks
}

fn part1(input: &Input) -> u32 {
    let mut total_score: u32 = 0;

    for line in input  {
        let char: char = find_intersection_character(line);
        total_score += calculate_priority_for(char);
    }
    total_score
}

fn part2(input: &Input) -> u32 {
    let mut total_score: u32 = 0;
    let chunks = get_chunks_of_three(input);
    for chunk in chunks  {
        let char: char = find_intersection_character_in_list(chunk);
        total_score += calculate_priority_for(char);
    }
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
    use crate::{part1, part2,parse_input, calculate_priority_for, find_intersection_character, find_intersection_character_in_list};

    #[test]
    fn should_solve_part_1 () {
        let example = "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";
        let parsed_input = parse_input(example);
        let result = part1(&parsed_input);

        assert_eq!(result,157);
    }

    #[test]
    fn should_solve_part_2 () {
        let example = "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";
        let parsed_input = parse_input(example);
        let result = part2(&parsed_input);

        assert_eq!(result,70);
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
        assert_eq!(parsed_example.first().unwrap().0,"vJrwpWtwJgWr");
        assert_eq!(parsed_example.first().unwrap().1,"hcsFMMfFFhFp");
        assert_eq!(parsed_example.get(1).unwrap().0,"jqHRNqRjqzjGDLGL");
        assert_eq!(parsed_example.get(1).unwrap().1,"rsFMfFZSrLrFZsSL");
    }
    
    #[test]
    fn should_calculate_priority_of_intersection_lowercase () {
        let result = calculate_priority_for('p');

        assert_eq!(result,16);
    }

    #[test]
    fn should_calculate_priority_of_intersection_uppercase () {
        let result = calculate_priority_for('P');

        assert_eq!(result,42);
    }

    #[test]
    fn should_find_intersection_in_tuple_lowercase () {
        let example = ("vJrwpWtwJgWr","hcsFMMfFFhFp");

        let result = find_intersection_character(&example);

        assert_eq!(result,'p');
    }

    #[test]
    fn should_find_intersection_in_tuple_uppercase () {
        let example = ("jqHRNqRjqzjGDLGL","rsFMfFZSrLrFZsSL");

        let result = find_intersection_character(&example);

        assert_eq!(result,'L');
    }

    #[test]
    fn should_find_intersection_in_chunk () {
        let example = vec![
            String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
            String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            String::from("PmmdzqPrVvPwwTWBwg")];

        let result = find_intersection_character_in_list(example);

        assert_eq!(result,'r');
    }
}
