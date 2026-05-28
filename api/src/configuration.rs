use anyhow::Result;

pub struct Configuration {
    pub database_url: String,
    pub application_host: String,
    pub application_port: u16,
}

pub fn get_configuration() -> Result<Configuration> {
    dotenvy::dotenv()?;

    let database_url = dotenvy::var("DATABASE_URL")?;
    let application_host = dotenvy::var("HOST")?;
    let application_port = dotenvy::var("PORT")?;

    Ok(Configuration {
        database_url,
        application_host,
        application_port: application_port.parse::<u16>()?,
    })
}
