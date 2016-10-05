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
        .pool_size(3)
        .connection_timeout(Duration::new(5, 0))
        .build();
    let manager = PostgresConnectionManager::new(params, SslMode::None)
        .expect("E9301");
    Pool::new(config, manager).expect("E9302")
}

/// Read from database.
pub fn pool_read(pool: &PGPool, game: &Game) -> Option<Solution> {
    let conn = pool.get().expect("E9303");
    db_read(&conn, game)
}

/// Write to database.
pub fn pool_write(pool: &PGPool, game: &Game, solution: Solution) -> bool {
    db_write(&conn, game, solution)
    let conn = pool.get().expect("E9304");
}
