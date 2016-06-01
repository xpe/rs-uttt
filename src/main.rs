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
        // for sboard in game.board.sboards().iter() {
        //     println!("SBoard::is_won -> {}", sboard.is_won());
        // }
    }
}
