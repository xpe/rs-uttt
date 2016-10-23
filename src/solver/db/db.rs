/// Database.

use data::*;
use postgres::{Connection, TlsMode};
use postgres::params::IntoConnectParams;
use postgres::rows::{Row as DataRow, Rows as DataRows};
use postgres::stmt::Statement;
use solver::{Outcome, Solution};
use std::collections::HashSet;

// == public API: table functions ==============================================

pub fn db_create_table(conn: &Connection) {
    let rows_modified = conn.execute(CREATE_TABLE, &[]).expect("E85011");
    if rows_modified != 0 { panic!("E85012") };
}

pub fn db_drop_table(conn: &Connection) {
    let rows_modified = conn.execute(DROP_TABLE, &[]).expect("E85021");
    if rows_modified != 0 { panic!("E85022") };
}

pub fn db_truncate_table(conn: &Connection) {
    let rows_modified = conn.execute(TRUNCATE_TABLE, &[]).expect("E85031");
    if rows_modified != 0 { panic!("E85032") };
}

pub fn db_create_indexes(conn: &Connection) {
    for command in CREATE_INDEXES {
        let rows_modified = conn.execute(command, &[]).expect("E85041");
        if rows_modified != 0 { panic!("E85042") };
    }
}

// == public API: connect / read / write =======================================

pub fn db_connect<T: IntoConnectParams> (params: T) -> Connection {
    Connection::connect(params, TlsMode::None).expect("E85051")
}

pub fn db_read_stmt<'c>(conn: &'c Connection) -> Statement<'c> {
    conn.prepare(
        "SELECT solutions \
         FROM solutions \
         WHERE game_1 = $1 AND game_2 = $2 AND game_3 = $3").expect("E85061")
}

/// Read function.
pub fn db_read(stmt: &Statement, game: &Game) -> Vec<Solution> {
    let (game_1, game_2, game_3): (i64, i64, i32) = game_columns_from(game);
    let rows: DataRows = stmt.query(
        &[&game_1, &game_2, &game_3]).expect("E85062");
    match rows.len() {
        0 => vec![],
        1 => {
            let next_player: Option<Player> = game.next_player();
            let row: DataRow = rows.get(0);
            let solutions: Vec<i16> = row.get(0);
            solutions.iter()
                .map(|sol| solution_from(*sol, next_player))
                .collect::<Vec<Solution>>()
        },
        _ => panic!("E85063"),
    }
}

pub fn db_write_stmt<'c>(conn: &'c Connection) -> Statement<'c> {
    conn.prepare(
        "INSERT INTO solutions
         (game_1, game_2, game_3, plays, solutions, sol_turns, unknown) \
         VALUES ($1, $2, $3, $4, $5, $6, $7) \
         ON CONFLICT (game_1, game_2, game_3) \
         DO UPDATE \
         SET (plays, solutions, sol_turns, unknown) = ($4, $5, $6, $7) \
         WHERE \
         solutions.game_1 = $1 AND \
         solutions.game_2 = $2 AND \
         solutions.game_3 = $3").expect("E85071")
}

/// Write to database, inserting or updating as appropriate. This function is
/// not responsible for testing if an overwrite is the 'sensible' thing to do;
/// e.g. a caller could overwrite `Outcome::Unknown { turns: 6 }` to
/// `Outcome::Unknown { turns : 3 }`.
pub fn db_write(stmt: &Statement, game: &Game, sols: &Vec<Solution>) -> bool {
    let (game_1, game_2, game_3): (i64, i64, i32) = game_columns_from(game);
    let solutions: Vec<i16> = sols.into_iter()
        .map(|sol| sol_i16(*sol))
        .collect::<Vec<i16>>();
    let plays: i16 = game.board.play_count() as i16;
    let (sol_turns, unknown) = turns_and_unknown(sols);
    let rows_modified = stmt.execute(
        &[&game_1, &game_2, &game_3, &plays, &solutions, &sol_turns, &unknown])
        .expect("E85072");
    match rows_modified {
        0 => false,
        1 => true,
        _ => panic!("E85073"),
    }
}

/// Returns a tuple with two elements:
/// 1. The (shared / equal) turns value in each of the vector of solutions.
/// 2. A boolean indicating if each of the solutions is unknown. (If one
///    is unknown, then they must all be unknown)
pub fn turns_and_unknown(solutions: &Vec<Solution>) -> (i16, bool) {
    let mut turns_set: HashSet<Count> = HashSet::new();
    let mut wins: u32 = 0;
    let mut ties: u32 = 0;
    let mut unknowns: u32 = 0;
    for solution in solutions {
        let turns = match solution.outcome {
            Outcome::Win { player: _, turns: t } => { wins += 1; t },
            Outcome::Tie { turns: t } => { ties += 1; t },
            Outcome::Unknown { turns: t } => { unknowns += 1; t },
        };
        turns_set.insert(turns);
    }
    if turns_set.len() == 1 {
        let turns: i16 = turns_set.into_iter().next().expect("E85081") as i16;
        if unknowns > 0 && (ties > 0 || wins > 0) { panic!("E85082"); }
        (turns, unknowns > 0)
    } else {
        panic!("E85083");
    }
}

// == internal database read/write functions ===================================

// TODO: From the postgres crate documentation: "If the same statement will be
// repeatedly executed (perhaps with different query parameters), consider using
// the prepare and prepare_cached methods."

// == PostgreSQL command strings ===============================================

/// Command to create the 'solutions' table.
///
/// Mapping between PostgreSQL and Rust types:
///
/// column      PostgreSQL   Rust
/// ------      ----------   ----
/// game_1      BIGINT       i64
/// game_2      BIGINT       i64
/// game_3      INT          i32
/// plays       SMALLINT     i16
/// solutions   SMALLINT[]   i16
/// sol_turns   SMALLINT     i16
/// unknown     BOOLEAN      bool
///
/// Note: game_1, game_2, game_3 form a composite primary key.
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
pub const CREATE_TABLE: &'static str =
    "CREATE TABLE IF NOT EXISTS solutions (\
       game_1     BIGINT      NOT NULL, \
       game_2     BIGINT      NOT NULL, \
       game_3     INT         NOT NULL, \
       plays      SMALLINT    NOT NULL, \
       solutions  SMALLINT[]  NOT NULL, \
       sol_turns  SMALLINT    NOT NULL, \
       unknown    BOOLEAN     NOT NULL, \
       PRIMARY KEY (game_1, game_2, game_3) USING INDEX TABLESPACE uttt_1 \
     ) TABLESPACE uttt_1";

pub const CREATE_INDEXES: &'static [ &'static str ] = &[
    "CREATE INDEX IF NOT EXISTS solutions_plays_idx \
    ON solutions (plays) TABLESPACE uttt_1",

    "CREATE INDEX IF NOT EXISTS solutions_sol_turns_idx \
    ON solutions (sol_turns) TABLESPACE uttt_1",

    "CREATE INDEX IF NOT EXISTS solutions_unknown_idx \
    ON solutions (unknown) TABLESPACE uttt_1",
];

pub const DROP_TABLE: &'static str = "DROP TABLE IF EXISTS solutions";

pub const TRUNCATE_TABLE: &'static str = "TRUNCATE TABLE solutions";

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

/// Converts a Solution to a 16-bit integer.
fn sol_i16(solution: Solution) -> i16 {
    let x_outcome: u16 = match solution.outcome {
        Outcome::Unknown { .. } => 0,
        Outcome::Tie { .. } => 1,
        Outcome::Win { player: Player::O, .. } => 2,
        Outcome::Win { player: Player::X, .. } => 3,
    };
    let x_location: u16 = location_u16(solution.opt_play);
    let x_turns: u16 = solution.outcome.turns() as u16;
    let x: u16 = x_outcome << 14 | x_location << 7 | x_turns;
    x as i16
}

// == helpers for conversions (structs -> database values) =====================

/// Returns either a location's encoding (8 bits) or 127
fn last_location_u32(game: &Game) -> u32 {
    match game.last_loc {
        Some(loc) => loc.encoding as u32,
        None => 0xFF,
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

fn location_u16(opt_play: Option<Play>) -> u16 {
    match opt_play {
        None => 0x7F,
        Some(play) => {
            let location: Loc = play.loc;
            let row_idx = location.row().as_u8();
            let col_idx = location.col().as_u8();
            (row_idx * 9 + col_idx) as u16
        }
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
                player: player.expect("E85091")
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
        _ => panic!("E85092"),
    }
}

// == helper conversions (database values -> structs) ==========================

/// Converts from an 8-bit unsigned integer to an optional location.
fn opt_loc_from(x: u8) -> Option<Loc> {
    // TODO: Use math instead of this dreadful pattern match.
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
        _ => panic!("E85101"),
    }
}
