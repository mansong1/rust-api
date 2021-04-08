/*!
tests/health_check.rs

- the health check is exposed at /health_check; 
- the health check is behind a GET method;
- the health check always returns a 200;
- the health check’s response has no body.

cargo add actix-rt --dev --vers 2
cargo add reqwest --dev --vers 0.11
cargo add tokio --dev --vers 1

*/

#[actix_rt::test]
async fn test_health_check() {
    start_app();

    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request. ");
    
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn start_app() {
    let server = rust_api::run().expect("Failed to bind address");

    let _ = tokio::spawn(server);
}