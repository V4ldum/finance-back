use finance_api::state::AppState;
use finance_api::{run, Database};
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use std::error::Error;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let database_url = dotenvy::var("DATABASE_URL")?;

    let database =
        SqlitePool::connect_with(SqliteConnectOptions::from_str(&database_url)?.extension("libsqlite3_unaccent"))
            .await?;
    let state = AppState {
        database: Database::new(database),
    };

    run(state).await;
    Ok(())
}
