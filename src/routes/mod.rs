use actix_web::web;
use actix_governor::{Governor, GovernorConfigBuilder};

pub mod health;
pub mod nope;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {

    // configure a rate limit: 5 requests per second, burst up to 2
    let standard_govnah = GovernorConfigBuilder::default()
        .requests_per_minute(100)
        .burst_size(5)
        .finish()
        .expect("Invalid strict governor configuration");

    cfg
    .service(
        web::scope("/health")
            .service(health::health)
    )
    .service(
        web::scope("/")
            .wrap(Governor::new(&standard_govnah))
            .service(nope::get_random_nope)
    );
}