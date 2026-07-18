use std::env;
use std::process::exit;

use coin_extractor::program_parameters::ProgramParameters;
use coin_extractor::run;

use anyhow::{Context, Result};
use secrecy::SecretString;
use sqlx::migrate::MigrateDatabase;
use sqlx::{Connection, Sqlite, SqliteConnection};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;
    let api_key = SecretString::from(dotenvy::var("API_KEY").context("API_KEY not set")?);
    let database_url = dotenvy::var("DATABASE_URL").context("DATABASE_URL not set")?;

    let mut args = env::args();
    let program_name = args.next().unwrap_or_default();
    let (Some(first_arg), None) = (args.next(), args.next()) else {
        eprintln!("Usage : {program_name} <numista_coin_id>");
        exit(1);
    };

    let Ok(coin_id) = first_arg.parse() else {
        eprintln!("Invalid argument {first_arg}, argument should be a number");
        exit(1);
    };

    assert!(
        Sqlite::database_exists(&database_url).await.unwrap_or(false),
        "Database not found"
    );

    let db = SqliteConnection::connect(&database_url).await?;

    let params = ProgramParameters {
        numista_url: "https://api.numista.com/api/v3/types/".to_string(),
        numista_api_key: api_key,
        coin_id,
        db,
    };

    run(params).await.context("Failed to run")
}
