use std::error::Error;

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::database::Database;
use crate::database::generated::prelude::Users;
use crate::database::generated::users;
use crate::database::generated::users::Model as UsersModel;

impl Database {
    pub async fn get_user(&self, key: &str) -> Result<Option<UsersModel>, Box<dyn Error>> {
        let result = Users::find()
            .filter(users::Column::ApiKey.eq(key))
            .one(&self.db)
            .await?;

        Ok(result)
    }
}
