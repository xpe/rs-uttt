extern crate rand;
extern crate uttt;

use rand::{SeedableRng, StdRng};
use uttt::data::*;
use uttt::random::*;
use uttt::utility::{h, p};

fn main() {
    h(0, "Ended Games");
    let seed: &[_] = &[219, 9990002, 22004, 23];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let mut xs = 0;
    let mut os = 0;
    let mut ties = 0;
    for i in 0 .. 10 {
        // h(1, format!("Game #{}", i).as_str());
        let games = random_game(&mut rng);
        let game = games.iter().last().unwrap();
        // p(game);
        // print_winner(game.winner());
        let winner = game.winner();
        println!("{:04} {:?}", i, winner);
        match winner {
            None => ties = ties + 1,
            Some(Player::X) => xs = xs + 1,
            Some(Player::O) => os = os + 1,
        }
    }
    println!("\n");
    println!("X wins: {:4}", xs);
    println!("O wins: {:4}", os);
    println!("  ties: {:4}", ties);
}

#[allow(dead_code)]
fn print_winner(winner: Option<Player>) {
    let p = match winner {
        Some(player) => format!("{:?}", player),
        None => format!("-"),
    };
    println!("                 winner : {}", p);
}

#[allow(dead_code)]
fn main_2() {
    h(0, "Games");
    let seed: &[_] = &[1, 2, 4, 12];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let games = random_game(&mut rng);
    for (i, game) in games.iter().enumerate() {
         println!("Move #{}\n", i);
         p(game);
         println!("                 winner : {:?}", game.winner());
    }
}
