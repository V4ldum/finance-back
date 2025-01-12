use sqlx::{SqliteConnection};

pub struct ProgramParameters {
    pub api_key: String,
    pub coin_id: u32,
    pub db: SqliteConnection,
}
