use anyhow::Result;
use chrono::{NaiveDate, Utc};
use sqlx::migrate::MigrateDatabase;
use sqlx::{Sqlite, SqlitePool};

pub struct Database {
    db: SqlitePool,
}

impl Database {
    pub async fn build() -> Result<Self> {
        dotenvy::dotenv()?;
        let database_url = dotenvy::var("DATABASE_URL")?;

        assert!(
            Sqlite::database_exists(&database_url).await.unwrap_or(false),
            "Database not found"
        );

        let db = SqlitePool::connect(&database_url).await?;

        Ok(Database { db })
    }

    pub async fn update_gold_price(&self, price: f64) -> Result<()> {
        self.update_value("Gold", price).await
    }

    pub async fn update_silver_price(&self, price: f64) -> Result<()> {
        self.update_value("Silver", price).await
    }

    pub async fn update_sp500_price(&self, price: f64) -> Result<()> {
        self.update_value("SP500", price).await
    }

    async fn update_value(&self, key: &str, price: f64) -> Result<()> {
        let price = (price * 100.0).round() / 100.0; // Rounding price to 2 digits after the decimal point

        let date = Utc::now().to_rfc3339();
        let date = date
            .split('T')
            .next()
            .expect("The first part of the RFC3339 date should be found");
        let date = date.parse::<NaiveDate>().expect("Date should be properly formatted");

        let entry = sqlx::query!(
            r#"
            SELECT * FROM prices WHERE name = $1
            "#,
            key
        )
        .fetch_optional(&self.db)
        .await?;

        match entry {
            Some(_) => {
                // UPDATE
                sqlx::query!(
                    r#"
                    UPDATE prices SET value = $1, date = $2 WHERE name = $3
                    "#,
                    price,
                    date,
                    key
                )
                .execute(&self.db)
                .await?;
            }
            None => {
                // INSERT
                sqlx::query!(
                    r#"
                    INSERT INTO prices (name, value, date)
                    VALUES($1, $2, $3)
                    "#,
                    key,
                    price,
                    date,
                )
                .execute(&self.db)
                .await?;
            }
        }

        Ok(())
    }
}
