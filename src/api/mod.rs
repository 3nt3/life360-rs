//! Life360 API things
//!
//! To get started, see [`fn@login`].
//!
//! For a list of possible things to do, see [`Life360API`].

use anyhow::Result;
use std::collections::HashMap;

use serde;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    access_token: String,
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

/// Login to Life360 and return an API Client
pub async fn login<S>(email: S, password: S) -> Result<Life360API>
where
    S: Into<String>,
{
    let url = "https://api-cloudfront.life360.com/v3/oauth2/token";
    let client = reqwest::Client::new();

    let email = email.into();
    let password = password.into();

    let mut body = HashMap::new();
    body.insert("grant_type", "password");
    body.insert("username", &email);
    body.insert("password", &password);

    let res = client.post(url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("User-Agent", "SafetyMapKoko/22.2.0.487/CBC47A39-34C3-43F2-9924-E7F1F928AC1C")
        .header("Authorization", "Basic YnJ1czR0ZXZhcHV0UmVadWNydUJSVXdVYnJFTUVDN1VYZTJlUEhhYjpSdUt1cHJBQ3JhbWVzV1UydVRyZVF1bXVtYTdhemFtQQ==")
        .json(&body)
        .send()
        .await?;

    Ok(Life360API{access_token:  res.json::<LoginResponse>().await?.access_token })
}

impl Life360API {
    /// Get a list of all circles
    pub async fn get_circles(&self) -> Result<Vec<Circle>> {
        let res = self.get::<CircleResponse>("circles").await;
        Ok(res?.circles)
    }

    /// Get a specific circle
    pub async fn get_circle(&self, circle_id: &str) -> Result<Circle> {
        Ok(self.get::<Circle>(&format!("circles/{}", circle_id)).await?)
    }
    /// Make a GET request to the Life360 API
    async fn get<'a, R>(&self, path: &'a str) -> Result<R> where R: serde::de::DeserializeOwned {
        let url = format!("https://api-cloudfront.life360.com/v3/{}", path);
        let client = reqwest::Client::new();
        let res = client.get(url).header("Authorization", format!("Bearer {}", self.access_token)).send().await?;

        Ok(res.json::<R>().await?)
    }

}
