use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Nope {
    pub language: String,
    pub nope: String,
}

#[derive(Debug, Deserialize)]
pub struct NopeQuery {
    pub lang: Option<String>,
}