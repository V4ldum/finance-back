use std::path::Path;
use std::process::Command;
use std::sync::LazyLock;

use api::{
    configuration::Configuration,
    startup::{Application, get_connection_pool},
    telemetry::{SubscriberConfig, get_subscriber, init_subscriber},
};
use tracing::level_filters::LevelFilter;
use uuid::Uuid;

mod fakes;
mod test_app;

pub use fakes::*;
pub use test_app::{TestApp, TestUser};

// Ensure the telemetry stack is only initialized once
static TRACING: LazyLock<()> = LazyLock::new(|| {
    let default_filter_level = LevelFilter::INFO;

    // If the environment variable TEST_LOG is set, output INFO logs, otherwise don't output anything
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(SubscriberConfig {
            json_filter: default_filter_level,
            json_sink: std::io::sink,
            text_filter: default_filter_level,
            text_sink: std::io::stdout,
        });
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(SubscriberConfig {
            json_filter: default_filter_level,
            json_sink: std::io::sink,
            text_filter: default_filter_level,
            text_sink: std::io::sink,
        });
        init_subscriber(subscriber);
    }
});

// Build the SQLite extension and return the absolute path to the resulting library.
// This guarantees the artifact exists before any test.
static UNACCENT_EXTENSION: LazyLock<String> = LazyLock::new(|| -> String {
    // The extension is its own workspace (excluded from the main one), so it must be
    // built through its own manifest and lands in its own target directory.
    let extension_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("api crate should have a parent directory")
        .join("sqlite3_unaccent");

    let status = Command::new(env!("CARGO"))
        .arg("build")
        .arg("--manifest-path")
        .arg(extension_dir.join("Cargo.toml"))
        .status()
        .expect("Failed to run cargo to build the sqlite3_unaccent extension");
    assert!(status.success(), "Failed to build the sqlite3_unaccent extension");

    // The cdylib name is platform specific: libsqlite3_unaccent.so / sqlite3_unaccent.dll
    let library = format!(
        "{}sqlite3_unaccent{}",
        std::env::consts::DLL_PREFIX,
        std::env::consts::DLL_SUFFIX
    );
    let path = extension_dir.join("target").join("debug").join(library);
    assert!(path.exists(), "Built extension was not found at {}", path.display());

    path.into_os_string()
        .into_string()
        .expect("Extension path should be valid UTF-8")
});

pub async fn spawn_app() -> TestApp {
    // Setup telemetry
    LazyLock::force(&TRACING);

    // Create the configuration
    let configuration = Configuration {
        database_url: format!("sqlite:file:memdb-{}?mode=memory&cache=shared", Uuid::new_v4()),
        application_host: "127.0.0.1".to_string(),
        application_port: 0, // Random OS port
        sqlite_extension: LazyLock::force(&UNACCENT_EXTENSION).to_owned(),
    };

    // Set up the database pool used by tests
    let pool = get_connection_pool(&configuration.database_url, &configuration.sqlite_extension)
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
