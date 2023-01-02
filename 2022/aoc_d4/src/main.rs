use std::{fs};

type Input<'a> = Vec<&'a str>; 

fn parse_input(puzzle_input: &str) -> Input  {
    let parsed_input = puzzle_input
    .split("\n")
    .map(|line| line.trim())
    .collect();
    
    parsed_input
}

fn get_tuple(sectors: &str) -> std::ops::RangeInclusive<u32> {
    let mut split = sectors.split("-");
    
    
    let left_number = split.next().unwrap().parse().unwrap();
    let right_number = split.next().unwrap().parse().unwrap();
    let sector_range: std::ops::RangeInclusive<u32> = std::ops::RangeInclusive::new(left_number,right_number);

    sector_range
}

fn has_sector_contain_another_sector(line: Vec<&str>) -> bool {
    let left = get_tuple(line.get(0).unwrap());
    let right = get_tuple(line.get(1).unwrap());
    
    left.contains(&right.start()) && left.contains(&right.end()) ||
    right.contains(&left.start()) && right.contains(&left.end())
}

fn has_overlaping_sector(line: Vec<&str>) -> bool {
    let left = get_tuple(line.get(0).unwrap());
    let right = get_tuple(line.get(1).unwrap());
    // (StartA <= EndB) and (EndA >= StartB)
    left.start() <= right.end() && left.end() >= right.start()
}

fn part1(input: &Input) -> u32 {
    let mut total_score: u32 = 0;
    for line in input {
        if has_sector_contain_another_sector(line.split(",").collect()) {
            total_score += 1;
        }
    }
    total_score
}

fn part2(input: &Input) -> u32 {
    let mut total_score: u32 = 0;
    for line in input {
        if has_overlaping_sector(line.split(",").collect()) {
            total_score += 1;
        }
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
    use crate::{part1,part2,parse_input, has_sector_contain_another_sector,get_tuple};

    #[test]
    fn should_solve_part_1 () {
        let example = "2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8";
        let parsed_input = parse_input(example);
        let result = part1(&parsed_input);

        assert_eq!(result,2);
    }

    #[test]
    fn should_solve_part_2 () {
        let example = "2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8";
        let parsed_input = parse_input(example);
        let result = part2(&parsed_input);

        assert_eq!(result,4);
    }

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

    #[test]
    fn should_return_tuple_of_sector_definition () {
        
        let result = get_tuple("2-8");

        assert_eq!(result.start().to_string(),"2");
        assert_eq!(result.end().to_string(),"8");
    }

    #[test]
    fn should_return_true_for_full_contained_sector () {
        
        let is_contained = has_sector_contain_another_sector(vec!["2-8","3-7"]);

        assert!(is_contained);
    }

    #[test]
    fn should_return_true_for_full_contained_sector_on_right_border () {
        
        let is_contained = has_sector_contain_another_sector(vec!["6-6","4-6"]);

        assert!(is_contained);
    }

    #[test]
    fn should_return_true_for_full_contained_sector_on_left_border () {
        
        let is_contained = has_sector_contain_another_sector(vec!["52-84","52-52"]);

        assert!(is_contained);
    }
    
    
}