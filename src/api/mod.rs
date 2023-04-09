pub mod models;

use anyhow::Result;
use std::collections::HashMap;

pub async fn login<S>(email: S, password: S) -> Result<String>
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

    Ok(res.json::<models::LoginResponse>().await?.access_token)
}
