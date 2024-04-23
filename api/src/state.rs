use axum::extract::FromRef;

use crate::database::Database;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub database: Database,
}
