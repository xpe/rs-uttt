#[macro_use]
extern crate bitflags;

// ---- data: game ----

// A Game is the combination of a board and an optional last play. Note: a last
// play is only None for an empty (i.e. initial) board.
#[derive(Debug)]
struct Game { board: Board, last_play: Option<Coord_> }

// ---- data: boards ----

// A Board is an array of 9 sub-boards, indexed like this:
// 0 1 2
// 3 4 5
// 6 7 8
#[derive(Debug)]
struct Board([SBoard; 9]);

// An SBoard (sub-board or 'small board') has 3 rows, each having 3 slots. This
// representation requires 16 bits.
bitflags! {
    flags SBoard: u16 {
        const R0 = 0b0000000000011111,
        const R1 = 0b0000001111100000,
        const R2 = 0b0111110000000000,
    }
}

// ---- data: row ----

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

// I want to use names that indicate when a type/data structure
// is a bit-level thing vs a conceptual thing.

// ---- data: slot ----

#[derive(Debug)]
#[repr(u8)]
enum Slot { Taken(Player), Empty }

// ---- data: player ----

#[derive(Debug)]
#[repr(u8)]
enum Player { X, O }

// ---- data: moves ----

// A board play
#[derive(Debug)]
struct Play { coord: Coord, player: Player }

// A sub-board play
#[derive(Debug)]
struct SPlay { coord: SCoord, player: Player }

// ---- data: coordinates (pairs of indexes) ----

// A board coordinate, consisting of a row and column.
#[derive(Debug)]
struct Coord { row: RI, col: CI }

// A compact representation for a boord coordinate.
bitflags! {
    flags Coord_: u8 {
        const R = 0b00001111,
        const C = 0b11110000,
    }
}

// A sub-board coordinate, consisting of a sub-board row and sub-board column.
#[derive(Debug)]
struct SCoord { row: SRI, col: SCI }

// ---- data: indexes ----

// A board row index, ranging from 0 to 8, inclusive.
#[derive(Debug)]
#[repr(u8)]
enum RI { R0, R1, R2, R3, R4, R5, R6, R7, R8 }

// A board column index, ranging from 0 to 8, inclusive.
#[derive(Debug)]
#[repr(u8)]
enum CI { C0, C1, C2, C3, C4, C5, C6, C7, C8 }

// A sub-board row index: 0, 1, or 2
#[derive(Debug)]
#[repr(u8)]
enum SRI { R0, R1, R2 }

// A sub-board column index: 0, 1, or 2
#[derive(Debug)]
#[repr(u8)]
enum SCI { C0, C1, C2 }

// ---- functions ----

fn init_game() -> Game {
    Game { board: empty_board(), last_play: None }
}

fn empty_board() -> Board {
    Board([SBoard::empty(); 9])
}

// TODO: remove #[allow(unused_variables)]
#[allow(unused_variables)]
fn slot(sb: SBoard) -> Slot {
    Slot::Empty
}

// Is the move valid for the given sub-board?
// TODO: remove #[allow(unused_variables)]
#[allow(unused_variables)]
fn is_valid_play(sb: SBoard, sp: SPlay) -> bool {
    // TODO: implement
    match sp.coord {
        SCoord { row: r, col: c } => true
    }
}

// Returns an array of three Slots for a given Row.
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

// Returns the Slot for a particular column index for a given Row.
// fn slot_in_row(col: SCI, row: Row) -> Slot {
//    match
// }

// ---- debugging / testing ----

fn print_examples() {
    print_players();
    print_rows();
    print_board_indexes();
    print_sub_board_indexes();
    print_slots();
    print_plays();
    print_sub_board_plays();

    println!("\ninit_game() ...");
    println!("{:?}", init_game());

    println!("\nslot() ...");
    println!("{:?}", slot(SBoard::empty()));

    println!("\nis_valid_play() ...");
    println!("{:?}", is_valid_play(
        SBoard::empty(),
        SPlay{ coord: SCoord { row: SRI::R1, col: SCI::C1 },
               player: Player::X}));

    println!("\nrow_slots() ...");
    println!("{:?}", row_slots(Row::EXO));
}

fn print_players() {
    println!("\nPlayer ...");
    println!("{:?}", Player::X);
    println!("{:?}", Player::O);
}

fn print_rows() {
    println!("\nRow ...");
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

fn print_board_indexes() {
    println!("\nRI, CI ...");
    println!("{:?}", RI::R0);
    println!("{:?}", RI::R1);
    println!("{:?}", RI::R2);
    println!("{:?}", RI::R3);
    println!("{:?}", RI::R4);
    println!("{:?}", RI::R5);
    println!("{:?}", RI::R6);
    println!("{:?}", RI::R7);
    println!("{:?}", RI::R8);
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

fn print_sub_board_indexes() {
    println!("\nSRI, SCI ...");
    println!("{:?}", SRI::R0);
    println!("{:?}", SRI::R1);
    println!("{:?}", SRI::R2);
    println!("{:?}", SCI::C0);
    println!("{:?}", SCI::C1);
    println!("{:?}", SCI::C2);
}

fn print_slots() {
    println!("\nSlot ...");
    println!("{:?}", Slot::Empty);
    println!("{:?}", Slot::Taken(Player::X));
    println!("{:?}", Slot::Taken(Player::O));
}

fn print_plays() {
    println!("\nPlay ...");
    println!("{:?}", Play { coord: Coord { row: RI::R0, col: CI::C0 },
                            player: Player::X });
}

fn print_sub_board_plays() {
    println!("\nSPlay ...");
    println!("{:?}", SPlay {
        coord: SCoord{row: SRI::R0, col: SCI::C2},
        player: Player::X });
}

fn main() {
    print_examples();
}
