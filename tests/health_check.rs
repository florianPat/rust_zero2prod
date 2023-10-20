mod common;

#[tokio::test]
async fn health_check_works() {
    let address = common::spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(& format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!("application/json", response.headers().get("Content-Type").unwrap());
}
