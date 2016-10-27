extern crate rand;
extern crate uttt;
extern crate chan;

use uttt::runners::*;
use uttt::solver::*;
use uttt::solver::db::db_connect;

fn main() {
    let conn_str = "postgres://xpe:xpe_0987@localhost";
    let conn = db_connect(conn_str);
    let stack = SSD_CPU_Stack::new(&conn);
    let mut rng = make_rng();
    let (_tx_quit, rx_quit) = chan::sync(0);
    run_ongoing_backwards_solve(true, &stack, rx_quit, &mut rng, 9, 6, true);
}
