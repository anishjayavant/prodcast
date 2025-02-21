//! tests/subscriptions.rs
pub mod common;
use common::utils::spawn_app;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let address = spawn_app();
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
