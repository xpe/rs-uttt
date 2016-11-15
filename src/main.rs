extern crate rand;
extern crate uttt;

use uttt::runners::*;
use uttt::solver::*;
use uttt::solver::db::db_connect;

fn main() {
    let conn_str = "postgres://xpe:xpe_0987@localhost";
    let conn = db_connect(conn_str);
    let stack = SSD_CPU_Stack::new(&conn);
    let mut rng = make_rng();
    run_ongoing_backwards_solve(true, &stack, &mut rng, 11, 7, true);
}
