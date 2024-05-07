//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "coins")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub numista_id: String,
    pub name: String,
    #[sea_orm(column_type = "Double")]
    pub weight: f64,
    #[sea_orm(column_type = "Double")]
    pub size: f64,
    #[sea_orm(column_type = "Double", nullable)]
    pub thickness: Option<f64>,
    pub min_year: String,
    pub max_year: Option<String>,
    pub composition: String,
    pub purity: i32,
    pub obverse: Option<i32>,
    pub reverse: Option<i32>,
    pub edge: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::coin_assets::Entity")]
    CoinAssets,
    #[sea_orm(
        belongs_to = "super::coin_images::Entity",
        from = "Column::Edge",
        to = "super::coin_images::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    CoinImages3,
    #[sea_orm(
        belongs_to = "super::coin_images::Entity",
        from = "Column::Reverse",
        to = "super::coin_images::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    CoinImages2,
    #[sea_orm(
        belongs_to = "super::coin_images::Entity",
        from = "Column::Obverse",
        to = "super::coin_images::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    CoinImages1,
}

impl Related<super::coin_assets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CoinAssets.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        super::coin_assets::Relation::Users.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::coin_assets::Relation::Coins.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
