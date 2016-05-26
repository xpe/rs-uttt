#![feature(slice_patterns)]

#[macro_use]
extern crate bitflags;

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
// requires 16 bits.
bitflags! {
    flags SBoard: u16 {
        const R0 = 0b0000000000011111,
        const R1 = 0b0000001111100000,
        const R2 = 0b0111110000000000,
    }
}

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
bitflags! {
    flags Loc: u8 {
        const R = 0b00001111,
        const C = 0b11110000,
    }
}

// A sub-board location, consisting of a sub-board row and sub-board column.
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

fn empty_board() -> Board {
    Board([SBoard::empty(); 9])
}

// -- functions: -> sub-board --------------------------------------------------

// -- functions: -> row --------------------------------------------------------

// Returns a row when given an array of three slots.
fn slot_row(slots: [Slot; 3]) -> Row {
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
fn row_slots(row: Row) -> [Slot; 3] {
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

// TODO: look at this again
#[allow(non_snake_case)]
fn print_Play() {
    heading("Play");
    println!("{:?}", Play { loc: Loc::empty(), player: Player::X });
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
    println!("{:?}", sboard_slot(SBoard::empty()));

    // data: game
    heading("init_game");
    println!("{:?}", init_game());

    // functions: -> bool
    heading("is_valid_play()");
    println!("{:?}", is_valid_play(
        SBoard::empty(),
        SPlay{ loc: SLoc { row: SRI::R1, col: SCI::C1 },
               player: Player::X}));

    // functions: -> players
    // TODO

    // functions: -> indexes
    // TODO

    // functions: -> slot
    heading("row_slots()");
    println!("{:?}", row_slots(Row::EXO));

    // functions: -> sub-board location
    // TODO

    // functions: -> board location
    // TODO

    // functions: -> sub-board play
    // TODO

    // functions: -> board play
    // TODO

    // functions: -> row
    heading("slot_row");
    println!("{:?}", slot_row(
        [Slot::Empty, Slot::Taken(Player::X), Slot::Taken(Player::O)]));

    // functions: -> sub-board
    // TODO

    // functions: -> board
    heading("sboard_slot");
    println!("{:?}", sboard_slot(SBoard::empty()));

    // functions: -> game
    heading("init_game");
    println!("{:?}", init_game());

}
