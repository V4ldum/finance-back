use std::env;
use std::error::Error;
use std::process::exit;

use coin_extractor::program_parameters::ProgramParameters;
use coin_extractor::run;

use sqlx::migrate::MigrateDatabase;
use sqlx::{Connection, Sqlite, SqliteConnection};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let api_key = dotenvy::var("API_KEY")?;
    let database_url = dotenvy::var("DATABASE_URL")?;

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage : {} <numista_coin_id>", args[0]);
        exit(1);
    }

    let first_arg = &args[1];
    let Ok(coin_id) = first_arg.parse() else {
        eprintln!("Invalid argument {}, argument should be a number", first_arg);
        exit(1);
    };

    if !Sqlite::database_exists(&database_url).await.unwrap_or(false) {
        panic!("Database not found")
    }

    let db = SqliteConnection::connect(&database_url).await?;

    let params = ProgramParameters { coin_id, api_key, db };

    run(params).await
}
