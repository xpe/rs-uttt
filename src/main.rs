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
        const ROW_0 = 0b0000000000011111,
        const ROW_1 = 0b0000001111100000,
        const ROW_2 = 0b0111110000000000,
    }
}

#[derive(Debug)]
enum Player { X, O }

#[derive(Debug)]
enum Slot { Taken(Player), Empty }

// A Move has a row and a column
bitflags! {
    flags Move: u8 {
        const ROW = 0b00001111,
        const COL = 0b11110000,
    }
}

// ---------- functions ----------

fn init_game() -> Game {
    Game { board: empty_board(), last_move: None }
}

fn empty_board() -> Board {
    Board([SBoard::empty(); 9])
}

#[allow(unused_variables)]
fn slot(sb: SBoard) -> Slot {
    Slot::Empty
}

fn print_examples() {
    println!("player X : {:?}", Player::X);
    println!("player O : {:?}", Player::O);
    println!("slot - empty : {:?}", Slot::Empty);
    println!("slot - taken by X : {:?}", Slot::Taken(Player::X));
    println!("initial game : {:?}", init_game());
    println!("slot(empty SBoard) : {:?}", slot(SBoard::empty()));
}

fn main() {
    print_examples();
}
