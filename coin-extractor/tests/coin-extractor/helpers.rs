use coin_extractor::program_parameters::ProgramParameters;
use fake::Fake;
use sqlx::{Connection, SqliteConnection};
use uuid::Uuid;

pub struct TestApp {
    pub params: ProgramParameters,
    pub db: SqliteConnection,
}

pub async fn test_app(url: String) -> TestApp {
    let db_url = format!("sqlite:file:memdb-{}?mode=memory&cache=shared", Uuid::new_v4());

    // Keep one connection alive for the whole test: a `mode=memory&cache=shared` database
    // only lives while at least one connection is open, and `run` drops the one it owns.
    // Migrations are applied through this connection and seen by `run` via the shared cache.
    let mut db = SqliteConnection::connect(&db_url)
        .await
        .expect("Failed to open database");
    sqlx::migrate!("../api/migrations")
        .run(&mut db)
        .await
        .expect("Failed to run migrations");

    let params = ProgramParameters {
        numista_url: format!("http://{url}/"),
        numista_api_key: (1000..9999).fake::<u32>().to_string(),
        coin_id: (u32::MIN..u32::MAX).fake(),
        db: SqliteConnection::connect(&db_url)
            .await
            .expect("Failed to open database"),
    };

    TestApp { params, db }
}
