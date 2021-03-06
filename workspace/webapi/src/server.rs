use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sqlx::PgPool;
use std::net::TcpListener;
use actix_web::middleware::Compress;
use tracing_actix_web::TracingLogger;
use crate::handlers::catalog;

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(Compress::default())
            .route("/health", web::get().to(health_check))
            .configure(catalog::config_services)
            .app_data(web::JsonConfig::default().limit(4096))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
