use std::error::Error;

use crate::database::tables::users::Users;
use crate::database::Database;

impl Database {
    pub async fn get_user(&self, key: &str) -> Result<Option<Users>, Box<dyn Error>> {
        let result = sqlx::query!("SELECT * FROM users WHERE id = $1", key)
            .fetch_optional(&self.db)
            .await?;

        Ok(result.map(|record| Users {
            id: record.id,
            api_key: record.api_key,
        }))
    }
}
