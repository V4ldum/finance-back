use chrono::Utc;
use rusqlite::Connection;

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn build() -> Result<Self, rusqlite::Error> {
        let database = "database.db";
        let conn = Connection::open(database)?;

        conn.execute(
            "\
            CREATE TABLE IF NOT EXISTS prices (\
                name VARCHAR(10) PRIMARY KEY,\
                value REAL NOT NULL,\
                date DATE NOT NULL\
            )",
            (),
        )?;

        Ok(Database { connection: conn })
    }

    pub fn update_value(&self, key: &str, price: f64) -> Result<(), rusqlite::Error> {
        let date = Utc::now().to_rfc3339();
        let date = date
            .split('T')
            .next()
            .expect("The first part of the RFC3339 date should be found");

        let rows_number = self
            .connection
            .prepare("SELECT name FROM prices WHERE name=?1")?
            .query_map((key,), |_| Ok(()))?
            .count();

        if rows_number == 0 {
            // No data found, creating entry
            self.connection.execute(
                "INSERT INTO prices (name, value, date) VALUES(?1, ROUND(?2, 2), ?3)",
                (key, price, date),
            )?;
        } else {
            // Data found, updating entry
            self.connection.execute(
                "UPDATE prices SET value=ROUND(?2, 2), date=?3 WHERE name=?1",
                (key, price, date),
            )?;
        }

        Ok(())
    }
}
