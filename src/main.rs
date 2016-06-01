extern crate uttt;

use uttt::random::*;
use uttt::utility::{h, p};

fn main() {
    h(0, "Games");
    let games = random_game();
    for (i, game) in games.iter().enumerate() {
        println!("Move #{}\n", i);
        p(game);
        println!("\nGame::is_won -> {}\n", game.board.is_won());
    }
}
