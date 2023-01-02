use std::{fs};

type Input<'a> = Vec<&'a str>; 

fn parse_input(puzzle_input: &str) -> Input  {
    let parsed_input = puzzle_input
    .split("\n")
    .collect();
    
    parsed_input
}


fn part1(input: &Input) -> u32 {
    let mut total_score: u32 = 0;

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
    use crate::{part1, part2,parse_input};

    #[test]
    fn should_return_parsed_input () {
        let example = "2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8";

        let parsed_example = parse_input(example);

        assert_eq!(parsed_example.len(),6);
        assert_eq!(parsed_example.get(0).unwrap().to_owned(), "2-4,6-8");
    }
    
    
}