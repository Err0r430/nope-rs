use actix_web::{web, post, HttpResponse};
use entity::nopes;
use crate::db::postgres_service::PostgresService;
use crate::require_api_key;
use crate::response;
use crate::config;
use crate::types::nope::Nope;
use std::sync::Arc;
use sea_orm::SqlErr;

const PERMISSION_LEVEL: config::PermissionLevel = config::PermissionLevel::Ring0;

#[post("/create")]
async fn admin_nope_create(
    req: actix_web::HttpRequest,
    db: web::Data<Arc<PostgresService>>,
    nope: web::Json<Nope>,
) -> HttpResponse {
    require_api_key!(&req, PERMISSION_LEVEL);

    let id = crate::utils::id::generate_id();
    let nope = match db.create_nope(
        nopes::Model {
            id: id.clone(),
            language: nope.language.clone(),
            nope: nope.nope.clone()
        }
    ).await {
        Ok(nope) => nope,
        Err(e) => {
            match e.sql_err() {
                Some(SqlErr::UniqueConstraintViolation(_)) => {
                    return HttpResponse::Conflict().json(
                        response::make_query_response(
                            false,
                            Some(&"Yeah so that nope already exists... Gonna have to pass on that one chief"),
                            None,
                            None
                        )
                    );
                }
                _ => {
                    // We dont care about the other errors lmao. Just move on to standard 500.
                    }
            }
            return HttpResponse::InternalServerError().json(
                response::make_query_response(
                    false,
                    Some(&format!("Error creating nope: {}", e)),
                    None,
                    None
                )
            );
        }
    };



    HttpResponse::Ok().json(
        response::make_query_response(
            true,
            Some(&nope),
            None,
            Some("Successfully created nope.")
        )
    )
}