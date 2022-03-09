use sqlx::PgPool;
use std::net::TcpListener;
use webapi::configuration::Settings;
use webapi::server::run;
use webapi::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber =
        get_subscriber("trenako".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = Settings::load().expect("Failed to read configuration");

    let address = TcpListener::bind(config.get_address())?;

    let connection_pool = PgPool::connect(&config.get_database_url())
        .await
        .expect("Failed to connect to Postgres.");

    run(address, connection_pool)?.await
}
