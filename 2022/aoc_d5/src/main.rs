use std::{fs};

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

type Input<'a> = Vec<&'a str>; 

fn parse_input(puzzle_input: &str) -> Input  {
    let parsed_input = puzzle_input
    .split('\n')
    .map(|line| line.trim())
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
    use crate::{part1,part2,parse_input};

    #[test]
    fn should_return_parsed_input () {
        let example = "move 1 from 3 to 9
        move 3 from 5 to 3";

        let parsed_example = parse_input(example);

        assert_eq!(parsed_example.len(),2);
        assert_eq!(parsed_example.first().unwrap().to_owned(), "move 1 from 3 to 9");
    }

       
    
}