use anyhow::{Context, Result};

pub struct Configuration {
    pub database_url: String,
    pub application_host: String,
    pub application_port: u16,
}

pub fn get_configuration() -> Result<Configuration> {
    // dotenvy returns an error if the .env file is not found
    // We don't use a .env in production so we need to ignore it
    let _ = dotenvy::dotenv();

    let database_url = dotenvy::var("DATABASE_URL").context("Failed to read DATABASE_URL")?;
    let application_host = dotenvy::var("HOST").context("Failed to read HOST")?;
    let application_port = dotenvy::var("PORT").context("Failed to read PORT")?;

    Ok(Configuration {
        database_url,
        application_host,
        application_port: application_port.parse::<u16>()?,
    })
}
