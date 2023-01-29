use std::{fs, collections::HashSet};

type Input<'a> = Vec<&'a str>;

fn parse_input(puzzle_input: &str) -> Input  {
    let parsed_input = puzzle_input
    .split('\n')
    .map(|line| line.trim())
    .collect();
    
    parsed_input
}

fn are_all_chars_unique(slice: Option<&str>)  -> bool {
    let string = slice.unwrap();
    let mut set = HashSet::new();
    for c in string.chars() {
        if set.contains(&c) {
            return false;
        }
        set.insert(c);
    }
    true
}

fn find_marker(input: &str, size: usize) -> Option<u32> {
    for i in 0..input.len()-size {
        let slice = input.get(i..i+size);
        if are_all_chars_unique(slice) {
            return Some((i+size) as u32);
        }  
    }
    None
}

fn main() {
    let puzzle_input = fs::read_to_string("resources/puzzle_input.txt").unwrap();
    let parsed_input = parse_input(&puzzle_input);
    
    println!("Part 1: {:?}", find_marker(parsed_input.get(0).unwrap(), 4));

    println!("Part 2: {:?}", find_marker(parsed_input.get(0).unwrap(), 14));
}

#[cfg(test)]
mod test {
    use crate::{find_marker, are_all_chars_unique};
    use rstest::rstest;

    fn get_test_input() -> Vec<&'static str> {
        return vec!["mjqjpqmgbljsphdztnvjfqwrcgsmlb",
                    "bvwbjplbgvbhsrlpgdmjqwftvncz",
                    "nppdvjthqldpwncqszvftbrmjlhg",
                    "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
                    "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"];
    }

    #[rstest]
    #[case(0, 7)]
    #[case(1, 5)]
    #[case(2, 6)]
    #[case(3, 10)]
    #[case(4, 11)]
    fn should_find_first_maker (#[case] input: usize, #[case] expected: u32) {
        let text_input = get_test_input();
        let marker = find_marker(text_input.get(input).unwrap(), 4);

        assert_eq!(marker.unwrap(),expected);
    }

    #[rstest]
    #[case(0, 19)]
    #[case(1, 23)]
    #[case(2, 23)]
    #[case(3, 29)]
    #[case(4, 26)]
    fn should_find_maker (#[case] input: usize, #[case] expected: u32) {
        let text_input = get_test_input();
        let marker = find_marker(text_input.get(input).unwrap(), 14);

        assert_eq!(marker.unwrap(),expected);
    }

    #[rstest]
    #[case("abcd", true)]
    #[case("mjqj", false)]
    fn should_check_for_unique_chars (#[case] input: &str, #[case] expected: bool) {
        let example = Some(input);

        let result = are_all_chars_unique(example);

        assert_eq!(result,expected);
    }

}