use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
}

#[derive(Debug)]
pub struct Life360API {
    pub access_token: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Circle {
    pub id: String,
    pub name: String,
    pub color: String,
    pub members: Option<Vec<Member>>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub location: Option<Location>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Location {
    pub latitude: String,
    pub longitude: String,
    pub accuracy: String,
    pub battery: String,
    pub speed: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CircleResponse {
    pub circles: Vec<Circle>,
}
