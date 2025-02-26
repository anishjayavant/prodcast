use crate::common::tests_utils::spawn_app;

#[tokio::test]
async fn healthz_works() {
    // Arrange
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/healthz", &address))
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
    let address = spawn_app().await;
    let client = reqwest::Client::new();
    let body = "name=Van%20Damme&email=jean_claude_van_damme%40gmail.com";

    // Act
    let response = client
        .post(format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    let status = response.status().as_u16();

    // Assert
    assert_eq!(status, 200);
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let address = spawn_app().await;
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
            .post(format!("{}/subscriptions", &address))
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
