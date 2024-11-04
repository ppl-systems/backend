use backend::config::get_config;
use backend::startup::run;
use backend::telemetry::{get_subscriber, init_subscriber};
use core::panic;
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("ppl-backend".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let config = match get_config() {
        Ok(config) => config,
        Err(e) => panic!("Failed to read config.yaml... {e}"),
    };
    let connection_pool = match PgPool::connect(&config.database.connection_string()).await {
        Ok(connection_pool) => connection_pool,
        Err(e) => panic!("Failed to connect to Postgres... {e}"),
    };

    let address = format!("127.0.0.1:{}", config.application_port);

    let listener = match TcpListener::bind(address) {
        Ok(listener) => listener,
        Err(e) => panic!("Failed to bid to address... {e}"),
    };

    run(listener, connection_pool).expect("Failed to run").await
}
