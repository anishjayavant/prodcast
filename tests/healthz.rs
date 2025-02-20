//! tests/healthz.rs
// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
#[tokio::test]
async fn healthz_works() {
    // Arrange
    spawn_app();
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get("http://127.0.0.1:8000/healthz")
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application in the background ~somehow~
fn spawn_app() {
    let server = prodcast::run().expect("Failed to bind address");
    tokio::spawn(server);
}
