#[tokio::test]
async fn health_check_works(){
    spwam_app();

    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:9000/health_check")
        .send()
        .await
        .expect("Failed to execute resquests.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spwam_app() {
    let server = rust2prod::run().expect("Faield to bind address");
    let _ = tokio::spawn(server);
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data(){
    let client = reqwest::Client::new();

    let body = "name=Gabriel&email=gabrieldutra8@proton.me";
    let response = client
        .post(&format!("{}/subscriptions", "http://127.0.0.1:9000"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=gabriel%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", "http://127.0.0.1:9000"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 when the payload was {}.",
            error_message
        );
    }

}
