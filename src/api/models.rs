use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
}

#[derive(Debug)]
pub struct Life360API {
    pub access_token: String,
}

#[derive(Debug, Deserialize)]
pub struct Circle {
    pub id: String,
    pub name: String,
    pub color: String,
}

#[derive(Debug, Deserialize)]
pub struct CircleResponse {
    pub circles: Vec<Circle>,
}
