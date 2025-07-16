use actix_web::{get, HttpResponse};
use crate::require_api_key;
use crate::response;
use crate::config;

const PERMISSION_LEVEL: config::PermissionLevel = config::PermissionLevel::Public;

#[get("")]
async fn health(
    req: actix_web::HttpRequest
) -> HttpResponse {
    require_api_key!(&req, PERMISSION_LEVEL);

    HttpResponse::Ok().json(
        response::make_query_response(
            true,
            Some(&"Endpoints are healthy!"),
            None,
            Some("Server is healthy!")
        )
    )
}