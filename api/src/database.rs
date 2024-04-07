use std::collections::HashMap;

use rusqlite::Connection;

use crate::TradeValue;

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn build() -> Result<Self, rusqlite::Error> {
        let database = "database.db";
        let conn = Connection::open(database)?;

        Ok(Database { connection: conn })
    }

    pub fn query_trade_values(&self) -> Result<HashMap<String, TradeValue>, rusqlite::Error> {
        let mut stmt = self
            .connection
            .prepare("SELECT name, value, date FROM prices")?;
        let rows = stmt.query_map((), |row| {
            let name: String = row.get(0).expect("3 columns were queries");
            let value: f64 = row.get(1).expect("3 columns were queries");
            let date: String = row.get(2).expect("3 columns were queries");

            println!("{name} {value} {date}");

            Ok((name, value, date))
        })?;

        let rows = rows.map(|row| {
            let (name, value, date) = row
                .expect("There should not be any errors when reading data from the prices table");
            (
                name,
                TradeValue {
                    price: value,
                    last_update: date,
                },
            )
        });

        Ok(rows.into_iter().collect())
    }

    pub fn query_trade_value(&self, key: &str) -> Result<TradeValue, rusqlite::Error> {
        let mut stmt = self
            .connection
            .prepare("SELECT value, date FROM prices WHERE name = ?1")?;

        stmt.query_row([key], |row| {
            let value: f64 = row.get(0).expect("2 columns were queries");
            let date: String = row.get(1).expect("2 columns were queries");

            Ok(TradeValue {
                price: value,
                last_update: date,
            })
        })
    }

    pub fn check_api_key(&self, key: &str) -> Result<bool, rusqlite::Error> {
        let mut stmt = self
            .connection
            .prepare("SELECT count(*) FROM api_keys WHERE key=?1")?;

        let count = stmt.query_row([key], |row| {
            let count: i32 = row.get(0)?;

            println!("{count}");

            Ok(count)
        });

        Ok(count.unwrap_or(0) == 1)
    }
}
