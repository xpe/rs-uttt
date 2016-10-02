/// SSD Device.

use data::*;
use solver::*;
use solver::db::*;

pub const CONN_STR: &'static str = "postgres://xpe:xpe_0987@localhost";

pub struct SSD {}

impl SSD {
    pub fn new() -> Device {
        let conn = db_connect(CONN_STR);
        db_create_table(&conn);
        Device {
            compute: SSD::compute,
            read: SSD::read,
            write: SSD::write,
            has_compute: false,
            has_read: true,
            has_write: true,
            conn: Some(conn),
        }
    }

    #[allow(unused_variables)]
    fn compute(game: &Game, depth: Count, stack: &Stack) -> Option<Solution> {
        unimplemented!();
    }

    fn read(device: &Device, game: &Game) -> Option<Solution> {
        match device.conn {
            Some(ref conn) => db_read(conn, game),
            None => panic!("Error 6523"),
        }
    }

    fn write(device: &Device, game: &Game, solution: Solution) -> bool {
        match device.conn {
            Some(ref conn) => db_write(conn, game, solution),
            None => panic!("Error 0401"),
        }
    }

}
