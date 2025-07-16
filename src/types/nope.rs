use serde::{Serialize, Deserialize};

// Json schema for the nope type
#[derive(Serialize, Deserialize, Debug)]
pub struct Nope {
    pub language: String,
    pub nope: String,
}