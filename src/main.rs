use sojurust::startup::run;
use sqlx::{Connection, PgConnection};
use std::net::TcpListener;
use sojurust::configuration::get_configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let connection = PgConnection::connect(&configuration.database.connection_string())
    .await
    .expect("Failed to connect to Postgres.");
    let listener = TcpListener::bind(address)
    .expect("Port 4000 already in use");
    run(listener, connection)?.await
}