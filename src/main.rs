extern crate uttt;

use uttt::examples::*;
use uttt::show::Show;

fn h(level: u8, s: &str) {
    let prefix = match level {
        0 => "# ",
        1 => "## ",
        2 => "### ",
        _ => "",
    };
    println!("\n{}{}\n", prefix, s);
}

fn p<T: Show>(x: T) {
    println!("{}", x.show());
}

fn main() {
    // -- game -----------------------------------------------------------------
    h(0, "Game");
    // p(example_game_1());

    // -- board ----------------------------------------------------------------
    h(0, "Board");
    p(example_board_1());

    // -- sub-board ------------------------------------------------------------
    h(0, "Sub-Board");
    // p(example_sboard_1());
}
