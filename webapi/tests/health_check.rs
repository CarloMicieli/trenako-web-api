mod common;
use common::spawn_app;

#[actix_rt::test]
async fn health_check_works() {
    let service = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(service.endpoint("/health"))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
