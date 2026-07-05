use std::time::Duration;

use crate::domain::{CoinQuery, CoinQuerySide};
use crate::program_parameters::ProgramParameters;
use anyhow::{Context, Result};
use reqwest::Client;
use secrecy::ExposeSecret;
use sqlx::{Connection, SqliteConnection};

mod domain;
pub mod program_parameters;

pub async fn run(mut params: ProgramParameters) -> Result<()> {
    let response = Client::new()
        .get(format!("{}{}?lang=fr", params.numista_url, params.coin_id))
        .header("Numista-API-Key", params.numista_api_key.expose_secret())
        .timeout(Duration::from_secs(1))
        .send()
        .await
        .context("Failed to fetch coin data from numista API")?;

    let coin = serde_json::from_str::<CoinQuery>(&response.text().await?).context("Failed to parse coin data")?;

    assert!(coin.watermark.is_none(), "Watermark found");

    insert_in_database(coin, params.coin_id, &mut params.db)
        .await
        .context("Failed to insert coin data into database")?;

    Ok(())
}

async fn insert_in_database(coin: CoinQuery, numista_id: u32, db: &mut SqliteConnection) -> Result<()> {
    let mut transaction = db.begin().await?;

    let obverse_id = match coin.obverse {
        Some(obverse) => {
            let id = insert_coin_side(obverse, &mut transaction)
                .await
                .context("Failed to insert obverse")?;
            Some(id)
        }
        None => None,
    };
    let reverse_id = match coin.reverse {
        Some(reverse) => {
            let id = insert_coin_side(reverse, &mut transaction)
                .await
                .context("Failed to insert reverse")?;
            Some(id)
        }
        None => None,
    };
    let edge_id = match coin.edge {
        Some(edge) => {
            let id = insert_coin_side(edge, &mut transaction)
                .await
                .context("Failed to insert edge")?;
            Some(id)
        }
        None => None,
    };
    // let watermark_id = match coin.watermark {
    //     Some(watermark) => {
    //         let model = insert_coin_side(watermark, &db).await.context("Failed to insert watermark")?;
    //         Some(id)
    //     },
    //     None => None,
    // };

    sqlx::query!(
        r#"
        INSERT INTO coins (numista_id, name, weight, size, thickness, min_year, max_year, composition, purity, obverse, reverse, edge)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        "#,
        numista_id,
        coin.title,
        coin.weight,
        coin.size,
        coin.thickness,
        coin.min_year,
        coin.max_year,
        coin.composition.composition,
        coin.composition.purity,
        obverse_id,
        reverse_id,
        edge_id
    )
    .execute(&mut *transaction)
    .await
    .context("Failed to insert coin data")?;

    transaction.commit().await.context("Failed to commit transaction")?;

    Ok(())
}

async fn insert_coin_side(side: CoinQuerySide, transaction: &mut SqliteConnection) -> Result<i64> {
    let result = sqlx::query!(
        r#"
        INSERT INTO coin_images (image_url, thumbnail_url, lettering, description, copyright)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id
        "#,
        side.picture,
        side.thumbnail,
        side.lettering,
        side.description,
        side.picture_copyright
    )
    .fetch_one(transaction)
    .await
    .context("Failed to insert coin side")?;

    Ok(result.id)
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use claims::assert_ok;
    use fake::{faker::lorem::en::Sentence, Fake};
    use secrecy::{ExposeSecret, SecretString};
    use sqlx::{Connection, SqliteConnection};
    use uuid::Uuid;
    use wiremock::{matchers::method, Mock, MockServer, ResponseTemplate};

    use crate::{program_parameters::ProgramParameters, run};

    struct TestApp {
        pub params: ProgramParameters,
        pub db: SqliteConnection,
    }

    async fn test_app(url: String) -> TestApp {
        let db_url = format!("sqlite:file:memdb-{}?mode=memory&cache=shared", Uuid::new_v4());

        // Keep one connection alive for the whole test: a `mode=memory&cache=shared` database
        // only lives while at least one connection is open, and `run` drops the one it owns.
        // Migrations are applied through this connection and seen by `run` via the shared cache.
        let mut db = SqliteConnection::connect(&db_url)
            .await
            .expect("Failed to open database");
        sqlx::migrate!("../api/migrations")
            .run(&mut db)
            .await
            .expect("Failed to run migrations");

        let params = ProgramParameters {
            numista_url: format!("http://{url}/"),
            numista_api_key: SecretString::from((1000..9999).fake::<u32>().to_string()),
            coin_id: (u32::MIN..u32::MAX).fake(),
            db: SqliteConnection::connect(&db_url)
                .await
                .expect("Failed to open database"),
        };

        TestApp { params, db }
    }

    #[tokio::test]
    async fn run_fires_a_request_to_numista_with_proper_headers() {
        // Arrange
        let mock_server = MockServer::start().await;
        let TestApp { params, .. } = test_app(mock_server.address().to_string()).await;
        let numista_api_key = params.numista_api_key.clone();

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_string(format!(
                r#"
                {{
                    "id": {},
                    "title": "{}",
                    "weight": {},
                    "size": {},
                    "thickness": {},
                    "min_year": {},
                    "max_year": {},
                    "composition": {{
                        "text": "Or 900‰"
                    }},
                    "obverse": null,
                    "reverse": null ,
                    "edge": null,
                    "watermark": null
                }}
                "#,
                params.coin_id,
                Sentence(1..3).fake::<String>(),
                (1.0..100.0).fake::<f64>(),
                (10.0..50.0).fake::<f64>(),
                (0.5..5.0).fake::<f64>(),
                (1800..1900).fake::<u32>(),
                (1900..2024).fake::<u32>(),
            )))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Act
        let result = run(params).await;

        // Assert
        let requests = mock_server
            .received_requests()
            .await
            .expect("Failed to retrieve received requests");
        let headers = &requests[0].headers;

        assert_eq!(headers["Numista-API-Key"], numista_api_key.expose_secret());
        assert_ok!(result);
    }

    #[tokio::test]
    #[allow(clippy::too_many_lines)]
    async fn run_inserts_the_correct_coin_information_into_the_database() {
        // Arrange
        let mock_server = MockServer::start().await;
        let TestApp { params, mut db } = test_app(mock_server.address().to_string()).await;

        let id = params.coin_id;
        let title = Sentence(1..3).fake::<String>();
        let weight = (1.0..100.0).fake::<f64>();
        let size = (10.0..50.0).fake::<f64>();
        let thickness = (0.5..5.0).fake::<f64>();
        let min_year = (1800..1900).fake::<u32>();
        let max_year = (1900..2024).fake::<u32>();

        let obverse_description = Sentence(1..3).fake::<String>();
        let obverse_lettering = Sentence(1..3).fake::<String>();
        let obverse_picture = Sentence(1..3).fake::<String>();
        let obverse_thumbnail = Sentence(1..3).fake::<String>();
        let obverse_picture_copyright = Sentence(1..3).fake::<String>();

        let reverse_description = Sentence(1..3).fake::<String>();
        let reverse_lettering = Sentence(1..3).fake::<String>();
        let reverse_picture = Sentence(1..3).fake::<String>();
        let reverse_thumbnail = Sentence(1..3).fake::<String>();
        let reverse_picture_copyright = Sentence(1..3).fake::<String>();

        let edge_description = Sentence(1..3).fake::<String>();
        let edge_lettering = Sentence(1..3).fake::<String>();
        let edge_picture = Sentence(1..3).fake::<String>();
        let edge_thumbnail = Sentence(1..3).fake::<String>();
        let edge_picture_copyright = Sentence(1..3).fake::<String>();

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_string(format!(
                r#"
                {{
                    "id": {id},
                    "title": "{title}",
                    "weight": {weight},
                    "size": {size},
                    "thickness": {thickness},
                    "min_year": {min_year},
                    "max_year": {max_year},
                    "composition": {{
                        "text": "Or 900‰"
                    }},
                    "obverse": {{
                        "description": "{obverse_description}",
                        "lettering": "{obverse_lettering}",
                        "picture": "{obverse_picture}",
                        "thumbnail": "{obverse_thumbnail}",
                        "picture_copyright": "{obverse_picture_copyright}"
                    }},
                    "reverse": {{
                        "description": "{reverse_description}",
                        "lettering": "{reverse_lettering}",
                        "picture": "{reverse_picture}",
                        "thumbnail": "{reverse_thumbnail}",
                        "picture_copyright": "{reverse_picture_copyright}"
                    }},
                    "edge": {{
                        "description": "{edge_description}",
                        "lettering": "{edge_lettering}",
                        "picture": "{edge_picture}",
                        "thumbnail": "{edge_thumbnail}",
                        "picture_copyright": "{edge_picture_copyright}"
                    }},
                    "watermark": null
                }}
                "#
            )))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Act
        run(params).await.expect("Failed to run");

        // Assert
        let coin = sqlx::query!("SELECT * FROM coins")
            .fetch_one(&mut db)
            .await
            .expect("Failed to fetch coin");

        assert_eq!(coin.numista_id, id.to_string());
        assert_eq!(coin.name, title);
        assert_relative_eq!(coin.weight, weight);
        assert_relative_eq!(coin.size, size);
        assert_relative_eq!(coin.thickness.expect("thickness should be set"), thickness);
        assert_eq!(coin.min_year, min_year.to_string());
        assert_eq!(coin.max_year, Some(max_year.to_string()));
        assert_eq!(coin.composition, "GOLD");
        assert_eq!(coin.purity, 9000);

        let obverse = sqlx::query!("SELECT * FROM coin_images WHERE id = ?", coin.obverse)
            .fetch_one(&mut db)
            .await
            .expect("Failed to fetch obverse image");

        assert_eq!(obverse.image_url, Some(obverse_picture));
        assert_eq!(obverse.thumbnail_url, Some(obverse_thumbnail));
        assert_eq!(obverse.lettering, Some(obverse_lettering));
        assert_eq!(obverse.description, Some(obverse_description));
        assert_eq!(obverse.copyright, Some(obverse_picture_copyright));

        let reverse = sqlx::query!("SELECT * FROM coin_images WHERE id = ?", coin.reverse)
            .fetch_one(&mut db)
            .await
            .expect("Failed to fetch reverse image");

        assert_eq!(reverse.image_url, Some(reverse_picture));
        assert_eq!(reverse.thumbnail_url, Some(reverse_thumbnail));
        assert_eq!(reverse.lettering, Some(reverse_lettering));
        assert_eq!(reverse.description, Some(reverse_description));
        assert_eq!(reverse.copyright, Some(reverse_picture_copyright));

        let edge = sqlx::query!("SELECT * FROM coin_images WHERE id = ?", coin.edge)
            .fetch_one(&mut db)
            .await
            .expect("Failed to fetch edge image");

        assert_eq!(edge.image_url, Some(edge_picture));
        assert_eq!(edge.thumbnail_url, Some(edge_thumbnail));
        assert_eq!(edge.lettering, Some(edge_lettering));
        assert_eq!(edge.description, Some(edge_description));
        assert_eq!(edge.copyright, Some(edge_picture_copyright));
    }

    #[tokio::test]
    #[should_panic = "Watermark found"]
    async fn run_panics_when_a_watermark_is_found() {
        // Arrange
        let mock_server = MockServer::start().await;
        let TestApp { params, .. } = test_app(mock_server.address().to_string()).await;

        let id = params.coin_id;
        let title = Sentence(1..3).fake::<String>();
        let weight = (1.0..100.0).fake::<f64>();
        let size = (10.0..50.0).fake::<f64>();
        let thickness = (0.5..5.0).fake::<f64>();
        let min_year = (1800..1900).fake::<u32>();
        let max_year = (1900..2024).fake::<u32>();

        let watermark_description = Sentence(1..3).fake::<String>();
        let watermark_lettering = Sentence(1..3).fake::<String>();
        let watermark_picture = Sentence(1..3).fake::<String>();
        let watermark_thumbnail = Sentence(1..3).fake::<String>();
        let watermark_picture_copyright = Sentence(1..3).fake::<String>();

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_string(format!(
                r#"{{
                        "id": {id},
                        "title": "{title}",
                        "weight": {weight},
                        "size": {size},
                        "thickness": {thickness},
                        "min_year": {min_year},
                        "max_year": {max_year},
                        "composition": {{
                            "text": "Or 900‰"
                        }},
                        "obverse": null,
                        "reverse": null,
                        "edge": null,
                        "watermark": {{
                            "description": "{watermark_description}",
                            "lettering": "{watermark_lettering}",
                            "picture": "{watermark_picture}",
                            "thumbnail": "{watermark_thumbnail}",
                            "picture_copyright": "{watermark_picture_copyright}"
                        }}
                    }}"#
            )))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Act
        let _ = run(params).await;

        // Assert
        // Nothing to assert, it should panic
    }
}
