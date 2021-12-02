pub mod util;

#[derive(PartialEq, Debug)]
enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

pub fn get_solution_for_part_one(input_lines: &Vec<String>) -> i32 {
    let mut horizontal_position = 0;
    let mut depth = 0;

    for line in input_lines {
        let command = get_command(line);
        match command {
            Command::Forward(correction) => {
                horizontal_position += correction;
            }
            Command::Up(correction) => depth -= correction,
            Command::Down(correction) => depth += correction,
        }
    }

    return depth * horizontal_position;
}

pub fn get_solution_for_part_two(input_lines: &Vec<String>) -> i32 {
    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in input_lines {
        let command = get_command(line);
        match command {
            Command::Forward(correction) => {
                horizontal_position += correction;
                depth += correction * aim;
            }
            Command::Up(correction) => aim -= correction,
            Command::Down(correction) => aim += correction,
        }
    }

    return depth * horizontal_position;
}

fn get_command(input: &String) -> Command {
    let mut split = input.split(" ");
    let command_string = split.next().unwrap();
    let value = split.next().unwrap().parse().unwrap();
    return match command_string {
        "forward" => Command::Forward(value),
        "up" => Command::Up(value),
        "down" => Command::Down(value),
        _ => panic!(),
    };
}

#[cfg(test)]
mod test {
    use crate::Command::Forward;
    use crate::{get_command, get_solution_for_part_one, get_solution_for_part_two};

    #[test]
    fn should_identify_command() {
        let command_line = "forward 5".to_string();
        let actual_command = get_command(&command_line);

        assert_eq!(actual_command, Forward(5));
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
