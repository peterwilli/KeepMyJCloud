use rocket::serde::{Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct InfoResponse {
    pub version: String
}

impl InfoResponse {
    pub fn new() -> Self {
        return InfoResponse {
            version: "0.1.0".to_string()
        };
    }
}