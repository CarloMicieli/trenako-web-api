use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use configuration::Settings;
use sqlx::PgPool;
use std::net::TcpListener;

mod configuration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Settings::load().expect("Failed to read configuration");

    let address = TcpListener::bind(config.get_address())?;

    println!("{:?}", config.get_address());
    println!("{:?}", &config.get_database_url());

    let connection_pool = PgPool::connect(&config.get_database_url())
        .await
        .expect("Failed to connect to Postgres.");

    HttpServer::new(move || {
        App::new()
            .app_data(connection_pool.clone())
            .route("/health", web::get().to(health_check))
    })
    .listen(address)?
    .run()
    .await
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
