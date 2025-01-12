use finance_api::state::AppState;
use finance_api::{run, Database};
use sqlx::SqlitePool;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let database_url = dotenvy::var("DATABASE_URL")?;

    let database = SqlitePool::connect(&database_url).await?;
    let state = AppState {
        database: Database::new(database),
    };

    run(state).await;

    Ok(())
}
