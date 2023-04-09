use std::collections::HashMap;
use toml;

mod api;
mod models;

use anyhow::Result;
// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
//

#[tokio::main]
async fn main() -> Result<()> {
    let config_file = std::fs::read_to_string("config.toml")?;
    let config: models::Config = toml::from_str(&config_file)?;

    let access_token = api::login(&config.life360.username, &config.life360.password).await?;
    dbg!(access_token);

    Ok(())
}

