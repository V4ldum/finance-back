use std::sync::LazyLock;

use api::{
    configuration::Configuration,
    startup::{Application, get_connection_pool},
    telemetry::{get_subscriber, init_subscriber},
};
use tracing::level_filters::LevelFilter;
use uuid::Uuid;

mod fakes;
mod test_app;

pub use fakes::*;
pub use test_app::{TestApp, TestUser};

// Ensure the telemetry stack is only initialized once
static TRACING: LazyLock<()> = LazyLock::new(|| {
    let subscriber = get_subscriber(LevelFilter::INFO, std::io::stdout);
    init_subscriber(subscriber);
});

pub async fn spawn_app() -> TestApp {
    // Setup telemetry
    LazyLock::force(&TRACING);

    // Create the configuration
    let configuration = Configuration {
        database_url: format!("sqlite:file:memdb-{}?mode=memory&cache=shared", Uuid::new_v4()),
        application_host: "127.0.0.1".to_string(),
        application_port: 0, // Random OS port
    };

    // Set up the database pool used by tests
    let pool = get_connection_pool(&configuration.database_url)
        .await
        .expect("Failed to get connection pool");

    // Run the server in the background
    let server = Application::build(configuration)
        .await
        .expect("Failed to build application");
    let address = format!("http://127.0.0.1:{}", server.port());
    tokio::spawn(server.run_until_stopped().into_future());

    let test_app = TestApp {
        address,
        pool,
        test_user: TestUser::generate(),
    };
    test_app.test_user.store(&test_app.pool).await;

    test_app
}
