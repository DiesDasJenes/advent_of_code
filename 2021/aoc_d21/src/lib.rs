
pub struct Player {
    pub starting_point: i32,
    pub score: i32
}



#[cfg(test)]
mod tests {
    use crate::Player;

    fn get_test_input_players() -> (Player, Player) {
        return (Player{ starting_point: 4, score: 0 }, Player{ starting_point: 8, score: 0 })
    }

    #[test]
    fn play_one_round_with_deterministic_dice() {
         let (mut player1,mut player2) = get_test_input_players();
         rollDice(player1);
         rollDice(player2);
    }

}