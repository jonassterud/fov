use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    sb1_access_token: String,
    // ...
}

impl Config {
    pub fn from_file(path: &str) -> Result<Config> {
        let content = std::fs::read_to_string(path)?;
        let out = toml::from_str::<Config>(&content)?;

        Ok(out)
    }
}