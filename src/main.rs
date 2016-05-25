#[macro_use]
extern crate bitflags;

// ---------- data structures ----------

// A Game is the combination of a board and an optional last move. Note: a last
// move is only None for an empty (i.e. initial) board.
#[derive(Debug)]
struct Game { board: Board, last_move: Option<Move> }

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

#[derive(Debug)]
enum Player { X, O }

#[derive(Debug)]
enum Slot { Taken(Player), Empty }

// A Move has a row and a column
bitflags! {
    flags Move: u8 {
        const R = 0b00001111,
        const C = 0b11110000,
    }
}

// A sub-board move
#[derive(Debug)]
struct SMove { coord: SCoord, player: Player }

// A sub-board coordinate
#[derive(Debug)]
struct SCoord { row: SRow, col: SCol }

// A sub-board row: 0, 1, or 2
#[derive(Debug)]
enum SRow { R0, R1, R2 }

// A sub-board column: 0, 1, or 2
#[derive(Debug)]
enum SCol { C0, C1, C2 }

// ---------- functions ----------

fn init_game() -> Game {
    Game { board: empty_board(), last_move: None }
}

fn empty_board() -> Board {
    Board([SBoard::empty(); 9])
}

// TODO: remove #[allow(unused_variables)]
#[allow(unused_variables)]
fn slot(sb: SBoard) -> Slot {
    Slot::Empty
}

// Plays (makes a move) on the sub-board and returns an updated sub-board.
// TODO: remove #[allow(unused_variables)]
#[allow(unused_variables)]
fn play(sb: SBoard, sm: SMove) -> SCoord {
    match sm.coord {
        SCoord { row: r, col: c } => SCoord { row: r, col: c }
    }
}

// ---------- debugging / testing ----------

fn print_examples() {
    println!("sub-board row 0 {:?}", SRow::R0);
    println!("sub-board row 1 {:?}", SRow::R1);
    println!("sub-board row 1 {:?}", SRow::R2);
    println!("sub-board col 0 {:?}", SCol::C0);
    println!("sub-board col 1 {:?}", SCol::C1);
    println!("sub-board col 1 {:?}", SCol::C2);

    println!("sub-board move {:?}", SMove {
        coord: SCoord{row: SRow::R0, col: SCol::C2},
        player: Player::X
    });

    println!("player X : {:?}", Player::X);
    println!("player O : {:?}", Player::O);

    println!("slot - empty : {:?}", Slot::Empty);
    println!("slot - taken by X : {:?}", Slot::Taken(Player::X));

    println!("initial game : {:?}", init_game());

    println!("slot(empty SBoard) : {:?}", slot(SBoard::empty()));

    println!("play() : {:?}",
             play(SBoard::empty(),
                  SMove{ coord: SCoord { row: SRow::R1, col: SCol::C1 },
                         player: Player::X}));
}

fn main() {
    print_examples();
}
