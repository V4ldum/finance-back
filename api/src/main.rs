use anyhow::Result;
use api::configuration::get_configuration;
use api::startup::Application;
use api::telemetry::get_subscriber;
use api::telemetry::init_subscriber;
use tracing::level_filters::LevelFilter;

#[tokio::main]
async fn main() -> Result<()> {
    // Setup telemetry
    let subscriber = get_subscriber(LevelFilter::INFO);
    init_subscriber(subscriber);

    // Read configuration from environment variables
    let configuration = get_configuration().expect("Failed to read configuration");

    // Run the server
    let server = Application::build(configuration).await?;
    server.run_until_stopped().await?;

    Ok(())
}
