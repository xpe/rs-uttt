use data::{SRI};
use constants::{EMPTY_GAME, EMPTY_BOARD, EMPTY_SBOARD, EMPTY_ROW};

#[test]
fn test_empty_game() {
    assert!(EMPTY_GAME == EMPTY_GAME);
}

#[test]
fn test_empty_board() {
    assert!(EMPTY_BOARD == EMPTY_BOARD);
}

#[test]
fn test_empty_sboard() {
    let sb = EMPTY_SBOARD;
    assert!(sb.row(SRI::R0) == EMPTY_ROW);
    assert!(sb.row(SRI::R1) == EMPTY_ROW);
    assert!(sb.row(SRI::R2) == EMPTY_ROW);
}
