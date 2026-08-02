use claims::assert_some;

use crate::{
    coins::{insert_coin_with_name, nuke_coins_table},
    helpers::{name, spawn_app},
};

#[tokio::test]
async fn search_coins_returns_400_when_search_query_is_invalid() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.search_coins(" ").await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 400);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["status"], status);
    assert_eq!(json_response["reason"], "Invalid search query: ' '");
}

#[tokio::test]
async fn search_coins_fails_and_returns_500_if_there_is_a_fatal_database_error() {
    // Arrange
    let app = spawn_app().await;
    nuke_coins_table(&app).await;

    // Act
    let response = app.search_coins("coins").await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 500);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["status"], status);
    assert_eq!(json_response["reason"], "Failed to fetch coins");
}

#[tokio::test]
async fn search_coins_returns_the_correct_coins() {
    // Arrange
    let app = spawn_app().await;

    let coin_name = name();
    let coin_name_variant1 = format!("{coin_name}{}", name());
    let coin_name_variant2 = format!("{coin_name}{}", name());
    insert_coin_with_name(&app, &coin_name_variant1).await;
    insert_coin_with_name(&app, &coin_name_variant2).await;
    insert_coin_with_name(&app, &name()).await;

    // Act
    let response = app.search_coins(&coin_name).await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 200);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    let array = assert_some!(json_response.as_array());
    assert_eq!(array.len(), 2);
    assert!(array.iter().any(|v| v["name"] == coin_name_variant1));
    assert!(array.iter().any(|v| v["name"] == coin_name_variant2));
}

#[tokio::test]
async fn search_coins_partial_search_returns_the_correct_coins() {
    // Arrange
    let app = spawn_app().await;

    let coin_name1 = "Silver Krugerrand";
    let coin_name2 = "5 francs Napoléon";
    let coin_name3 = "tete lauree";
    let coin_name4 = "5 francs \"Semeuse\"";

    insert_coin_with_name(&app, coin_name1).await;
    insert_coin_with_name(&app, coin_name2).await;
    insert_coin_with_name(&app, coin_name3).await;
    insert_coin_with_name(&app, coin_name4).await;

    let test_cases = vec![
        (coin_name1, "Silver", "only the beginning of the full name"),
        (coin_name1, "Silver ", "a query with a trailing whitespace"),
        (coin_name1, "Krugerrand", "only the end of the full name"),
        (coin_name1, "Kruger", "only the beginning of a word of the full name"),
        (coin_name1, coin_name1, "the full name"),
        (coin_name1, "Krugerrand Silver", "the full name with reversed order"),
        (coin_name1, "silver", "all in lowercase"),
        (coin_name1, "SILVER", "all in uppercase"),
        (coin_name2, "Napoléon", "with an accent in the query"),
        (coin_name2, "Napoleon", "without an accent in the query"),
        (coin_name3, "tête", "with an accent in the query not the name"),
        (coin_name4, "semeuse", "quoted word without quotes"),
        (coin_name4, "\"semeuse\"", "quoted word with quotes"),
        (
            coin_name4,
            "francs \"semeuse\"",
            "quoted word with quotes and another word",
        ),
    ];

    for (name, query, error_message) in test_cases {
        // Act
        let response = app.search_coins(query).await;

        // Assert
        let json_response = response.json::<serde_json::Value>().await.unwrap();
        let array = assert_some!(json_response.as_array());
        assert_eq!(array.len(), 1, "Failed to find the coin when searching {error_message}");
        assert!(
            array.iter().any(|v| v["name"] == name),
            "Failed to find the coin when searching {error_message}"
        );
    }
}

#[tokio::test]
async fn search_coins_doesnt_fail_on_special_character_queries() {
    // Arrange
    let app = spawn_app().await;
    let test_cases = vec![
        "AND",
        "OR",
        "NOT",
        "NEAR",
        "foo-bar",
        "\"quoted\"",
        "star*",
        "(paren)",
        "colon:",
        "^caret",
        "dash-",
    ];

    for query in test_cases {
        insert_coin_with_name(&app, &format!("prefix-{query}")).await;

        // Act
        let response = app.search_coins(query).await;

        // Assert
        let status = response.status().as_u16();
        assert_eq!(status, 200);
    }
}
