extern crate rand;
extern crate uttt;

use uttt::data::*;
use uttt::random::*;
use uttt::utility::*;

fn main() {
    h(0, "Ended Games");
    // let seed: &[_] = &[235, 9990005, 22004, 23];
    // let mut rng: rand::StdRng = rand::SeedableRng::from_seed(seed);
    let mut rng = rand::thread_rng();
    let mut x_wins = 0;
    let mut o_wins = 0;
    let mut ties = 0;
    let mut games_len = 0;
    let trials = 1000;
    for i in 0 .. trials {
        let games = random_games(&mut rng);
        let game_len = games.len();
        let game = games.iter().last().unwrap();
        let winner = game.winner();
        println!("Game #{:4}: {} in {}", i, result_str(winner), game_len);
        games_len += game_len;
        match winner {
            None => ties += 1,
            Some(Player::X) => x_wins += 1,
            Some(Player::O) => o_wins += 1,
        }
    }
    println!("");
    println!("X wins: {:4}", x_wins);
    println!("O wins: {:4}", o_wins);
    println!("  ties: {:4}", ties);
    println!("");
    println!("average game length: {}",
             (games_len as f64) / (trials as f64));
}

fn result_str(op: Option<Player>) -> &'static str {
    match op {
        Some(Player::X) => "X wins",
        Some(Player::O) => "O wins",
        None => "  tie ",
    }
}
