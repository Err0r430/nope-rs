use actix_web::web;
use actix_governor::{Governor, GovernorConfigBuilder};

pub mod admin;
pub mod health;
pub mod nope;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {

    // configure a rate limit: 60 requests per minute, burst up to 5
    let standard_govnah = GovernorConfigBuilder::default()
        .requests_per_minute(50)
        .burst_size(5)
        .finish()
        .expect("Invalid standard governor configuration");

    cfg
    .service(
        web::scope("/health")
            .service(health::health)
    )
    .service(
        web::scope("/nope")
            .wrap(Governor::new(&standard_govnah))
            .service(nope::health)
    )
    .service(
        web::scope("/admin")
            .service(admin::admin_nope_create)
    )
    ;
}