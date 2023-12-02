use core::num;
use std::fs;

type Input<'a> = Vec<&'a str>; 
struct LineParser;
impl LineParser {
    fn parse_without_numbers_as_word(&self, line:&str) -> Vec<u32> {
        return line.chars()
        .filter_map(|c| c.to_digit(10))
        .collect();
    }

    fn parse_with_numbers_as_word(&self, line:&str) -> Vec<u32> {
        let mut current_index = 0;
        let mut result = Vec::new();

        while current_index < line.len() {
            if let Some((word, value)) = &self.find_word_at_index(&line[current_index..]) {
                result.push(*value);
                current_index += word.len()-1;
            } else if let Some(digit) = line.chars().nth(current_index).and_then(|c| c.to_digit(10)) {
                result.push(digit);
                current_index += 1;
            } else {
                current_index += 1;
            }
        }

        return result
    }

    fn find_word_at_index(&self,substring: &str) -> Option<(&str, u32)> {
        let words = ["one","two", "three","four","five","six","seven","eight","nine"];
        
        words
            .iter()
            .filter_map(|&word| {
                if substring.starts_with(word) {
                    Some((word, self.word_to_digit(word).unwrap_or(0)))
                } else {
                    None
                }
            })
            .max_by_key(|(word, _)| word.len())
    }
    

    fn word_to_digit(&self, word: &str) -> Option<u32> {
        match word.to_lowercase().as_str() {
            "one" => Some(1),
            "two" => Some(2),
            "three" => Some(3),
            "four" => Some(4),
            "five" => Some(5),
            "six" => Some(6),
            "seven" => Some(7),
            "eight" => Some(8),
            "nine" => Some(9),
            _ => word.parse().ok(),
        }
    }
}

fn parse_input(puzzle_input: &str) -> Input {
    puzzle_input.split("\n").collect()
}

fn part1(input: &Input) -> u32 {
    input.iter().map(|&line| get_sum_of_digits(LineParser::parse_without_numbers_as_word, line)).sum()
}

fn part2(input: &Input) -> u32 {
    input.iter().map(|&line| get_sum_of_digits(LineParser::parse_with_numbers_as_word, line)).sum()
}

fn get_sum_of_digits(operation: fn(&LineParser, &str) -> Vec<u32>, line: &str) -> u32 {
    let line_parser = LineParser;
    let mut numbers: Vec<u32> = operation(&line_parser, line);
    
    if numbers.len() >= 2 {
        if let Some(first) = numbers.first().cloned() {
            if let Some(last) = numbers.last().cloned() {
                numbers = vec![first,last]
            }
        }
    }
    if numbers.len() == 1 {
        if let Some(first) = numbers.first().cloned(){
            numbers.push(first)
        }
    }
    println!("{:?}", numbers);
    return numbers.iter().fold(0, |acc, &digit| acc * 10 + digit);
}

fn main() {
    let puzzle_input = fs::read_to_string("resources/puzzle_input.txt").unwrap();
    let parsed_input = parse_input(&puzzle_input);
    println!("Part 1: {}", part1(&parsed_input));
    println!("Part 2: {}", part2(&parsed_input));
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::{parse_input, part1, part2, get_sum_of_digits, LineParser};

    #[test]
    fn should_return_parsed_input () {
        let example = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

        let parsed_example = parse_input(&example);

        assert_eq!(parsed_example.len(),4);
    }

    #[rstest(
        digits, expected,
        case("1abc2", 12),
        case("pqr3stu8vwx", 38),
    )]
    fn should_return_first_and_last_digit_in_string_with_two_digits_in_the_string (digits: &str, expected: u32) {
        let digit = get_sum_of_digits(LineParser::parse_without_numbers_as_word,&digits);

        assert_eq!(digit,expected);
    }

    #[test]
    fn should_find_first_and_last_digit_in_string_with_several_digits_in_the_string () {
        let example = "a1b2c3d4e5f";

        let digit = get_sum_of_digits(LineParser::parse_without_numbers_as_word,&example);

        assert_eq!(digit,15);
    }

    #[test]
    fn should_find_only_digit_in_string_and_duplicate_it () {
        let example = "treb7uchet";

        let digit = get_sum_of_digits(LineParser::parse_without_numbers_as_word,&example);

        assert_eq!(digit,77);
    }

    #[test]
    fn should_return_correct_sum_for_part_1 () {
        let puzzle_input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        let parsed_input = parse_input(&puzzle_input);
        
        let digit = part1(&parsed_input);

        assert_eq!(digit,142);
    }

    #[test]
    fn should_return_correct_sum_for_part_2 () {
        let puzzle_input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        let parsed_input = parse_input(&puzzle_input);
        
        let digit = part2(&parsed_input);

        assert_eq!(digit,281);
    }

    #[test]
    fn should_match_overlapping_numbers () {
        let puzzle_input = "8ghsxbzoneightg";
        let lineparser = LineParser;
        
        
        let digit = lineparser.parse_with_numbers_as_word(puzzle_input);

        assert_eq!(digit,vec![8,1,8]);
    }
}
