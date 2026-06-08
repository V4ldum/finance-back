use std::path::Path;
use std::process::Command;
use std::sync::LazyLock;

use api::{
    Application, Configuration, get_connection_pool,
    telemetry::{SubscriberConfig, get_subscriber, init_subscriber},
};
use sqlx::SqlitePool;
use tracing::level_filters::LevelFilter;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub pool: SqlitePool,
}

impl TestApp {
    pub async fn get_healthcheck(&self) -> reqwest::Response {
        reqwest::Client::new()
            .get(format!("{}/health", self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn get_auth_middleware(&self, api_key: Option<&str>) -> reqwest::Response {
        let mut client = reqwest::Client::new().get(format!("{}/trade_values", self.address));

        if let Some(api_key) = api_key {
            client = client.header("X-API-KEY", api_key);
        }

        client.send().await.expect("Failed to execute request")
    }

    pub async fn post_raw_asset(&self, body: &serde_json::Value) -> reqwest::Response {
        reqwest::Client::new()
            .post(format!("{}/assets/raw", self.address))
            .header("X-API-KEY", "123")
            .json(body)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_cash_asset(&self, body: &serde_json::Value) -> reqwest::Response {
        reqwest::Client::new()
            .post(format!("{}/assets/cash", self.address))
            .header("X-API-KEY", "123")
            .json(body)
            .send()
            .await
            .expect("Failed to execute request")
    }
}

// Ensure the telemetry stack is only initialized once
static TRACING: LazyLock<()> = LazyLock::new(|| {
    let default_filter_level = LevelFilter::INFO;
    let subscriber_name = "test".to_string();

    // If the environment variable TEST_LOG is set, output tracing to stdout, otherwise don't output it
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(SubscriberConfig {
            service: subscriber_name,
            json_filter: default_filter_level,
            json_sink: std::io::sink,
            text_filter: default_filter_level,
            text_sink: std::io::stdout,
        });
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(SubscriberConfig {
            service: subscriber_name,
            json_filter: default_filter_level,
            json_sink: std::io::sink,
            text_filter: default_filter_level,
            text_sink: std::io::sink,
        });
        init_subscriber(subscriber);
    };
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

async fn configure_database(pool: &SqlitePool) {
    // Insert a test user into the database
    sqlx::query!("INSERT OR IGNORE INTO users(api_key) VALUES ('123')")
        .execute(pool)
        .await
        .expect("Failed to insert user into database");
}

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
    configure_database(&pool).await;

    // Run the server in the background
    let server = Application::build(configuration)
        .await
        .expect("Failed to build application");
    let address = format!("http://127.0.0.1:{}", server.port());
    tokio::spawn(server.run_until_stopped().into_future());

    TestApp { address, pool }
}
