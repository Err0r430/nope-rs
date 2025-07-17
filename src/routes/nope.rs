use actix_web::{get, web, HttpResponse};
use crate::require_api_key;
use crate::response;
use crate::config;
use std::sync::Arc;
use crate::nope_service::NopeService;
use crate::types::nope::NopeQuery;

const PERMISSION_LEVEL: config::PermissionLevel = config::PermissionLevel::Public;

#[get("")]
pub async fn get_random_nope(
    req: actix_web::HttpRequest,
    nope_service: web::Data<Arc<NopeService>>,
    query: web::Query<NopeQuery>,
) -> HttpResponse {
    require_api_key!(&req, PERMISSION_LEVEL);

    // Get a random nope, with optional language query.
    match nope_service.get_random_nope(query.lang.clone()) {
        Some(nope) => {
            return HttpResponse::Ok().json(
                response::make_query_response(
                    true,
                    Some(nope),
                    None,
                    Some("Successfully got random nope.")
                )
            );
        },
        None => {
            return HttpResponse::InternalServerError().json(
                response::make_query_response(
                    false,
                    Some(&"Error getting random nope: no nopes loaded"),
                    None,
                    None
                )
            );
        }
    };
}