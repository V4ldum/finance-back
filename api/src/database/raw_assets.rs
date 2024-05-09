use std::error::Error;

use sea_orm::ActiveValue::{Set, Unchanged};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::database::generated::prelude::RawAssets;
use crate::database::generated::raw_assets;
use crate::database::Database;

impl Database {
    pub async fn get_raw_assets(
        &self,
        id_user: i32,
    ) -> Result<Vec<raw_assets::Model>, Box<dyn Error>> {
        let raw_assets = RawAssets::find()
            .filter(raw_assets::Column::IdUser.eq(id_user))
            .all(&self.db)
            .await?;

        Ok(raw_assets)
    }

    pub async fn find_raw_asset(
        &self,
        asset_id: i32,
        user_id: i32,
    ) -> Result<Option<raw_assets::Model>, Box<dyn Error>> {
        let asset = RawAssets::find_by_id(asset_id)
            .filter(raw_assets::Column::IdUser.eq(user_id))
            .one(&self.db)
            .await?;

        Ok(asset)
    }

    pub async fn add_raw_asset(
        &self,
        name: String,
        possessed: i32,
        unit_weight: i32,
        composition: String,
        purity: i32,
        user_id: i32,
    ) -> Result<(), Box<dyn Error>> {
        let add_raw_asset = raw_assets::ActiveModel {
            name: Set(name),
            possessed: Set(possessed),
            unit_weight: Set(unit_weight),
            composition: Set(composition),
            purity: Set(purity),
            id_user: Set(user_id),
            ..Default::default()
        };

        RawAssets::insert(add_raw_asset).exec(&self.db).await?;

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn update_raw_asset(
        &self,
        id: i32,
        user_id: i32,
        name: Option<String>,
        possessed: Option<i32>,
        unit_weight: Option<i32>,
        composition: Option<String>,
        purity: Option<i32>,
    ) -> Result<(), Box<dyn Error>> {
        let update_raw_asset = raw_assets::ActiveModel {
            id: Unchanged(id),
            id_user: Unchanged(user_id),
            name: match name {
                Some(name) => Set(name),
                None => Default::default(),
            },
            possessed: match possessed {
                Some(possessed) => Set(possessed),
                None => Default::default(),
            },
            unit_weight: match unit_weight {
                Some(unit_weight) => Set(unit_weight),
                None => Default::default(),
            },
            composition: match composition {
                Some(composition) => Set(composition),
                None => Default::default(),
            },
            purity: match purity {
                Some(purity) => Set(purity),
                None => Default::default(),
            },
        };

        RawAssets::update(update_raw_asset).exec(&self.db).await?;

        Ok(())
    }

    pub async fn delete_raw_asset(&self, id: i32, user_id: i32) -> Result<(), Box<dyn Error>> {
        let delete_raw_asset = raw_assets::ActiveModel {
            id: Unchanged(id),
            id_user: Unchanged(user_id),
            ..Default::default()
        };

        RawAssets::delete(delete_raw_asset).exec(&self.db).await?;

        Ok(())
    }
}
