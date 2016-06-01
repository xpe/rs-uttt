extern crate rand;
extern crate uttt;

use rand::{SeedableRng, StdRng};
use uttt::random::*;
use uttt::utility::{h, p};

fn main() {
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
