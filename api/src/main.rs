use std::error::Error;

use sea_orm::Database as SeaOrmDatabase;

use finance_api::database::Database;
use finance_api::run;
use finance_api::state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let database_url = dotenvy::var("DATABASE_URL")?;

    let database = SeaOrmDatabase::connect(database_url).await?;
    let state = AppState {
        database: Database::new(database),
    };

    run(state).await;

    Ok(())
}
