use crate::database::Database;
use axum::extract::FromRef;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub database: Database,
}
