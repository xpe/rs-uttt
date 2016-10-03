/// Database Connection Pool.

use data::*;
use postgres::IntoConnectParams;
use r2d2::{Config, Pool};
use r2d2_postgres::{PostgresConnectionManager, SslMode};
use solver::db::db::*;
use solver::Solution;
use std::time::Duration;

pub type PGPool = Pool<PostgresConnectionManager>;

// == public API ===============================================================

pub fn pool_new<T: IntoConnectParams>(params: T) -> PGPool {
    let config = Config::builder()
        .pool_size(10)
        .connection_timeout(Duration::new(5, 0))
        .build();
    let manager = PostgresConnectionManager::new(params, SslMode::None)
        .expect("Error 9990");
    Pool::new(config, manager).expect("Error 9990")
}

/// Read from database.
pub fn pool_read(pool: &PGPool, game: &Game) -> Option<Solution> {
    let conn = pool.get().expect("Error 9991");
    db_read(&conn, game)
}

/// Write to database.
pub fn pool_write(pool: &PGPool, game: &Game, solution: Solution) -> bool {
    let conn = pool.get().expect("Error 9992");
    db_write(&conn, game, solution)
}
