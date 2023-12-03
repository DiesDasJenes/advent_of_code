use std::{fs::File, io::{self, BufRead}, collections::HashMap};
use lazy_static::lazy_static;

fn read_file(file_path: &str) -> io::Result<Vec<String>> {
    
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    Ok(lines)
}

fn part1(path_to_puzzle_input: &str) -> u32 {
    let mut sum = 0;
    
    match read_file(path_to_puzzle_input) {
        Ok(lines) => {
            for line in lines {
                
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                let mut game_number: u32 = 0;            
                if let Some(game_info) = parts.last() {
                    if let Some(number) = parts.first().unwrap().trim().strip_prefix("Game ") {
                            game_number = number.parse::<u32>().unwrap();
                    }
               
                
                let game_rounds: Vec<&str> = game_info.split(';').collect();
                let mut is_valid_game = true;        
                for round in game_rounds {
                    let draws: Vec<&str> = round.split(',').map(|draw| draw.trim()).collect();
                    
                    for draw in &draws {
                       if is_invalid_move(draw) {
                        is_valid_game = false
                       } 
                    }
                   
                }
                    if is_valid_game {
                        sum += game_number;
                    }
                }
            }
        }
        Err(err) => eprintln!("Error reading file: {}", err),
    }

    sum
}

fn is_invalid_move(draw: &str) -> bool {
    let draw_parts: Vec<&str> = draw.split_ascii_whitespace().collect();

    match (draw_parts.first(), draw_parts.last()) {
        (Some(first_part), Some(_last_part @ &"red")) if first_part.parse::<u32>().unwrap_or(0) > 12 => true,
        (Some(first_part), Some(_last_part @ &"blue")) if first_part.parse::<u32>().unwrap_or(0) > 14 => true,
        (Some(first_part), Some(_last_part @ &"green")) if first_part.parse::<u32>().unwrap_or(0) > 13 => true,
        _ => false,
    }
}

fn part2(path_to_puzzle_input: &str) -> u32 {
    let mut sum = 0;
    
    match read_file(path_to_puzzle_input) {
        Ok(lines) => {
            for line in lines {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if let Some(game_info) = parts.last() {
                    let cubes_per_color = fewest_number_of_cubes_of_each_color(game_info);
                    let product = cubes_per_color.get("red") .unwrap() * cubes_per_color.get("blue") .unwrap() * cubes_per_color.get("green") .unwrap();
                    sum += product;
                }    
            }
        }
        Err(err) => eprintln!("Error reading file: {}", err),
    }
    
    sum
}

lazy_static! {
    static ref CUBES: HashMap<&'static str, u32> = {
        let mut map = HashMap::new();
        map.insert("blue", 0);
        map.insert("red", 0);
        map.insert("green", 0);
        map
    };
}

fn fewest_number_of_cubes_of_each_color(game_info: &str) -> HashMap<&str, u32> {
    let game_rounds: Vec<&str> = game_info.split(';').collect();
    let mut cubes = CUBES.clone();

    for round in game_rounds {
        let draws: Vec<&str> = round.split(',').map(|draw| draw.trim()).collect();        
            for draw in &draws {
                let draw_parts: Vec<&str> = draw.split_ascii_whitespace().collect();

                match (draw_parts.first(), draw_parts.last()) {
                (Some(first_part), Some(last_part @ &"red")) if first_part.parse::<u32>().unwrap_or(0) > cubes.get(last_part).cloned().unwrap_or(0) => {
                    *cubes.get_mut(last_part).unwrap() = first_part.parse::<u32>().unwrap() ;
                }
                (Some(first_part), Some(last_part @ &"blue")) if first_part.parse::<u32>().unwrap_or(0) > cubes.get(last_part).cloned().unwrap_or(0) => {
                    *cubes.get_mut(last_part).unwrap() = first_part.parse::<u32>().unwrap() ;
                }
                (Some(first_part), Some(last_part @ &"green")) if first_part.parse::<u32>().unwrap_or(0) > cubes.get(last_part).cloned().unwrap_or(0) => {
                    *cubes.get_mut(last_part).unwrap() = first_part.parse::<u32>().unwrap() ;
                }
                _ => {}
            }
         }
    
        }
        cubes
}




fn main() {
    println!("Part 1: {}", part1("resources/puzzle_input.txt"));
    println!("Part 2: {}", part2("resources/puzzle_input.txt"));
}

#[cfg(test)]
mod test {
    
    use rstest::rstest;

    use crate::{part1, is_invalid_move, part2, fewest_number_of_cubes_of_each_color};

    #[test]
    fn should_return_correct_sum_for_part_1 () {
       
        let sum_of_indexes_of_invalid_games = part1("resources/test_part_1.txt");

        assert_eq!(sum_of_indexes_of_invalid_games, 8);
    }

    #[test]
    fn should_return_correct_sum_for_part_2 () {
       
        let sum_of_indexes_of_invalid_games = part2("resources/test_part_1.txt");

        assert_eq!(sum_of_indexes_of_invalid_games, 2286);
    }
    

    #[rstest(
        draw, expected,
        case("20 red", true),
        case("15 blue", true),
        case("15 green", true),
        case("12 red", false),
        case("14 blue", false),
        case("13 green", false),
    )]
    fn should_return_true_for_invalid_draw (draw: &str, expected: bool) {       
        let actual = is_invalid_move(draw);
        assert_eq!(actual,expected);
    }

    
    #[rstest(
        round,
        red_cube,
        green_cube,
        blue_cube,
        case("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 4,2,6),
        case("1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 1,3,4),
        case("8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", 20,13,6),
        case("1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", 14,3,15),
        case("6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 6,3,2),
    )]
    fn should_find_fewest_available_cube_count_for_each_color(round: &str, blue_cube: u32,red_cube:u32,green_cube:u32) {
        let mut expected = std::collections::HashMap::new();
        expected.insert("blue", blue_cube);
        expected.insert("red", red_cube);
        expected.insert("green", green_cube);

        let actual = fewest_number_of_cubes_of_each_color(round);
        assert_eq!(actual, expected);
    }
}
