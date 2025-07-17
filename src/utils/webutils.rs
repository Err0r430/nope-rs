use actix_web::{HttpRequest, HttpResponse};
use crate::config::{EnvConfig};
use crate::config::PermissionLevel;
use crate::config;
use crate::response;


pub struct WebUtils;

impl WebUtils {
    // Extract the API key from the request headers

    pub fn extract_api_key(req: &HttpRequest) -> Option<String> {
    req.headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|value| {
            value.strip_prefix("Bearer ")
                .or_else(|| value.strip_prefix("Bearer%20"))
        })
        .map(|s| s.to_string())
    }

    // Check the api key against the configured api key.

    pub fn check_api_key(req: &HttpRequest, required: PermissionLevel) -> bool {
        // Public routes are always open
        if required == PermissionLevel::Public {
            return true;
        }

        let cfg = EnvConfig::from_env();
        let token = match Self::extract_api_key(req) {
            Some(t) => t,
            None => return false,
        };

        // Figure out which ring the client key belongs to
        let client = if token == cfg.keys.ring0 {
            PermissionLevel::Ring0
        } else if token == cfg.keys.ring1 {
            PermissionLevel::Ring1
        } else if token == cfg.keys.ring2 {
            PermissionLevel::Ring2
        } else {
            return false;
        };

        // Allow if client ring is at least as privileged as required
        client <= required
    }

    pub fn require_api_key(
        req: &HttpRequest,
        level: config::PermissionLevel
    ) -> Option<HttpResponse> {
        if WebUtils::check_api_key(req, level) {
            None
        } else {
            Some(
                HttpResponse::Unauthorized().json(
                    response::make_query_response::<()>(
                        false,
                        None,
                        Some("Unauthorized"),
                        None
                    )
                )
            )
        }
    }

    
}
