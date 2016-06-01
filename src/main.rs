extern crate rand;
extern crate uttt;

use rand::{SeedableRng, StdRng};
use uttt::data::*;
use uttt::random::*;
use uttt::utility::{h, p};

fn main() {
    h(0, "Ended Games");
    let seed: &[_] = &[1, 2, 4, 12];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    for i in 0 .. 1000 {
        // h(1, format!("Game #{}", i).as_str());
        let games = random_game(&mut rng);
        let game = games.iter().last().unwrap();
        // p(game);
        // print_winner(game.winner());
        println!("{:04} {:?}", i, game.winner());
    }
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
