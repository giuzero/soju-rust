use sojurust::startup::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4000")
    .expect("Port 4000 already in use");
    run(listener)?.await
}