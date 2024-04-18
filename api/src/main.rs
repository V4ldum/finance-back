use std::error::Error;

use sea_orm::Database;

use finance_api::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let database_url = dotenvy::var("DATABASE_URL")?;

    let db = Database::connect(database_url).await?;

    run(db).await;

    Ok(())
}
