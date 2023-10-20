mod common;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app_address = common::spawn_app();
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(& format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

#[rstest::rstest]
#[case("name=le%20guin", "missing the email")]
#[case("email=ursula_le_guin%40gmail.com", "missing the name")]
#[case("", "missing both email and name")]
#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing(#[case] invalid_body: String, #[case] error_message: String) {
    let app_address = common::spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .post(& format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/json")
        .body(invalid_body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(
        400,
       response.status().as_u16(),
        "API did not fail with 400 Bad Request when the payload was {}",
        error_message,
    );
}
