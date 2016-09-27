/// SSD Device.

use data::*;
use postgres::Connection;
use solver::*;
use solver::db::*;

pub const CONNECTION_STRING: &'static str =
    "postgres://xpe@localhost";

pub struct SSD {
    conn: Connection,
}

impl SSD {
    pub fn new() -> SSD {
        let conn = db_connect(CONNECTION_STRING);
        // db_drop_table(&conn);
        db_create_table(&conn);
        SSD { conn: conn }
    }
}

impl Device for SSD {
    #[allow(unused_variables)]
    fn compute(&self, game: &Game, depth: Count) -> Option<Solution> {
        None
    }

    fn read(&self, game: &Game) -> Option<Solution> {
        db_read(&self.conn, game)
    }

    fn write(&self, game: &Game, solution: Solution) -> bool {
        db_write(&self.conn, game, solution)
    }

    fn supports_compute(&self) -> bool { false }
    fn supports_read(&self) -> bool { true }
    fn supports_write(&self) -> bool { true }

    fn label(&self) -> &str {
        "SSD"
    }
}
