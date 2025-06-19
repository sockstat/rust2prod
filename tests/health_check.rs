#[tokio::test]

async fn health_check_worlds(){
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