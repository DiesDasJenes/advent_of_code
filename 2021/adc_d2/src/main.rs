use std::str::FromStr;

mod util;

struct OldSubmarine {
    horizontal_position: i32,
    depth: i32,
}

struct NewSubmarine {
    horizontal_position: i32,
    depth: i32,
    aim: i32,
}

#[derive(PartialEq, Debug)]
enum Command {
    Forward,
    Up,
    Down,
}

impl Command {
    fn calc(&self, current_value: i32, correction: i32) -> i32 {
        match self {
            Self::Forward => current_value + correction,
            Self::Up => current_value - correction,
            Self::Down => current_value + correction,
        }
    }
}

fn main() {
    println!("Hello, Wichtel!");

    if let Ok(lines) = util::read_lines("./puzzle_input.txt") {
        let input_lines = lines.map(|line| line.unwrap()).collect::<Vec<String>>();

        let solution_part_one = get_solution_for_part_one(&input_lines);
        println!("Part 1");
        println!("Result: {}", solution_part_one);

        let solution_part_two = get_solution_for_part_two(&input_lines);
        println!("Part 2");
        println!("Result: {}", solution_part_two);
    }
}

fn get_solution_for_part_one(input_lines: &Vec<String>) -> i32 {
    let mut submarine = OldSubmarine {
        horizontal_position: 0,
        depth: 0,
    };
    for line in input_lines {
        let command_tuple = get_command(line);
        match command_tuple.0 {
            Command::Forward => {
                submarine.horizontal_position =
                    Command::Forward.calc(submarine.horizontal_position, command_tuple.1)
            }
            Command::Up => submarine.depth = Command::Up.calc(submarine.depth, command_tuple.1),
            Command::Down => submarine.depth = Command::Down.calc(submarine.depth, command_tuple.1),
        }
    }

    return submarine.depth * submarine.horizontal_position;
}

fn get_command(input: &String) -> (Command, i32) {
    let split: Vec<&str> = input.split(" ").collect();
    return (
        match split.get(0).unwrap() {
            &"forward" => Command::Forward,
            &"up" => Command::Up,
            &"down" => Command::Down,
            _ => panic!(),
        },
        i32::from_str(split.get(1).unwrap()).unwrap(),
    );
}

fn get_solution_for_part_two(input_lines: &Vec<String>) -> i32 {
    let mut submarine = NewSubmarine {
        horizontal_position: 0,
        depth: 0,
        aim: 0,
    };
    for line in input_lines {
        let command_tuple = get_command(line);
        match command_tuple.0 {
            Command::Forward => {
                submarine.horizontal_position =
                    Command::Forward.calc(submarine.horizontal_position, command_tuple.1);
                submarine.depth += command_tuple.1 * submarine.aim;
            }
            Command::Up => submarine.aim = Command::Up.calc(submarine.aim, command_tuple.1),
            Command::Down => submarine.aim = Command::Down.calc(submarine.aim, command_tuple.1),
        }
    }

    return submarine.depth * submarine.horizontal_position;
}

#[cfg(test)]
mod test {
    use crate::Command::Forward;
    use crate::{get_command, get_solution_for_part_one, get_solution_for_part_two};

    #[test]
    fn should_identify_command() {
        let command = "forward 5".to_string();
        let actual_command_tuple = get_command(&command);

        assert_eq!(actual_command_tuple.0, Forward);
        assert_eq!(actual_command_tuple.1, 5)
    }

    #[test]
    fn should_calculate_position_of_old_submarine() {
        let test_input_lines: Vec<String> = vec![
            "forward 5".to_string(),
            "down 5".to_string(),
            "forward 8".to_string(),
            "up 3".to_string(),
            "down 8".to_string(),
            "forward 2".to_string(),
        ];
        let expected_result = 150;
        let actual_result = get_solution_for_part_one(&test_input_lines);

        assert_eq!(expected_result, actual_result)
    }

    #[test]
    fn should_calculate_position_of_new_submarine() {
        let test_input_lines: Vec<String> = vec![
            "forward 5".to_string(),
            "down 5".to_string(),
            "forward 8".to_string(),
            "up 3".to_string(),
            "down 8".to_string(),
            "forward 2".to_string(),
        ];
        let expected_result = 900;
        let actual_result = get_solution_for_part_two(&test_input_lines);

        assert_eq!(expected_result, actual_result)
    }
}
