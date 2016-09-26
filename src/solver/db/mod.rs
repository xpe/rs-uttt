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

use data::{CI, Count, Game, Loc, Play, Player, RI};
use postgres::rows::{Row, Rows};
use postgres::{Connection, IntoConnectParams, SslMode};
use solver::{Outcome, Solution};

// == public API: table functions ==============================================

pub fn db_create(conn: &Connection) {
    conn.execute(CREATE_TABLE, &[]).unwrap();
}

pub fn db_drop(conn: &Connection) {
    conn.execute(DROP_TABLE, &[]).unwrap();
}

// == public API: connect / read / write =======================================

pub fn db_connect<T: IntoConnectParams> (params: T) -> Connection {
    Connection::connect(params, SslMode::None).unwrap()
}

// TODO: From the postgres crate documentation: "If the same statement will be
// repeatedly executed (perhaps with different query parameters), consider using
// the prepare and prepare_cached methods."
//
// let stmt =
//     conn.prepare_cached("UPDATE foo SET bar = $1 WHERE baz = $2").unwrap();
// for (bar, baz) in updates {
//     stmt.execute(&[bar, baz]).unwrap();
// }

/// Reads the database for a given game and depth. If no match is found, returns
/// None. If the retrieved game has an unknown outcome where turns is less than
/// depth, return None. Otherwise, return some solution.
pub fn db_read(conn: &Connection, game: &Game, depth: Count)
               -> Option<Solution> {
    match db_read_(conn, game) {
        Some(solution) => {
            match solution.outcome {
                Outcome::Unknown { turns: t } if t < depth => None,
                _ => Some(solution),
            }
        },
        None => None,
    }
}

/// Low-level read function.
fn db_read_(conn: &Connection, game: &Game) -> Option<Solution> {
    let game_cols: (i64, i64, i32) = game_columns_from(game);
    let game_1: i64 = game_cols.0;
    let game_2: i64 = game_cols.1;
    let game_3: i32 = game_cols.2;
    let rows: Rows = conn.query(
        "SELECT solution \
         FROM solution \
         WHERE game_1 = $1, game_2 = $2, game_3 = $3",
        &[&game_1, &game_2, &game_3]).unwrap();
    match rows.len() {
        0 => None,
        1 => {
            let row: Row = rows.get(0);
            let sol: i16 = row.get(0);
            Some(solution_from(sol, game.next_player()))
        },
        _ => panic!("Error 1143: expected 0 or 1 row"),
    }
}


/// Write to database, inserting or updating as appropriate. This function is
/// not responsible for testing if an overwrite is the 'sensible' thing to do;
/// e.g. a caller could overwrite `Outcome::Unknown { turns: 6 }` to
/// `Outcome::Unknown { turns : 3 }`.
pub fn db_write(conn: &Connection, game: &Game, solution: Solution) -> bool {
    let game_cols: (i64, i64, i32) = game_columns_from(game);
    let game_1: i64 = game_cols.0;
    let game_2: i64 = game_cols.1;
    let game_3: i32 = game_cols.2;
    let sol: i16 = sol_i16(solution);
    let rows_modified = conn.execute(
        "INSERT INTO solutions (game_1, game_2, game_3, solution) \
         ON CONFLICT DO UPDATE \
         VALUES ($1, $2, $3, $4)",
        &[&game_1, &game_2, &game_3, &sol]).unwrap();
    match rows_modified {
        0 => false,
        1 => true,
        _ => panic!("Error 6873"),
    }
    // Test for success.
}

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

// == conversions (structs -> database values) =================================

/// Converts a Game struct to a 3-tuple (a triple) of types (i64, i64, i32)
/// suitable for the 'game1', 'game2', 'game3' columns in the 'solutions' table.
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

// TODO
/// Converts a Solution to a 16-bit integer.
#[allow(unused_variables)]
fn sol_i16(solution: Solution) -> i16 {
    unimplemented!()
}

// == helpers for conversions (structs -> database values) =====================

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

// == conversions (database values -> structs) =================================

/// Converts the 'solution' column (an i16) in the 'solution' table to
/// a Solution struct.
fn solution_from(sol: i16, player: Option<Player>) -> Solution {
    let x: u16 = sol as u16;
    let outcome: u8 = (x >> 14 & 3) as u8;
    let location: u8 = (x >> 7 & 0x7F) as u8;
    let turns: u8 = (x & 0x7F) as u8;
    let opt_play: Option<Play> = match opt_loc_from(location) {
        Some(loc) => {
            Some(Play {
                loc: loc,
                player: player.expect("Error 2294: expected some player")
            })
        },
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
        _ => panic!("Error 4771"),
    }
}

// == helper conversions (database values -> structs) ==========================

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

        9 => Some(Loc::new(RI::R1, CI::C0)),
        10 => Some(Loc::new(RI::R1, CI::C1)),
        11 => Some(Loc::new(RI::R1, CI::C2)),
        12 => Some(Loc::new(RI::R1, CI::C3)),
        13 => Some(Loc::new(RI::R1, CI::C4)),
        14 => Some(Loc::new(RI::R1, CI::C5)),
        15 => Some(Loc::new(RI::R1, CI::C6)),
        16 => Some(Loc::new(RI::R1, CI::C7)),
        17 => Some(Loc::new(RI::R1, CI::C8)),

        18 => Some(Loc::new(RI::R2, CI::C0)),
        19 => Some(Loc::new(RI::R2, CI::C1)),
        20 => Some(Loc::new(RI::R2, CI::C2)),
        21 => Some(Loc::new(RI::R2, CI::C3)),
        22 => Some(Loc::new(RI::R2, CI::C4)),
        23 => Some(Loc::new(RI::R2, CI::C5)),
        24 => Some(Loc::new(RI::R2, CI::C6)),
        25 => Some(Loc::new(RI::R2, CI::C7)),
        26 => Some(Loc::new(RI::R2, CI::C8)),

        27 => Some(Loc::new(RI::R3, CI::C0)),
        28 => Some(Loc::new(RI::R3, CI::C1)),
        29 => Some(Loc::new(RI::R3, CI::C2)),
        30 => Some(Loc::new(RI::R3, CI::C3)),
        31 => Some(Loc::new(RI::R3, CI::C4)),
        32 => Some(Loc::new(RI::R3, CI::C5)),
        33 => Some(Loc::new(RI::R3, CI::C6)),
        34 => Some(Loc::new(RI::R3, CI::C7)),
        35 => Some(Loc::new(RI::R3, CI::C8)),

        36 => Some(Loc::new(RI::R4, CI::C0)),
        37 => Some(Loc::new(RI::R4, CI::C1)),
        38 => Some(Loc::new(RI::R4, CI::C2)),
        39 => Some(Loc::new(RI::R4, CI::C3)),
        40 => Some(Loc::new(RI::R4, CI::C4)),
        41 => Some(Loc::new(RI::R4, CI::C5)),
        42 => Some(Loc::new(RI::R4, CI::C6)),
        43 => Some(Loc::new(RI::R4, CI::C7)),
        44 => Some(Loc::new(RI::R4, CI::C8)),

        45 => Some(Loc::new(RI::R5, CI::C0)),
        46 => Some(Loc::new(RI::R5, CI::C1)),
        47 => Some(Loc::new(RI::R5, CI::C2)),
        48 => Some(Loc::new(RI::R5, CI::C3)),
        49 => Some(Loc::new(RI::R5, CI::C4)),
        50 => Some(Loc::new(RI::R5, CI::C5)),
        51 => Some(Loc::new(RI::R5, CI::C6)),
        52 => Some(Loc::new(RI::R5, CI::C7)),
        53 => Some(Loc::new(RI::R5, CI::C8)),

        54 => Some(Loc::new(RI::R6, CI::C0)),
        55 => Some(Loc::new(RI::R6, CI::C1)),
        56 => Some(Loc::new(RI::R6, CI::C2)),
        57 => Some(Loc::new(RI::R6, CI::C3)),
        58 => Some(Loc::new(RI::R6, CI::C4)),
        59 => Some(Loc::new(RI::R6, CI::C5)),
        60 => Some(Loc::new(RI::R6, CI::C6)),
        61 => Some(Loc::new(RI::R6, CI::C7)),
        62 => Some(Loc::new(RI::R6, CI::C8)),

        63 => Some(Loc::new(RI::R7, CI::C0)),
        64 => Some(Loc::new(RI::R7, CI::C1)),
        65 => Some(Loc::new(RI::R7, CI::C2)),
        66 => Some(Loc::new(RI::R7, CI::C3)),
        67 => Some(Loc::new(RI::R7, CI::C4)),
        68 => Some(Loc::new(RI::R7, CI::C5)),
        69 => Some(Loc::new(RI::R7, CI::C6)),
        70 => Some(Loc::new(RI::R7, CI::C7)),
        71 => Some(Loc::new(RI::R7, CI::C8)),

        72 => Some(Loc::new(RI::R8, CI::C0)),
        73 => Some(Loc::new(RI::R8, CI::C1)),
        74 => Some(Loc::new(RI::R8, CI::C2)),
        75 => Some(Loc::new(RI::R8, CI::C3)),
        76 => Some(Loc::new(RI::R8, CI::C4)),
        77 => Some(Loc::new(RI::R8, CI::C5)),
        78 => Some(Loc::new(RI::R8, CI::C6)),
        79 => Some(Loc::new(RI::R8, CI::C7)),
        80 => Some(Loc::new(RI::R8, CI::C8)),

        _ => panic!("Error 8689"),
    }
}
