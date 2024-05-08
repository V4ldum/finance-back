use std::error::Error;

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use sea_orm::ActiveValue::Set;

use crate::database::Database;
use crate::database::generated::cash_assets;
use crate::database::generated::prelude::CashAssets;

impl Database {
    pub async fn get_cash_assets(
        &self,
        id_user: i32,
    ) -> Result<Vec<cash_assets::Model>, Box<dyn Error>> {
        let cash_assets = CashAssets::find()
            .filter(cash_assets::Column::IdUser.eq(id_user))
            .all(&self.db)
            .await?;

        Ok(cash_assets)
    }

    pub async fn find_cash_asset(
        &self,
        asset_id: i32,
        user_id: i32,
    ) -> Result<Option<cash_assets::Model>, Box<dyn Error>> {
        let asset = CashAssets::find_by_id(asset_id)
            .filter(cash_assets::Column::IdUser.eq(user_id))
            .one(&self.db)
            .await?;

        Ok(asset)
    }

    pub async fn add_cash_asset(
        &self,
        name: String,
        possessed: i32,
        unit_value: i32,
        user_id: i32,
    ) -> Result<(), Box<dyn Error>> {
        let add_cash_asset = cash_assets::ActiveModel {
            name: Set(name),
            possessed: Set(possessed),
            unit_value: Set(unit_value),
            id_user: Set(user_id),
            ..Default::default()
        };

        CashAssets::insert(add_cash_asset).exec(&self.db).await?;

        Ok(())
    }
}
