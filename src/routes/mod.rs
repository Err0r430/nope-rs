use actix_web::web;

pub mod health;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(
        web::scope("/health")
            .service(health::health)
    );
}