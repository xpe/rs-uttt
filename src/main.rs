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
    // h(0, "Game");
    h(1, "Example Game #1");
    p(example_game_1());
    h(1, "Example Game #2");
    p(example_game_2());
    h(1, "Example Game #3");
    p(example_game_3());
    h(1, "Example Game #4");
    p(example_game_4());
    h(1, "Example Game #5");
    p(example_game_5());
    h(1, "Example Game #6");
    p(example_game_6());

    // -- board ----------------------------------------------------------------
    // h(0, "Board");
    // p(example_board_1());

    // -- sub-board ------------------------------------------------------------
    // h(0, "Sub-Board");
    // p(example_sboard_1());
}
