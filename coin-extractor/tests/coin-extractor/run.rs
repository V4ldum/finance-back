use approx::assert_relative_eq;
use claims::assert_ok;
use coin_extractor::run;
use fake::{faker::lorem::en::Sentence, Fake};
use wiremock::{matchers::method, Mock, MockServer, ResponseTemplate};

use crate::helpers::{test_app, TestApp};

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

    assert_eq!(headers["Numista-API-Key"], numista_api_key);
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
