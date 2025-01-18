use std::error::Error;

use crate::database::tables::users::Users;
use crate::database::Database;

impl Database {
    pub async fn get_user(&self, key: &str) -> Result<Option<Users>, Box<dyn Error>> {
        let result = sqlx::query_as!(Users, "SELECT * FROM users WHERE api_key = $1", key)
            .fetch_optional(&self.db)
            .await?;

        Ok(result)
    }
}
