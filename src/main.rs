use rust2prod::configuration::get_configuration;
use rust2prod::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to database.");

    run(listener, connection)?.await
}
