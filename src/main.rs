#![feature(slice_patterns)]

// Note: Types (i.e. structs or enums) that end in 'X' are 'extra' data
// structures, intended for internal use. Often, they are more convenient, but
// less efficient.

// == data =====================================================================

// -- data: game ---------------------------------------------------------------

// A Game is the combination of a board and an optional last play. Note: a last
// play is only None for an empty (i.e. initial) board.
#[derive(Debug)]
struct Game { board: Board, last_loc: Option<Loc> }

// -- data: board, sub-board ---------------------------------------------------

// A Board is an array of 9 sub-boards, indexed like this:
// 0 1 2
// 3 4 5
// 6 7 8
#[derive(Debug)]
struct Board([SBoard; 9]);

// An SBoard2 (a sub-board) is an array of 9 slots, indexed like this:
// 0 1 2
// 3 4 5
// 6 7 8
#[allow(dead_code)]
#[derive(Debug)]
struct SBoardX([Slot; 9]);

// An SBoard (a sub-board) has 3 rows, each having 3 slots. This representation
// requires 16 bits:
//
// 0b0000000000011111
// 0b0000001111100000
// 0b0111110000000000
#[derive(Debug, Copy, Clone)]
struct SBoard(u16);

// -- data: row ----------------------------------------------------------------

// An enumeration of possible row values. ('E' means empty, 'X' means player X,
// 'O' means player O.) I'd like to only use 5 bits but Rust needs to align data
// structures on byte boundaries.
#[derive(Debug)]
#[repr(u8)]
enum Row {
    EEE, EXE, EOE, XEE, XXE, XOE, OEE, OXE, OOE,
    EEX, EXX, EOX, XEX, XXX, XOX, OEX, OXX, OOX,
    EEO, EXO, EOO, XEO, XXO, XOO, OEO, OXO, OOO
}

// -- data: play, sub-board play -----------------------------------------------

// A board play, consisting of a location and player.
#[derive(Debug)]
struct Play { loc: Loc, player: Player }

// A sub-board play, consisting of a sub-board location and player.
#[derive(Debug)]
struct SPlay { loc: SLoc, player: Player }

// -- data: location, sub-board location ---------------------------------------

// A location on a board, consisting of a row index and column index.
#[allow(dead_code)]
#[derive(Debug)]
struct LocX { row: RI, col: CI }

// A location on a board.
#[derive(Debug)]
struct Loc(u8);
// 0b00001111
// 0b11110000

// A sub-board location, having two indexes into the sub-board row and col.
#[derive(Debug)]
struct SLoc { row: SRI, col: SCI }

// -- data: slot ---------------------------------------------------------------

// A slot is either taken by a player or empty.
#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
enum Slot { Taken(Player), Empty }

// -- data: row and column indexes ---------------------------------------------

// A board row index, ranging from 0 to 8, inclusive.
#[derive(Debug)]
#[repr(u8)]
enum RI { R0, R1, R2, R3, R4, R5, R6, R7, R8 }

// A board column index, ranging from 0 to 8, inclusive.
#[derive(Debug)]
#[repr(u8)]
enum CI { C0, C1, C2, C3, C4, C5, C6, C7, C8 }

// A sub-board row index: 0, 1, or 2.
#[derive(Debug)]
#[repr(u8)]
enum SRI { R0, R1, R2 }

// A sub-board column index: 0, 1, or 2.
#[derive(Debug)]
#[repr(u8)]
enum SCI { C0, C1, C2 }

// -- data: player -------------------------------------------------------------

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
enum Player { X, O }

// == functions ================================================================

// -- functions: -> game -------------------------------------------------------

fn init_game() -> Game {
    Game { board: empty_board(), last_loc: None }
}

// -- functions: -> board ------------------------------------------------------

// TODO: implement
fn empty_board() -> Board {
    let sb = empty_sboard();
    Board([sb; 9])
}

// -- functions: -> sub-board --------------------------------------------------

// TODO: examine this
// Note: `SBoard(0)` may be problematic since it assumes 0 means empty.
fn empty_sboard() -> SBoard {
    SBoard(0)
}

// Returns a sub-board from 3 rows.
// TODO: implement
#[allow(unused_variables)]
#[allow(dead_code)]
fn rows_as_sboard(rows: [Row; 3]) -> SBoard {
    // let r0 = rows[0];
    // let r1 = rows[1];
    // let r2 = rows[2];
    SBoard(0)
}

// Returns a sub-board from 9 slots.
// TODO: implement
#[allow(unused_variables)]
#[allow(dead_code)]
fn slots_as_sboard(rows: [Slot; 9]) -> SBoard {
    SBoard(0)
}

// -- functions: -> rows --------------------------------------------------------

// impl SBoard {
// }

// -- functions: -> row --------------------------------------------------------

impl SBoard {
    #[allow(dead_code)]
    fn row(self, ri: SRI) -> Row {
        let s = match ri { SRI::R0 => 0, SRI::R1 => 5, SRI::R2 => 10 };
        u8_as_row((self.0 >> s & 0b11111) as u8)
    }
}

// Note: the function names includes `_as_` because this is inexpensive.
fn u8_as_row(x: u8) -> Row {
    match x {
        0x00 => Row::EEE,
        0x01 => Row::EEX,
        0x02 => Row::EEO,
        0x03 => Row::EXE,
        0x04 => Row::EXX,
        0x05 => Row::EXO,
        0x06 => Row::EOE,
        0x07 => Row::EOX,
        0x08 => Row::EOO,
        0x09 => Row::XEE,
        0x0A => Row::XEX,
        0x0B => Row::XEO,
        0x0C => Row::XXE,
        0x0D => Row::XXX,
        0x0E => Row::XXO,
        0x0F => Row::XOE,
        0x10 => Row::XOX,
        0x11 => Row::XOO,
        0x12 => Row::OEE,
        0x13 => Row::OEX,
        0x14 => Row::OEO,
        0x15 => Row::OXE,
        0x16 => Row::OXX,
        0x17 => Row::OXO,
        0x18 => Row::OOE,
        0x19 => Row::OOX,
        0x1A => Row::OOO,
        _ => panic!("internal error"),
    }
}

// Returns a row when given an array of three slots.
fn slots_as_row(slots: [Slot; 3]) -> Row {
    const E: Slot = Slot::Empty;
    const X: Slot = Slot::Taken(Player::X);
    const O: Slot = Slot::Taken(Player::O);
    match slots {
        [E, E, E] => Row::EEE,
        [E, E, O] => Row::EEO,
        [E, E, X] => Row::EEX,
        [E, O, E] => Row::EOE,
        [E, O, O] => Row::EOO,
        [E, O, X] => Row::EOX,
        [E, X, E] => Row::EXE,
        [E, X, O] => Row::EXO,
        [E, X, X] => Row::EXX,
        [O, E, E] => Row::OEE,
        [O, E, O] => Row::OEO,
        [O, E, X] => Row::OEX,
        [O, O, E] => Row::OOE,
        [O, O, O] => Row::OOO,
        [O, O, X] => Row::OOX,
        [O, X, E] => Row::OXE,
        [O, X, O] => Row::OXO,
        [O, X, X] => Row::OXX,
        [X, E, E] => Row::XEE,
        [X, E, O] => Row::XEO,
        [X, E, X] => Row::XEX,
        [X, O, E] => Row::XOE,
        [X, O, O] => Row::XOO,
        [X, O, X] => Row::XOX,
        [X, X, E] => Row::XXE,
        [X, X, O] => Row::XXO,
        [X, X, X] => Row::XXX,
    }
}

// -- functions: -> slot -------------------------------------------------------

// Returns an array of three slots for a given row.
fn row_as_slots(row: Row) -> [Slot; 3] {
    const E: Slot = Slot::Empty;
    const X: Slot = Slot::Taken(Player::X);
    const O: Slot = Slot::Taken(Player::O);
    match row {
        Row::EEE => [E, E, E],
        Row::EEO => [E, E, O],
        Row::EEX => [E, E, X],
        Row::EOE => [E, O, E],
        Row::EOO => [E, O, O],
        Row::EOX => [E, O, X],
        Row::EXE => [E, X, E],
        Row::EXO => [E, X, O],
        Row::EXX => [E, X, X],
        Row::OEE => [O, E, E],
        Row::OEO => [O, E, O],
        Row::OEX => [O, E, X],
        Row::OOE => [O, O, E],
        Row::OOO => [O, O, O],
        Row::OOX => [O, O, X],
        Row::OXE => [O, X, E],
        Row::OXO => [O, X, O],
        Row::OXX => [O, X, X],
        Row::XEE => [X, E, E],
        Row::XEO => [X, E, O],
        Row::XEX => [X, E, X],
        Row::XOE => [X, O, E],
        Row::XOO => [X, O, O],
        Row::XOX => [X, O, X],
        Row::XXE => [X, X, E],
        Row::XXO => [X, X, O],
        Row::XXX => [X, X, X],
    }
}

// TODO
// Returns the Slot for a particular column index for a given Row.
// fn slot_in_row(col: SCI, row: Row) -> Slot {
//    match
// }

// TODO: implement
#[allow(unused_variables)]
fn sboard_slot(sb: SBoard) -> Slot {
    Slot::Empty
}

// -- functions: player --------------------------------------------------------

// TODO: implement
// Returns the last player of a game.
#[allow(dead_code)]
#[allow(unused_variables)]
fn game_last_player(game: Game) -> Option<Player> {
    unimplemented!();
}

// -- functions: -> u8 ---------------------------------------------------------

#[allow(dead_code)]
fn row_as_u8(row: Row) -> u8 {
    match row {
        Row::EEE => 0x00,
        Row::EEX => 0x01,
        Row::EEO => 0x02,
        Row::EXE => 0x03,
        Row::EXX => 0x04,
        Row::EXO => 0x05,
        Row::EOE => 0x06,
        Row::EOX => 0x07,
        Row::EOO => 0x08,
        Row::XEE => 0x09,
        Row::XEX => 0x0A,
        Row::XEO => 0x0B,
        Row::XXE => 0x0C,
        Row::XXX => 0x0D,
        Row::XXO => 0x0E,
        Row::XOE => 0x0F,
        Row::XOX => 0x10,
        Row::XOO => 0x11,
        Row::OEE => 0x12,
        Row::OEX => 0x13,
        Row::OEO => 0x14,
        Row::OXE => 0x15,
        Row::OXX => 0x16,
        Row::OXO => 0x17,
        Row::OOE => 0x18,
        Row::OOX => 0x19,
        Row::OOO => 0x1A,
    }
}

// -- functions: -> bool -------------------------------------------------------

// Is the move valid for the given sub-board?
#[allow(unused_variables)]
fn is_valid_play(sb: SBoard, sp: SPlay) -> bool {
    // TODO: implement
    match sp.loc {
        SLoc { row: r, col: c } => true
    }
}

// -- functions: printing ------------------------------------------------------

#[allow(non_snake_case)]
fn print_Player() {
    heading("Player");
    println!("{:?}", Player::X);
    println!("{:?}", Player::O);
}

#[allow(non_snake_case)]
fn print_SRI() {
    heading("SRI");
    println!("{:?}", SRI::R0);
    println!("{:?}", SRI::R1);
    println!("{:?}", SRI::R2);
}

#[allow(non_snake_case)]
fn print_SCI() {
    heading("SCI");
    println!("{:?}", SCI::C0);
    println!("{:?}", SCI::C1);
    println!("{:?}", SCI::C2);
}

#[allow(non_snake_case)]
fn print_RI() {
    heading("RI");
    println!("{:?}", RI::R0);
    println!("{:?}", RI::R1);
    println!("{:?}", RI::R2);
    println!("{:?}", RI::R3);
    println!("{:?}", RI::R4);
    println!("{:?}", RI::R5);
    println!("{:?}", RI::R6);
    println!("{:?}", RI::R7);
    println!("{:?}", RI::R8);
}

#[allow(non_snake_case)]
fn print_CI() {
    heading("CI");
    println!("{:?}", CI::C0);
    println!("{:?}", CI::C1);
    println!("{:?}", CI::C2);
    println!("{:?}", CI::C3);
    println!("{:?}", CI::C4);
    println!("{:?}", CI::C5);
    println!("{:?}", CI::C6);
    println!("{:?}", CI::C7);
    println!("{:?}", CI::C8);
}

#[allow(non_snake_case)]
fn print_Slot() {
    heading("Slot");
    println!("{:?}", Slot::Empty);
    println!("{:?}", Slot::Taken(Player::X));
    println!("{:?}", Slot::Taken(Player::O));
}

#[allow(non_snake_case)]
fn print_SPlay() {
    heading("SPlay");
    println!("{:?}", SPlay {
        loc: SLoc { row: SRI::R0, col: SCI::C2 },
        player: Player::X });
}

// TODO: Loc(0) needs improvement
#[allow(non_snake_case)]
fn print_Play() {
    heading("Play");
    println!("{:?}", Play { loc: Loc(0), player: Player::X });
}

#[allow(non_snake_case)]
fn print_Row() {
    heading("Row");
    println!("{:?}", Row::EEE);
    println!("{:?}", Row::EEO);
    println!("{:?}", Row::EEX);
    println!("{:?}", Row::EOE);
    println!("{:?}", Row::EOO);
    println!("{:?}", Row::EOX);
    println!("{:?}", Row::EXE);
    println!("{:?}", Row::EXO);
    println!("{:?}", Row::EXX);
    println!("{:?}", Row::OEE);
    println!("{:?}", Row::OEO);
    println!("{:?}", Row::OEX);
    println!("{:?}", Row::OOE);
    println!("{:?}", Row::OOO);
    println!("{:?}", Row::OOX);
    println!("{:?}", Row::OXE);
    println!("{:?}", Row::OXO);
    println!("{:?}", Row::OXX);
    println!("{:?}", Row::XEE);
    println!("{:?}", Row::XEO);
    println!("{:?}", Row::XEX);
    println!("{:?}", Row::XOE);
    println!("{:?}", Row::XOO);
    println!("{:?}", Row::XOX);
    println!("{:?}", Row::XXE);
    println!("{:?}", Row::XXO);
    println!("{:?}", Row::XXX);
}

// -- functions: helpers for main ----------------------------------------------

fn heading(s: &str) {
    println!("\n{} ...", s);
}

// -- functions: main ----------------------------------------------------------

fn main() {
    // data: players
    print_Player();

    // data: indexes
    print_SRI();
    print_SCI();
    print_RI();
    print_CI();

    // data: slot
    print_Slot();

    // data: sub-board location
    // TODO
    // print_SLoc();

    // data: board location
    // TODO
    // print_Loc();

    // data: sub-board play
    print_SPlay();

    // data: board play
    print_Play();

    // data: row
    print_Row();

    // data: sub-board

    // data: board
    heading("sboard_slot");
    println!("{:?}", sboard_slot(empty_sboard()));

    // data: game
    heading("init_game");
    println!("{:?}", init_game());

    // functions: -> bool
    heading("is_valid_play()");
    println!("{:?}", is_valid_play(
        empty_sboard(),
        SPlay{ loc: SLoc { row: SRI::R1, col: SCI::C1 },
               player: Player::X}));

    // functions: -> u8
    // TODO

    // functions: -> players
    // TODO

    // functions: -> indexes
    // TODO

    // functions: -> slot
    heading("row_as_slots()");
    println!("{:?}", row_as_slots(Row::EXO));

    // functions: -> sub-board location
    // TODO

    // functions: -> board location
    // TODO

    // functions: -> sub-board play
    // TODO

    // functions: -> board play
    // TODO

    // functions: -> row
    heading("slots_as_row");
    println!("{:?}", slots_as_row(
        [Slot::Empty, Slot::Taken(Player::X), Slot::Taken(Player::O)]));

    // functions: -> sub-board
    heading("rows_as_sboard");
    println!("{:?}", rows_as_sboard([Row::XOO, Row::XEE, Row::XEO]));

    // functions: -> board
    heading("sboard_slot");
    println!("{:?}", sboard_slot(empty_sboard()));

    // functions: -> game
    heading("init_game");
    println!("{:?}", init_game());
}
