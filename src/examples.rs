/// Examples.

use data::*;
use constants::*;

pub fn example_game_1() -> Game {
    EMPTY_GAME
}

pub fn example_game_2() -> Game {
    example_game_1().play(Play {
        loc: Loc::new(RI::R2, CI::C8),
        player: Player::X,
    }).unwrap()
}

pub fn example_game_3() -> Game {
    example_game_2().play(Play {
        loc: Loc::new(RI::R6, CI::C8),
        player: Player::O,
    }).unwrap()
}

pub fn example_game_4() -> Game {
    example_game_3().play(Play {
        loc: Loc::new(RI::R2, CI::C7),
        player: Player::X,
    }).unwrap()
}

pub fn example_game_5() -> Game {
    example_game_4().play(Play {
        loc: Loc::new(RI::R7, CI::C5),
        player: Player::O,
    }).unwrap()
}

pub fn example_game_6() -> Game {
    example_game_5().play(Play {
        loc: Loc::new(RI::R5, CI::C7),
        player: Player::X,
    }).unwrap()
}

pub fn example_board_1() -> Board {
    EMPTY_BOARD
}

pub fn example_sboard_1() -> SBoard {
    EMPTY_SBOARD
}
