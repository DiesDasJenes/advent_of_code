use std::{fs};
use lazy_static::lazy_static;
/*
[N]     [C]                 [Q]    
[W]     [J] [L]             [J] [V]
[F]     [N] [D]     [L]     [S] [W]
[R] [S] [F] [G]     [R]     [V] [Z]
[Z] [G] [Q] [C]     [W] [C] [F] [G]
[S] [Q] [V] [P] [S] [F] [D] [R] [S]
[M] [P] [R] [Z] [P] [D] [N] [N] [M]
[D] [W] [W] [F] [T] [H] [Z] [W] [R]
 1   2   3   4   5   6   7   8   9 
*/

lazy_static! {
    #[derive(Debug)]
    static ref CONTAINER_MAP: Vec<Vec<char>> = vec![
        vec!['D', 'M', 'S','Z','R','F','W','N'], 
        vec!['W','P','Q','G','S'],
        vec!['W','R','V','Q','F','N','J','C'],
        vec!['F','Z','P','C','G','D','L'],
        vec!['T','P','S'],
        vec!['H','D','F','W','R','L'],
        vec!['Z','N','D','C'],
        vec!['W','N','R','F','V','S','J','Q'],
        vec!['R','M','S','G','Z','W','V'],
        ];
}

type Input<'a> = Vec<&'a str>; 

struct CargoCraneInstruction {
    amount: usize,
    source: usize,
    target: usize,
}

fn parse_input(puzzle_input: &str) -> Input  {
    let parsed_input = puzzle_input
    .split('\n')
    .map(|line| line.trim())
    .collect();
    
    parsed_input
}

fn transform_line_to_instruction(line: &&str) -> CargoCraneInstruction{
    let mut split_line = line.split_whitespace();
    
    CargoCraneInstruction{
        amount: split_line.nth(1).unwrap().parse().unwrap(),
        source: split_line.nth(1).unwrap().parse().unwrap(),
        target: split_line.nth(1).unwrap().parse().unwrap()
    }
}

fn get_top_crate_of_each_stack(container_map: &Vec<Vec<char>>) -> Vec<char> {
    container_map.iter().map(|stack| stack[stack.len()-1]).collect()
}

fn part1(input: &Input, container_map: &mut Vec<Vec<char>>) -> Vec<char> {
    move_crates(input, container_map, true)
}

fn part2(input: &Vec<&str>, container_map: &mut Vec<Vec<char>>) -> Vec<char> {
    move_crates(input, container_map, false)
}

fn move_crates(input: &Vec<&str>, container_map: &mut Vec<Vec<char>>, revert: bool) -> Vec<char> {
    for line in input {
        let instruction = transform_line_to_instruction(line);
        let source_stack: &mut Vec<char> = &mut container_map[instruction.source - 1];
        let mut moving_crates = vec![];
        if revert {
            moving_crates.extend(source_stack.drain(source_stack.len()-instruction.amount..).rev()) 
        } else {
            moving_crates.extend(source_stack.drain(source_stack.len()-instruction.amount..))
        }
        let target_stack: &mut Vec<char>  = &mut container_map[instruction.target - 1];
        target_stack.append(&mut moving_crates);
    }
    get_top_crate_of_each_stack(container_map)
}




fn main() {
    let puzzle_input = fs::read_to_string("resources/puzzle_input.txt").unwrap();
    let parsed_input = parse_input(&puzzle_input);
    
    let mut container_map = CONTAINER_MAP.clone();
    println!("Part 1: {:?}", part1(&parsed_input, &mut container_map).iter().collect::<String>());

    let mut container_map = CONTAINER_MAP.clone();
    println!("Part 2: {:?}", part2(&parsed_input, &mut container_map).iter().collect::<String>());
}

#[cfg(test)]
mod test {
    use crate::{part1,part2,parse_input,transform_line_to_instruction, get_top_crate_of_each_stack};

    fn get_test_container_map() -> Vec<Vec<char>> {
        vec![
            vec!['Z','N'],
            vec!['M','C', 'D'],
            vec!['P'],
        ]
    }

    #[test]
    fn should_solve_part_1 () {
        let mut container_map = get_test_container_map();
        let example = vec![
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2"
        ];

        let parsed_example = part1(&example, &mut container_map);

        assert_eq!(parsed_example,vec!['C','M','Z']); 
    }

    #[test]
    fn should_solve_part_2_non_revert_stack () {
        let mut container_map = get_test_container_map();
        let example = vec![
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2"
        ];

        let parsed_example = part2(&example, &mut container_map);

        assert_eq!(parsed_example,vec!['M','C','D']); 
    }

    #[test]
    fn should_return_parsed_input () {
        let example = "move 1 from 2 to 1
        move 3 from 1 to 3";

        let parsed_example = parse_input(example);

        assert_eq!(parsed_example.len(),2);
        assert_eq!(parsed_example.first().unwrap().to_owned(), "move 1 from 2 to 1");
    }

    #[test]
    fn should_transform_puzzle_input_line_to_instruction () {
        let example = "move 1 from 3 to 9";

        let actual = transform_line_to_instruction(&example);

        assert_eq!(actual.amount,1);
        assert_eq!(actual.source,3);
        assert_eq!(actual.target,9);
    }

    #[test]
    fn should_return_vec_of_top_of_each_stack () {
        let example = get_test_container_map();

        let actual = get_top_crate_of_each_stack(&example);

        assert_eq!(actual,vec!['N','D','P']);
    }

       
    
}