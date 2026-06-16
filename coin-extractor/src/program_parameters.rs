use secrecy::SecretString;
use sqlx::SqliteConnection;

pub struct ProgramParameters {
    pub numista_url: String,
    pub numista_api_key: SecretString,
    pub coin_id: u32,
    pub db: SqliteConnection,
}
