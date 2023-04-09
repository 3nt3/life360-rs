use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub life360: Life360Config,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Life360Config {
    pub username: String,
    pub password: String,
}
