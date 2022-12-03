mod lib;

fn main() {
    println!("Hello Philipp,");
    println!("The puzzle input is:");
    println!("Player 1 starting position: 8\nPlayer 2 starting position: 5");
    let mut player1 = lib::Player{ starting_point: 8, score: 0 };
    let mut player2 = lib::Player{ starting_point: 5, score: 0 };

}
