//use std::net::TcpListener;
//use rust2prod::startup::run;

use rust2prod::configuration::get_configuration;
use sqlx::{Connection, PgConnection, PgPool};
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}
/*
async fn spwam_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:9000").expect("Failed to bind port");
    let address = format!("http://127.0.0.1:9000");

    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(
        &configuration.database.connection_string()
        )
        .await
        .expect("failed to connect to Postgres");

    let server = run(listener, connection_pool.clone()).expect("Faield to bind address");
    let _ = tokio::spawn(server);

    TestApp { address: address, db_pool: connection_pool }
}
*/

#[tokio::test]
async fn health_check_works() {
    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:9000/health_check")
        .send()
        .await
        .expect("Failed to execute resquests.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn create_subscription() {
    let client = reqwest::Client::new();
    let body = format!("name=gabriel&email=gabrieldutra10@{}.com", Uuid::new_v4());

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
async fn subscribe_returns_a_200_for_valid_form_data() {
    let configuration = get_configuration().expect("failed to read configuration");
    let connection_string = configuration.database.connection_string();

    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fech saved subscriptions");

    assert_eq!(saved.name, "Gabriel");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=gabriel%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
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
