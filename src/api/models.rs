use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
}
