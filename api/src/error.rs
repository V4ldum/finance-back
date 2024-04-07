use serde::Serialize;

#[derive(Serialize)]
pub struct APIError {
    reason: String,
}

impl APIError {
    pub fn database_error() -> Self {
        APIError {
            reason: String::from("Database error"),
        }
    }
    pub fn unknown_query(query: &str) -> Self {
        APIError {
            reason: format!("Unknown query : {query}"),
        }
    }
    pub fn no_api_key() -> Self {
        APIError {
            reason: String::from(r#""key" parameter is required"#),
        }
    }
    pub fn bad_api_key() -> Self {
        APIError {
            reason: String::from("Invalid API Key"),
        }
    }
}
