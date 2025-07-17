use actix_web::{web, App, HttpServer};
use crate::config::EnvConfig;
use crate::routes::configure_routes;
use crate::nope_service::NopeService;
use std::sync::Arc;

mod config;
mod routes;
mod response;
mod types;
mod utils;
mod macros;
mod nope_service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let config = EnvConfig::from_env();
    let nope_service = Arc::new(NopeService::new());

    println!("Starting server on {}", config.port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&nope_service)))
            .configure(configure_routes)
    })
    .bind(format!("0.0.0.0:{}", config.port))?
    .run()
    .await
}
