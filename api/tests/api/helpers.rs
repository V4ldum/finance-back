use std::path::Path;
use std::process::Command;
use std::sync::LazyLock;

use api::{
    configuration::Configuration,
    startup::{Application, get_connection_pool},
    telemetry::{SubscriberConfig, get_subscriber, init_subscriber},
};
use chrono::Utc;
use fake::{Fake, faker::lorem::en::Sentence};
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
        let mut client = reqwest::Client::new().get(format!("{}/assets", self.address));

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

    pub async fn delete_raw_asset(&self, id: i64) -> reqwest::Response {
        reqwest::Client::new()
            .delete(format!("{}/assets/raw/{}", self.address, id))
            .header("X-API-KEY", "123")
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn patch_raw_asset(&self, id: i64, body: &serde_json::Value) -> reqwest::Response {
        reqwest::Client::new()
            .patch(format!("{}/assets/raw/{}", self.address, id))
            .header("X-API-KEY", "123")
            .json(body)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn get_raw_asset(&self, id: i64) -> reqwest::Response {
        reqwest::Client::new()
            .get(format!("{}/assets/raw/{}", self.address, id))
            .header("X-API-KEY", "123")
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

    pub async fn delete_cash_asset(&self, id: i64) -> reqwest::Response {
        reqwest::Client::new()
            .delete(format!("{}/assets/cash/{}", self.address, id))
            .header("X-API-KEY", "123")
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn patch_cash_asset(&self, id: i64, body: &serde_json::Value) -> reqwest::Response {
        reqwest::Client::new()
            .patch(format!("{}/assets/cash/{}", self.address, id))
            .header("X-API-KEY", "123")
            .json(body)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn get_cash_asset(&self, id: i64) -> reqwest::Response {
        reqwest::Client::new()
            .get(format!("{}/assets/cash/{}", self.address, id))
            .header("X-API-KEY", "123")
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_coin_asset(&self, body: &serde_json::Value) -> reqwest::Response {
        reqwest::Client::new()
            .post(format!("{}/assets/coin", self.address))
            .header("X-API-KEY", "123")
            .json(body)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn delete_coin_asset(&self, id: i64) -> reqwest::Response {
        reqwest::Client::new()
            .delete(format!("{}/assets/coin/{}", self.address, id))
            .header("X-API-KEY", "123")
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn patch_coin_asset(&self, id: i64, body: &serde_json::Value) -> reqwest::Response {
        reqwest::Client::new()
            .patch(format!("{}/assets/coin/{}", self.address, id))
            .header("X-API-KEY", "123")
            .json(body)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn get_coin_asset(&self, id: i64) -> reqwest::Response {
        reqwest::Client::new()
            .get(format!("{}/assets/coin/{}", self.address, id))
            .header("X-API-KEY", "123")
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn get_price(&self, name: &str) -> reqwest::Response {
        reqwest::Client::new()
            .get(format!("{}/prices/{}", self.address, name))
            .header("X-API-KEY", "123")
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn get_prices(&self) -> reqwest::Response {
        reqwest::Client::new()
            .get(format!("{}/prices", self.address))
            .header("X-API-KEY", "123")
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn get_assets(&self) -> reqwest::Response {
        reqwest::Client::new()
            .get(format!("{}/assets", self.address))
            .header("X-API-KEY", "123")
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn get_coin(&self) -> reqwest::Response {
        reqwest::Client::new()
            .get(format!("{}/coins/1", self.address))
            .header("X-API-KEY", "123")
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn search_coins(&self, query: &str) -> reqwest::Response {
        reqwest::Client::new()
            .get(format!("{}/coins/search", self.address))
            .query(&[("q", query)])
            .header("X-API-KEY", "123")
            .send()
            .await
            .expect("Failed to execute request")
    }
}

pub fn name() -> String {
    Sentence(1..3).fake()
}

pub fn fake_id() -> i64 {
    (1..1000).fake()
}

pub fn possessed() -> i64 {
    (1..1000).fake()
}

pub fn unit_value() -> i64 {
    (1..1000).fake()
}

pub fn unit_weight() -> i64 {
    (1..1000).fake()
}

pub fn composition() -> String {
    ["GOLD", "SILVER"][(0..2).fake::<usize>()].to_string()
}

pub fn purity() -> i64 {
    (1..=9999).fake()
}

// Ensure the telemetry stack is only initialized once
static TRACING: LazyLock<()> = LazyLock::new(|| {
    let default_filter_level = LevelFilter::INFO;

    // If the environment variable TEST_LOG is set, output tracing to stdout, otherwise don't output it
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

async fn configure_database(pool: &SqlitePool) {
    // Insert a test user into the database
    sqlx::query!("INSERT OR IGNORE INTO users(api_key) VALUES ('123')")
        .execute(pool)
        .await
        .expect("Failed to insert user into database");

    // Insert fake prices into the database
    let gold = (2000..5000).fake::<i64>();
    let silver = (50..100).fake::<i64>();
    let sp = (5000..9000).fake::<i64>();
    let date = Utc::now().format("%Y-%m-%d").to_string();

    sqlx::query!(
        r#"
        INSERT OR IGNORE INTO prices
        VALUES ('Gold', $1, $4),
               ('Silver', $2, $4),
               ('SP500', $3, $4)
        "#,
        gold,
        silver,
        sp,
        date,
    )
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
