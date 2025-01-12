use sqlx::SqlitePool;

mod cash_assets;
mod coin_assets;
mod coins;
mod raw_assets;
pub mod tables;
mod trade_values;
mod users;

#[derive(Clone)]
pub struct Database {
    db: SqlitePool,
}

impl Database {
    pub fn new(db: SqlitePool) -> Self {
        Database { db }
    }
}
