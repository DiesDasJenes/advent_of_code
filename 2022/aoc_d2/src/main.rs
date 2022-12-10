use std::{fs};

type Input<'a> = Vec<&'a str>; 

#[derive(PartialEq,Debug,Clone)]
enum GameResultType {
    Win, Lose, Draw
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum PlayerMoveType {
    Rock, Paper, Scissors
}

#[derive(Clone)]
struct GameResult {
    score: u32,
    result_type: GameResultType 
}

struct RockPaperScissorScores {
    score: u32,
    result_type: PlayerMoveType
}


struct RockPaperScissorIndicator {
    move_type: PlayerMoveType,
    indicator: [&'static str; 2]
}

static GAME_RESULT_SCORES: [GameResult; 3] = [
        GameResult{ score: 6, result_type: GameResultType::Win },
        GameResult{ score: 3, result_type: GameResultType::Draw },
        GameResult{ score: 0, result_type: GameResultType::Lose }
    ];

static MOVE_INDICATOR: [RockPaperScissorIndicator; 3] = [
    RockPaperScissorIndicator{ move_type: PlayerMoveType::Rock, indicator: ["A","X"] },
    RockPaperScissorIndicator{ move_type: PlayerMoveType::Paper, indicator: ["B","Y"] },
    RockPaperScissorIndicator{ move_type: PlayerMoveType::Scissors, indicator: ["C","Z"] }
    ];

static MOVE_SCORES: [RockPaperScissorScores; 3] = [
    RockPaperScissorScores{ score: 1, result_type: PlayerMoveType::Rock },
    RockPaperScissorScores{ score: 2, result_type: PlayerMoveType::Paper },
    RockPaperScissorScores{ score: 3, result_type: PlayerMoveType::Scissors }
];

fn get_moves_of_players_for_round(player_move_inputs: &str) -> (PlayerMoveType, PlayerMoveType) {
    let player_moves: Vec<&str> = player_move_inputs.split(' ').collect();
    
    let enemy_move = get_move(&player_moves, 0);
    let my_move = get_move(&player_moves, 1);


    (enemy_move.unwrap(), my_move.unwrap())
}

fn get_strategy_guide_for_round(player_move_inputs: &str) -> (PlayerMoveType, GameResultType) {
    let strategy_tipps: Vec<&str> = player_move_inputs.split(' ').collect();
    
    let opponent_move = get_move(&strategy_tipps, 0);
    let strategy_guide = get_tip(strategy_tipps.get(1).unwrap());


    (opponent_move.unwrap(), strategy_guide)
}

fn get_move(player_moves: &Vec<&str>, index: usize) -> Option<PlayerMoveType> {
    MOVE_INDICATOR.iter()
    .find(|x| x.indicator.contains(player_moves.get(index).unwrap()))
    .map(|s| s.move_type)
}

fn get_tip(player_moves: &str) -> GameResultType {
    match player_moves {
        "X" => return GameResultType::Lose,
        "Y" => return GameResultType::Draw,
        "Z" => return GameResultType::Win,
        _ => panic!("THIS CHARACTER WAS NOT FOUND")
    }
}

fn determine_result_of_round(playermoves: &str) -> (GameResultType, PlayerMoveType) {
    let moves = get_moves_of_players_for_round(playermoves);

    match moves{
        (PlayerMoveType::Rock, PlayerMoveType::Rock) => (GameResultType::Draw, moves.1),
        (PlayerMoveType::Rock, PlayerMoveType::Paper) => (GameResultType::Win, moves.1),
        (PlayerMoveType::Paper, PlayerMoveType::Paper) => (GameResultType::Draw, moves.1),
        (PlayerMoveType::Paper, PlayerMoveType::Scissors) => (GameResultType::Win, moves.1),
        (PlayerMoveType::Scissors, PlayerMoveType::Rock) => (GameResultType::Win, moves.1),
        (PlayerMoveType::Scissors, PlayerMoveType::Scissors) => (GameResultType::Draw, moves.1),
        _ => (GameResultType::Lose, moves.1),
    }
}

fn determine_result_of_round_with_tips(playermoves: &str) -> (GameResultType, PlayerMoveType) {
    let moves = get_strategy_guide_for_round(playermoves);

    match moves{
        (PlayerMoveType::Rock, GameResultType::Win) => (moves.1, PlayerMoveType::Paper),
        (PlayerMoveType::Rock, GameResultType::Lose) => (moves.1, PlayerMoveType::Scissors),
        (PlayerMoveType::Rock, GameResultType::Draw) => (moves.1, PlayerMoveType::Rock),
        (PlayerMoveType::Paper, GameResultType::Win) => (moves.1, PlayerMoveType::Scissors),
        (PlayerMoveType::Paper, GameResultType::Lose) => (moves.1, PlayerMoveType::Rock),
        (PlayerMoveType::Paper, GameResultType::Draw) => (moves.1, PlayerMoveType::Paper),
        (PlayerMoveType::Scissors, GameResultType::Win) => (moves.1, PlayerMoveType::Rock),
        (PlayerMoveType::Scissors, GameResultType::Lose) => (moves.1, PlayerMoveType::Paper),
        (PlayerMoveType::Scissors, GameResultType::Draw) => (moves.1, PlayerMoveType::Scissors),
    }
}

fn parse_input(puzzle_input: &str) -> Input {
    let parsed_input = puzzle_input
    .split('\n')
    .collect();

    parsed_input
}

fn part1(input: &Input) -> u32 {
    let mut total_score: u32 = 0;
    for round in input.iter() {
        let result_of_round = determine_result_of_round(&round);
    
        let game_score = GAME_RESULT_SCORES.iter().find(|x| result_of_round.0.eq(&x.result_type)).map(|f| f.score).unwrap();
        let move_score = MOVE_SCORES.iter().find(|x| result_of_round.1.eq(&x.result_type)).map(|f| f.score).unwrap();
    
        total_score += game_score + move_score;
    }

    total_score
}

fn part2(input: &Input) -> u32 {
    let mut total_score: u32 = 0;
    for round in input.iter() {
        let result_of_round = determine_result_of_round_with_tips(&round);
    
        let game_score = GAME_RESULT_SCORES.iter().find(|x| result_of_round.0.eq(&x.result_type)).map(|f| f.score).unwrap();
        let move_score = MOVE_SCORES.iter().find(|x| result_of_round.1.eq(&x.result_type)).map(|f| f.score).unwrap();
    
        total_score += game_score + move_score;
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
    use crate::{part1, part2,parse_input, get_moves_of_players_for_round,determine_result_of_round, PlayerMoveType, GameResultType};

    #[test]
    fn should_calculate_game_result_part_1 () {
        let example = "A Y\nB X\nC Z";

        let game_result = part1(&parse_input(example));

        assert_eq!(15,game_result);
    }

    #[test]
    fn should_calculate_game_result_part_2 () {
        let example = "A Y\nB X\nC Z";

        let game_result = part2(&parse_input(example));

        assert_eq!(12,game_result);
    }

    #[test]
    fn should_return_parsed_input () {
        let example = "A Y\nB X\nC Z";

        let parsed_example = parse_input(example);

        assert_eq!(parsed_example.len(),3);
    }
    
    #[test]
    fn should_parse_round_input_to_tuple () {
        
        let result = get_moves_of_players_for_round("A Y");
        assert_eq!((PlayerMoveType::Rock, PlayerMoveType::Paper), result);
    }

    #[test]
    fn should_determine_result_of_round () {
        
        let result = determine_result_of_round("A Y");
        assert_eq!((GameResultType::Win, PlayerMoveType::Paper), result);
    }

    #[test]
    fn should_calculate_result_of_round () {
        
        let result = determine_result_of_round("A Y");
        assert_eq!((GameResultType::Win, PlayerMoveType::Paper), result);
    }

    #[test]
    fn should_calculate_score_for_win_of_one_round () {
        let example = "A Y";
        let score = part1(&parse_input(example));
        
        assert_eq!(8 ,score)
    }

    #[test]
    fn should_calculate_score_for_loss_of_one_round () {
        let example = "B X";
        let score = part1(&parse_input(example));
        
        assert_eq!(1 ,score)
    }

    #[test]
    fn should_calculate_score_for_draw_of_one_round () {
        let example = "C Z";
        let score = part1(&parse_input(example));
        
        assert_eq!(6 ,score)
    }
}
