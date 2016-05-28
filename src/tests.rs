use data::{SBoard, Slot, SRI};
use constants::{EMPTY_GAME, EMPTY_BOARD, EMPTY_SBOARD, EMPTY_ROW};
use constants::{SE, SX, SO};

// == game =====================================================================

// TODO: think about this / currently, this is just a placeholder
#[test]
fn test_empty_game() {
    let eg = EMPTY_GAME;
    assert!(eg == eg);
}

// == board ====================================================================

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

// == sub-board ================================================================

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
fn test_sboard_from_slots() {
    let slots_1 = [
        SE, SE, SX,
        SO, SX, SE,
        SE, SE, SE,
    ];
    assert!(SBoard::from_slots(slots_1) == SBoard(0b0000001010100001));
    let slots_2 = [
        SE, SE, SE,
        SE, SE, SX,
        SO, SX, SE,
    ];
    assert!(SBoard::from_slots(slots_2) == SBoard(0b0101010000100000));
}

// == rows =====================================================================

// == row ======================================================================

#[test]
fn test_empty_row() {
    let er = EMPTY_ROW;
    assert!(er.slots() == [SE, SE, SE]);
}

// == board play ===============================================================

// == sub-board play ===========================================================

// == board location ===========================================================

// == sub-board location =======================================================

// == slots ====================================================================

// == slot =====================================================================

// == board indexes ============================================================

// == sub-board indexes ========================================================

// == player ===================================================================

// == u8 =======================================================================

// == bool =====================================================================