use crate::common::tests_utils::{create_test_connection_pool, get_test_config, spawn_app};
use prodcast::{
    models::newsletter::User, repository::newsletter::NewsletterPostGresRepository,
    repository::newsletter::NewsletterRepository,
};

#[tokio::test]
async fn healthz_works() {
    // Arrange
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/healthz", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();
    // create a test User
    let user = User::new(
        "jean_claude_van_damme@gmail.com".to_string(),
        "Van Damme".to_string(),
    );
    // create the body of the request using the test User, with proper URL encoding
    let body = format!(
        "name={}&email={}",
        user.name().replace(" ", "%20"),
        user.email().replace("@", "%40")
    );

    // Act
    let response = client
        .post(format!("{}/subscriptions", &test_app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    let status = response.status().as_u16();
    // verify that the user was saved, using the NewsletterPostGresRepository
    let newsletter_repository = NewsletterPostGresRepository::new(
        create_test_connection_pool(&get_test_config(0, &test_app.database_name).database)
            .await
            .expect("Failed to create connection pool."),
    );
    let saved_user = newsletter_repository
        .get_user(user.email().to_string())
        .await
        .expect("Failed to get user");

    // Assert that response is 200
    assert_eq!(status, 200);
    // Assert that the user was saved
    assert_eq!(user.email(), saved_user.email());
    assert_eq!(user.name(), saved_user.name());
}

#[tokio::test]
async fn subscribe_returns_a_500_if_repeated_with_same_user() {
    // Arrange
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();
    // create a test User
    let user = User::new(
        "jean_claude_van_damme@gmail.com".to_string(),
        "Van Damme".to_string(),
    );
    // create the body of the request using the test User, with proper URL encoding
    let body = format!(
        "name={}&email={}",
        user.name().replace(" ", "%20"),
        user.email().replace("@", "%40")
    );

    // Act
    let response = client
        .post(format!("{}/subscriptions", &test_app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body.clone())
        .send()
        .await
        .expect("Failed to execute request.");

    let status = response.status().as_u16();
    // verify that the user was saved, using the NewsletterPostGresRepository
    let newsletter_repository = NewsletterPostGresRepository::new(
        create_test_connection_pool(&get_test_config(0, &test_app.database_name).database)
            .await
            .expect("Failed to create connection pool."),
    );
    let saved_user = newsletter_repository
        .get_user(user.email().to_string())
        .await
        .expect("Failed to get user");

    // Assert that response is 200
    assert_eq!(status, 200);
    // Assert that the user was saved
    assert_eq!(user.email(), saved_user.email());
    assert_eq!(user.name(), saved_user.name());

    // Act
    let response = client
        .post(format!("{}/subscriptions", &test_app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert_eq!(response.status().as_u16(), 500);
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=Van%20Damme", "missing the email"),
        (
            "email=jean_claude_van_damme%40gmail.com",
            "missing the name",
        ),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(format!("{}/subscriptions", &test_app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        let status = response.status().as_u16();

        // Assert
        assert_eq!(
            status, 400,
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
