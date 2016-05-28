use data::{Slot, SRI};
use constants::{EMPTY_GAME, EMPTY_BOARD, EMPTY_SBOARD, EMPTY_ROW, SE};

// TODO: think about this / currently, this is just a placeholder
#[test]
fn test_empty_game() {
    let eg = EMPTY_GAME;
    assert!(eg == eg);
}

// TODO: think about this / currently, this is just a placeholder
#[test]
fn test_empty_board() {
    assert!(EMPTY_BOARD == EMPTY_BOARD);
    assert!(EMPTY_BOARD.sboards() == [
        EMPTY_SBOARD, EMPTY_SBOARD, EMPTY_SBOARD,
        EMPTY_SBOARD, EMPTY_SBOARD, EMPTY_SBOARD,
        EMPTY_SBOARD, EMPTY_SBOARD, EMPTY_SBOARD,
    ]);
    // Note: compare slices as a work-around, since Rust does not currently
    // allow direct comparison of arrays bigger than 32 elements:
    // https://github.com/rust-lang/rfcs/issues/1038
    let slots: [Slot; 81] = EMPTY_BOARD.slots();
    let slots_81 = [
        SE, SE, SE, SE, SE, SE, SE, SE, SE,
        SE, SE, SE, SE, SE, SE, SE, SE, SE,
        SE, SE, SE, SE, SE, SE, SE, SE, SE,
        SE, SE, SE, SE, SE, SE, SE, SE, SE,
        SE, SE, SE, SE, SE, SE, SE, SE, SE,
        SE, SE, SE, SE, SE, SE, SE, SE, SE,
        SE, SE, SE, SE, SE, SE, SE, SE, SE,
        SE, SE, SE, SE, SE, SE, SE, SE, SE,
        SE, SE, SE, SE, SE, SE, SE, SE, SE,
    ];
    assert!(&slots[..] == &slots_81[..]);
    assert!(EMPTY_BOARD.slots_9x9() == [
        [SE, SE, SE, SE, SE, SE, SE, SE, SE],
        [SE, SE, SE, SE, SE, SE, SE, SE, SE],
        [SE, SE, SE, SE, SE, SE, SE, SE, SE],
        [SE, SE, SE, SE, SE, SE, SE, SE, SE],
        [SE, SE, SE, SE, SE, SE, SE, SE, SE],
        [SE, SE, SE, SE, SE, SE, SE, SE, SE],
        [SE, SE, SE, SE, SE, SE, SE, SE, SE],
        [SE, SE, SE, SE, SE, SE, SE, SE, SE],
        [SE, SE, SE, SE, SE, SE, SE, SE, SE],
    ]);
}

#[test]
fn test_empty_sboard() {
    let sb = EMPTY_SBOARD;
    assert!(sb.row(SRI::R0) == EMPTY_ROW);
    assert!(sb.row(SRI::R1) == EMPTY_ROW);
    assert!(sb.row(SRI::R2) == EMPTY_ROW);
    assert!(sb.rows() == [EMPTY_ROW, EMPTY_ROW, EMPTY_ROW]);
    assert!(sb.slots() == [
        SE, SE, SE,
        SE, SE, SE,
        SE, SE, SE
    ]);
    assert!(sb.slots_3x3() == [
        [SE, SE, SE],
        [SE, SE, SE],
        [SE, SE, SE],
    ]);
}

#[test]
fn test_empty_row() {
    let er = EMPTY_ROW;
    assert!(er.slots() == [SE, SE, SE]);
}
