/// SSD Device.

use data::*;
use postgres::Connection;
use solver::*;
use solver::db::*;

pub const CONNECT : &'static str = "postgres://postgres@localhost";


pub struct SSD {
    conn: Connection,
}

impl SSD {
    pub fn new() -> SSD {
        SSD { conn: db_conn(CONNECT) }
    }
}

impl Device for SSD {
    fn read(&self, game: &Game, depth: Count) -> Option<Solution> {
        db_read(&self.conn, game, depth)
    }

    fn write(&self, game: &Game, solution: Solution) -> bool {
        db_write(&self.conn, game, solution)
    }

    fn is_writable(&self) -> bool {
        true
    }

    fn label(&self) -> &str {
        "SSD"
    }
}
