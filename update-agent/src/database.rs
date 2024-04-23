use std::error::Error;

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ConnectionTrait, DatabaseConnection, EntityTrait, Schema};
use sea_orm::ActiveValue::{Set, Unchanged};

use crate::database::generated::prelude::Prices;
use crate::database::generated::prices;

mod generated;

pub struct Database {
    db: DatabaseConnection,
}

impl Database {
    pub async fn build() -> Result<Self, Box<dyn Error>> {
        dotenvy::dotenv()?;
        let database_url = dotenvy::var("DATABASE_URL")?;

        let db = sea_orm::Database::connect(database_url).await?;

        let builder = db.get_database_backend();
        let schema = Schema::new(builder);
        let statement = builder.build(schema.create_table_from_entity(Prices).if_not_exists());
        db.execute(statement).await?;

        Ok(Database { db })
    }

    pub async fn update_value(&self, key: &str, price: f64) -> Result<(), Box<dyn Error>> {
        let price = (price * 100.0).round() / 100.0; // Rounding price to 2 digits after the decimal point

        let date = Utc::now().to_rfc3339();
        let date = date
            .split('T')
            .next()
            .expect("The first part of the RFC3339 date should be found");

        let entry = Prices::find_by_id(key).one(&self.db).await?;

        match entry {
            Some(_) => {
                // UPDATE
                prices::ActiveModel {
                    name: Unchanged(key.to_owned()),
                    value: Set(price),
                    date: Set(date.parse().expect("Date should be properly formatted")),
                }
                .update(&self.db)
                .await?;
            }
            None => {
                // INSERT
                prices::ActiveModel {
                    name: Unchanged(key.to_owned()),
                    value: Set(price),
                    date: Set(date.parse().expect("Date should be properly formatted")),
                }
                .insert(&self.db)
                .await?;
            }
        }

        Ok(())
    }
}
