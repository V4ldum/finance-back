use anyhow::{Context, Result, bail};
use chrono::{NaiveDate, Utc};
use sqlx::migrate::MigrateDatabase;
use sqlx::{Sqlite, SqlitePool};

static GOLD_PRICE_LABEL: &str = "GOLD";
static SILVER_PRICE_LABEL: &str = "SILVER";
static SP500_PRICE_LABEL: &str = "SP500";

pub struct Database {
    db: SqlitePool,
}

impl Database {
    pub async fn build(database_url: &str) -> Result<Self> {
        if !Sqlite::database_exists(database_url).await.unwrap_or(false) {
            bail!("Database not found")
        }

        let db = SqlitePool::connect(database_url)
            .await
            .context("Failed to connect to database")?;
        sqlx::migrate!("../migrations")
            .run(&db)
            .await
            .context("Failed to run migrations")?;

        Ok(Database { db })
    }

    pub async fn update_gold_price(&self, price: f64) -> Result<()> {
        self.update_value(GOLD_PRICE_LABEL, price)
            .await
            .context("Failed to update gold price")
    }

    pub async fn update_silver_price(&self, price: f64) -> Result<()> {
        self.update_value(SILVER_PRICE_LABEL, price)
            .await
            .context("Failed to update silver price")
    }

    pub async fn update_sp500_price(&self, price: f64) -> Result<()> {
        self.update_value(SP500_PRICE_LABEL, price)
            .await
            .context("Failed to update SP500 price")
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
        .await
        .context("Failed to fetch price entry")?;

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
                .await
                .context("Failed to update price entry")?;
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
                .await
                .context("Failed to insert price entry")?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use chrono::{NaiveDate, Utc};
    use claims::assert_ok;
    use fake::Fake;
    use uuid::Uuid;

    use crate::database::{Database, GOLD_PRICE_LABEL, SILVER_PRICE_LABEL, SP500_PRICE_LABEL};

    async fn database() -> Database {
        Database::build(&format!(
            "sqlite:file:memdb-{}?mode=memory&cache=shared",
            Uuid::new_v4()
        ))
        .await
        .expect("Failed to build database")
    }

    fn gold_price() -> f64 {
        let price = (2000.0..4000.0).fake::<f64>();
        // Rounded to 2 decimal points
        (price * 100.0).round() / 100.0
    }

    fn silver_price() -> f64 {
        let price = (20.0..100.0).fake::<f64>();
        // Rounded to 2 decimal points
        (price * 100.0).round() / 100.0
    }

    fn sp500_price() -> f64 {
        let price = (5000.0..8000.0).fake::<f64>();
        // Rounded to 2 decimal points
        (price * 100.0).round() / 100.0
    }

    fn date() -> NaiveDate {
        let date = Utc::now().to_rfc3339();
        let date = date
            .split('T')
            .next()
            .expect("The first part of the RFC3339 date should be found");
        date.parse::<NaiveDate>().expect("Date should be properly formatted")
    }

    #[tokio::test]
    async fn update_gold_price_inserts_in_empty_table() {
        // Arrange
        let db = database().await;
        let price = gold_price();
        let date = date();

        // Act
        let result = db.update_gold_price(price).await;

        // Assert
        assert_ok!(result);
        let saved = sqlx::query!("SELECT * FROM prices WHERE name = $1", GOLD_PRICE_LABEL)
            .fetch_one(&db.db)
            .await
            .expect("Failed to fetch saved gold price");

        assert_relative_eq!(saved.value, price);
        assert_eq!(saved.date, date);
    }

    #[tokio::test]
    async fn update_gold_price_updates_existing_entry() {
        // Arrange
        let db = database().await;
        let price = gold_price();
        let date = date();

        // Act
        db.update_gold_price(gold_price())
            .await
            .expect("Failed to insert initial gold price");
        let result = db.update_gold_price(price).await;

        // Assert
        assert_ok!(result);
        let saved = sqlx::query!("SELECT * FROM prices WHERE name = $1", GOLD_PRICE_LABEL)
            .fetch_one(&db.db)
            .await
            .expect("Failed to fetch saved gold price");

        assert_relative_eq!(saved.value, price);
        assert_eq!(saved.date, date);
    }

    #[tokio::test]
    async fn update_value_gold_inserts_in_empty_table() {
        // Arrange
        let db = database().await;
        let price = gold_price();
        let date = date();

        // Act
        let result = db.update_value(GOLD_PRICE_LABEL, price).await;

        // Assert
        assert_ok!(result);
        let saved = sqlx::query!("SELECT * FROM prices WHERE name = $1", GOLD_PRICE_LABEL)
            .fetch_one(&db.db)
            .await
            .expect("Failed to fetch saved gold price");

        assert_relative_eq!(saved.value, price);
        assert_eq!(saved.date, date);
    }

    #[tokio::test]
    async fn update_value_gold_updates_existing_entry() {
        // Arrange
        let db = database().await;
        let price = gold_price();
        let date = date();

        // Act
        db.update_value(GOLD_PRICE_LABEL, gold_price())
            .await
            .expect("Failed to insert initial gold price");
        let result = db.update_value(GOLD_PRICE_LABEL, price).await;

        // Assert
        assert_ok!(result);
        let saved = sqlx::query!("SELECT * FROM prices WHERE name = $1", GOLD_PRICE_LABEL)
            .fetch_one(&db.db)
            .await
            .expect("Failed to fetch saved gold price");

        assert_relative_eq!(saved.value, price);
        assert_eq!(saved.date, date);
    }

    #[tokio::test]
    async fn update_silver_price_inserts_in_empty_table() {
        // Arrange
        let db = database().await;
        let price = silver_price();
        let date = date();

        // Act
        let result = db.update_silver_price(price).await;

        // Assert
        assert_ok!(result);
        let saved = sqlx::query!("SELECT * FROM prices WHERE name = $1", SILVER_PRICE_LABEL)
            .fetch_one(&db.db)
            .await
            .expect("Failed to fetch saved silver price");

        assert_relative_eq!(saved.value, price);
        assert_eq!(saved.date, date);
    }

    #[tokio::test]
    async fn update_silver_price_updates_existing_entry() {
        // Arrange
        let db = database().await;
        let price = silver_price();
        let date = date();

        // Act
        db.update_silver_price(silver_price())
            .await
            .expect("Failed to insert initial silver price");
        let result = db.update_silver_price(price).await;

        // Assert
        assert_ok!(result);
        let saved = sqlx::query!("SELECT * FROM prices WHERE name = $1", SILVER_PRICE_LABEL)
            .fetch_one(&db.db)
            .await
            .expect("Failed to fetch saved silver price");

        assert_relative_eq!(saved.value, price);
        assert_eq!(saved.date, date);
    }

    #[tokio::test]
    async fn update_value_silver_inserts_in_empty_table() {
        // Arrange
        let db = database().await;
        let price = silver_price();
        let date = date();

        // Act
        let result = db.update_value(SILVER_PRICE_LABEL, price).await;

        // Assert
        assert_ok!(result);
        let saved = sqlx::query!("SELECT * FROM prices WHERE name = $1", SILVER_PRICE_LABEL)
            .fetch_one(&db.db)
            .await
            .expect("Failed to fetch saved silver price");

        assert_relative_eq!(saved.value, price);
        assert_eq!(saved.date, date);
    }

    #[tokio::test]
    async fn update_value_silver_updates_existing_entry() {
        // Arrange
        let db = database().await;
        let price = silver_price();
        let date = date();

        // Act
        db.update_value(SILVER_PRICE_LABEL, silver_price())
            .await
            .expect("Failed to insert initial silver price");
        let result = db.update_value(SILVER_PRICE_LABEL, price).await;

        // Assert
        assert_ok!(result);
        let saved = sqlx::query!("SELECT * FROM prices WHERE name = $1", SILVER_PRICE_LABEL)
            .fetch_one(&db.db)
            .await
            .expect("Failed to fetch saved silver price");

        assert_relative_eq!(saved.value, price);
        assert_eq!(saved.date, date);
    }

    #[tokio::test]
    async fn update_sp500_price_inserts_in_empty_table() {
        // Arrange
        let db = database().await;
        let price = sp500_price();
        let date = date();

        // Act
        let result = db.update_sp500_price(price).await;

        // Assert
        assert_ok!(result);
        let saved = sqlx::query!("SELECT * FROM prices WHERE name = $1", SP500_PRICE_LABEL)
            .fetch_one(&db.db)
            .await
            .expect("Failed to fetch saved sp500 price");

        assert_relative_eq!(saved.value, price);
        assert_eq!(saved.date, date);
    }

    #[tokio::test]
    async fn update_sp500_price_updates_existing_entry() {
        // Arrange
        let db = database().await;
        let price = sp500_price();
        let date = date();

        // Act
        db.update_sp500_price(sp500_price())
            .await
            .expect("Failed to insert initial sp500 price");
        let result = db.update_sp500_price(price).await;

        // Assert
        assert_ok!(result);
        let saved = sqlx::query!("SELECT * FROM prices WHERE name = $1", SP500_PRICE_LABEL)
            .fetch_one(&db.db)
            .await
            .expect("Failed to fetch saved sp500 price");

        assert_relative_eq!(saved.value, price);
        assert_eq!(saved.date, date);
    }

    #[tokio::test]
    async fn update_value_sp500_inserts_in_empty_table() {
        // Arrange
        let db = database().await;
        let price = sp500_price();
        let date = date();

        // Act
        let result = db.update_value(SP500_PRICE_LABEL, price).await;

        // Assert
        assert_ok!(result);
        let saved = sqlx::query!("SELECT * FROM prices WHERE name = $1", SP500_PRICE_LABEL)
            .fetch_one(&db.db)
            .await
            .expect("Failed to fetch saved sp500 price");

        assert_relative_eq!(saved.value, price);
        assert_eq!(saved.date, date);
    }

    #[tokio::test]
    async fn update_value_sp500_updates_existing_entry() {
        // Arrange
        let db = database().await;
        let price = sp500_price();
        let date = date();

        // Act
        db.update_value(SP500_PRICE_LABEL, sp500_price())
            .await
            .expect("Failed to insert initial sp500 price");
        let result = db.update_value(SP500_PRICE_LABEL, price).await;

        // Assert
        assert_ok!(result);
        let saved = sqlx::query!("SELECT * FROM prices WHERE name = $1", SP500_PRICE_LABEL)
            .fetch_one(&db.db)
            .await
            .expect("Failed to fetch saved sp500 price");

        assert_relative_eq!(saved.value, price);
        assert_eq!(saved.date, date);
    }
}
