pub mod models;

use anyhow::Result;
use std::collections::HashMap;

use self::models::Life360API;
use serde;

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

    Ok(Life360API{access_token:  res.json::<models::LoginResponse>().await?.access_token })
}

impl Life360API {
    /// Get a list of all circles
    pub async fn get_circles(&self) -> Result<Vec<models::Circle>> {
        let res = self.get::<models::CircleResponse>("circles").await;
        Ok(res?.circles)
    }

    /// Get a specific circle
    pub async fn get_circle(&self, circle_id: &str) -> Result<models::Circle> {
        let res = self.get::<models::Circle>(&format!("circles/{}", circle_id)).await;
        dbg!(res);
        todo!()
    }

    /// Make a GET request to the Life360 API
    async fn get<'a, R>(&self, path: &'a str) -> Result<R> where R: serde::de::DeserializeOwned {
        let url = format!("https://api-cloudfront.life360.com/v3/{}", path);
        let client = reqwest::Client::new();
        let res = client.get(url).header("Authorization", format!("Bearer {}", self.access_token)).send().await?;

        Ok(res.json::<R>().await?)
    }

}
