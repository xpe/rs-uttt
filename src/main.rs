extern crate uttt;

use uttt::random::*;
use uttt::utility::{h, p};

fn main() {
    h(0, "Games");
    let games = random_game();
    for (i, game) in games.iter().enumerate() {
        println!("Move #{}", i);
        p(game);
        println!("\n");
    }
}
