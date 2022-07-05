use anyhow::Result;
use serde::{Deserialize, Serialize};
use dialoguer::{Confirm, Password};

/// Config file for the application
#[derive(Deserialize, Serialize)]
pub struct Config {
    /// OAuth access token for the SpareBank 1 API
    pub sb1_access_token: Option<String>,
    /// API key for the Coinbase Pro API
    pub cbp_key: Option<String>,
    /// API key secret for the Coinbase Pro API
    pub cbp_secret: Option<String>,
    /// API key passphrase for the Coinbase Pro API
    pub cbp_passphrase: Option<String>,
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

    pub fn from_cli() -> Result<Config> {
        let sb1_active = Confirm::new().with_prompt("Enable SpareBank 1 API?").default(true).interact()?;
        let cbp_active = Confirm::new().with_prompt("Enable Coinbase Pro API?").default(true).interact()?;
        let nn_active = Confirm::new().with_prompt("Enable Nordnet API?").default(true).interact()?;

        let mut sb1_access_token: Option<String> = None;
        let mut cbp_key: Option<String> = None;
        let mut cbp_secret: Option<String> = None;
        let mut cbp_passphrase: Option<String> = None;

        if sb1_active {
            sb1_access_token = Some(Password::new().with_prompt("SpareBank 1 access token").interact()?);
        }

        if cbp_active {
            cbp_key = Some(Password::new().with_prompt("Coinbase Pro API key").interact()?);
            cbp_secret = Some(Password::new().with_prompt("Coinbase Pro API secret").interact()?);
            cbp_passphrase = Some(Password::new().with_prompt("Coinbase Pro API passphrase").interact()?);
        }

        if nn_active {
            // ...
        }

        Ok(Config {
            sb1_access_token,
            cbp_key,
            cbp_secret,
            cbp_passphrase,
        })
    }
}
