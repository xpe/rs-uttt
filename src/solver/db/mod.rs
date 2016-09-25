/// Database.
///
/// Bit mapping for the 'game_1' (BIGINT = 64 bits) column:
///
/// bits      width   note
/// -------   -----   --------
/// 63 - 48      16   SBoard 3
/// 47 - 32      16   SBoard 2
/// 31 - 16      16   SBoard 1
/// 15 -  0      16   SBoard 0
///
/// Bit mapping for the 'game_2' (BIGINT = 64 bits) column:
///
/// bits      width   note
/// -------   -----   --------
/// 63 - 48      16   SBoard 7
/// 47 - 32      16   SBoard 6
/// 31 - 16      16   SBoard 5
/// 15 -  0      16   SBoard 4
///
/// Bit mapping for the 'game_3' (INT = 32 bits) column:
///
/// bit(s)    width   note
/// -------   -----   --------
/// 31 - 30       2   last player (0 = O, 1 = X, 2 = none)
/// 29 - 28       2   next player (0 = O, 1 = X, 2 = none)
/// 27 - 24       4   unused
/// 23 - 16       8   last location (see Loc.encoding)
/// 15 -  0      16   SBoard 8
///
/// Bit mapping for the 'solution' (SMALLINT = 16 bits) column:
///
/// bits       width   note
/// --------   -----   ----
/// 15 - 14       2   outcome (0 = ?; 1 = tie, 2 = O to win, 3 = X to win)
/// 13 -  7       7   location (0 to 80; 127 for none)
///  6 -  0       7   turns (0 to 81; 82 to 127 impossible)
///
/// * I would have preferred to encode the same information as the Solution
/// struct (Option<Play> + Outcome), which when expanded is (Option<Loc> +
/// Option<Player> + Outcome). However, I could not encode all of this in 16
/// bits, so I dropped the Player component. This is not a problem in context,
/// because the player can quickly calculated after retrieving the current
/// player from the 'game_3' column.

use data::*;
use postgres::Connection;
use solver::{Outcome, Solution};
// use postgres::rows::Rows;

// == PostgreSQL command strings ===============================================

/// Command to create the 'solutions' table.
///
/// column     PostgreSQL   Rust
/// ------     ----------   ----
/// game_1     BIGINT       i64
/// game_2     BIGINT       i64
/// game_3     INT          i32
/// solution   SMALLINT     i16
///
/// Note: game1, game2, game3 form a composite primary key. See below.
pub const CREATE_TABLE: &'static str =
    "CREATE TABLE solutions (\
       game_1    BIGINT    NOT NULL,\
       game_2    BIGINT    NOT NULL,\
       game_3    INT       NOT NULL,\
       solution  SMALLINT  NOT NULL,\
       PRIMARY KEY (game_1, game_2, game_3)
     );";

// In case the PRIMARY KEY does not work, use this:
// CREATE INDEX game_idx ON solutions (game1, game2, game3);

pub const DROP_TABLE: &'static str = "DROP TABLE IF EXISTS solutions;";

// == table functions ========================================================

pub fn create(conn: Connection) {
    conn.execute(CREATE_TABLE, &[]).unwrap();
}

pub fn drop(conn: Connection) {
    conn.execute(DROP_TABLE, &[]).unwrap();
}

// == conversion functions =====================================================

/// Converts the 'solution' column (an i16) in the 'solution' table to
/// a Solution struct.
#[allow(dead_code)]
fn solution_from(sol: i16, player: Player) -> Solution {
    let x: u16 = sol as u16;
    let outcome: u8 = (x >> 14 & 3) as u8;
    let location: u8 = (x >> 7 & 0x7F) as u8;
    let turns: u8 = (x & 0x7F) as u8;
    let opt_play: Option<Play> = match opt_loc_from(location) {
        Some(loc) => Some(Play { loc: loc, player: player }),
        None => None,
    };
    match outcome {
        0 => Solution {
            outcome: Outcome::Unknown { turns: turns },
            opt_play: opt_play,
        },
        1 => Solution {
            outcome: Outcome::Tie { turns: turns },
            opt_play: opt_play,
        },
        2 => Solution {
            outcome: Outcome::Win { turns: turns, player: Player::O },
            opt_play: opt_play,
        },
        3 => Solution {
            outcome: Outcome::Win { turns: turns, player: Player::X },
            opt_play: opt_play,
        },
        _ => panic!("Internal error"),
    }
}

// TODO
fn opt_loc_from(x: u8) -> Option<Loc> {
    match x {
        127 => None,
        0 => Some(Loc::new(RI::R0, CI::C0)),
        1 => Some(Loc::new(RI::R0, CI::C1)),
        2 => Some(Loc::new(RI::R0, CI::C2)),
        3 => Some(Loc::new(RI::R0, CI::C3)),
        4 => Some(Loc::new(RI::R0, CI::C4)),
        5 => Some(Loc::new(RI::R0, CI::C5)),
        6 => Some(Loc::new(RI::R0, CI::C6)),
        7 => Some(Loc::new(RI::R0, CI::C7)),
        8 => Some(Loc::new(RI::R0, CI::C8)),
        // TODO
        _ => panic!("Internal error"),
    }
}

/// Converts a Game struct to a 3-tuple (a triple) of types (i64, i64, i32)
/// suitable for the 'game1', 'game2', 'game3' columns in the 'solutions' table.
#[allow(dead_code)]
fn game_columns_from(game: &Game) -> (i64, i64, i32) {
    let game_1: u64 =
        (game.board.sboards[3].encoding as u64) << 48 |
        (game.board.sboards[2].encoding as u64) << 32 |
        (game.board.sboards[1].encoding as u64) << 16 |
        (game.board.sboards[0].encoding as u64);
    let game_2: u64 =
        (game.board.sboards[7].encoding as u64) << 48 |
        (game.board.sboards[6].encoding as u64) << 32 |
        (game.board.sboards[5].encoding as u64) << 16 |
        (game.board.sboards[4].encoding as u64);
    let last_player: Option<Player> = game.last_player();
    let game_3: u32 =
        player_u32(last_player) << 30 |
        player_u32(game.next_player_(last_player)) << 28 |
        last_location_u32(game) << 16 |
        (game.board.sboards[8].encoding as u32);
    (game_1 as i64, game_2 as i64, game_3 as i32)
}

/// Returns either a location's encoding (8 bits) or 127
fn last_location_u32(game: &Game) -> u32 {
    match game.last_loc {
        Some(loc) => loc.encoding as u32,
        None => 127 as u32,
    }
}

/// Returns either 0, 1, or 2 for a given optional player.
fn player_u32(opt_player: Option<Player>) -> u32 {
    match opt_player {
        Some(Player::O) => 0,
        Some(Player::X) => 1,
        None => 2,
    }
}
