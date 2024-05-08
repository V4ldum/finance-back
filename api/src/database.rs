use sea_orm::DatabaseConnection;

mod cash_assets;
mod coin_assets;
mod coins;
pub(crate) mod generated;
mod raw_assets;
mod trade_values;
mod users;

#[derive(Clone)]
pub struct Database {
    db: DatabaseConnection,
}

impl Database {
    pub fn new(db: DatabaseConnection) -> Self {
        Database { db }
    }
}
