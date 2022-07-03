use anyhow::Result;
use serde::Deserialize;

/// Config file for the application
#[derive(Deserialize)]
pub struct Config {
    /// OAuth access token for the SpareBank 1 API
    pub sb1_access_token: String,
    /// API key for the Coinbase Pro API
    pub cbp_key: String,
    /// API key secret for the Coinbase Pro API
    pub cbp_secret: String,
    /// API key passphrase for the Coinbase Pro API
    pub cbp_passphrase: String,
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
