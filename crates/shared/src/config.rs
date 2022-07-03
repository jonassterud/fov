use anyhow::Result;
use serde::Deserialize;

/// Config file for the application
#[derive(Deserialize)]
pub struct Config {
    /// OAuth access token for the SpareBank 1 API
    pub sb1_access_token: String,
    // ...
}

impl Config {
    /// Returns a Config based on a TOML configuration file
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the config file
    pub fn from_file(path: &str) -> Result<Config> {
        let content = std::fs::read_to_string(path)?;
        let out = toml::from_str::<Config>(&content)?;

        Ok(out)
    }
}
