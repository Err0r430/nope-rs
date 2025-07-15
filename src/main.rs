use actix_web::{web, App, HttpServer};
use crate::config::EnvConfig;
use crate::routes::configure_routes;
use crate::db::postgres_service::PostgresService;
use std::sync::Arc;

mod config;
mod db;
mod routes;
mod response;
mod types;
mod utils;
mod macros;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let config = EnvConfig::from_env();
    let addr = format!("0.0.0.0:{}", config.port);

    let postgres_service = Arc::new(
        PostgresService::new(
            &config.db_url,
        )
            .await
            .expect("Failed to initialize PostgresService")
    );



    println!("Starting server on {}", addr);


    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&postgres_service)))
            .configure(configure_routes)
    })
    .bind(addr)?
    .run()
    .await
}