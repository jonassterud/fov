use anyhow::Result;
use dialoguer::{Confirm, Password};
use pwbox::{sodium::Sodium, ErasedPwBox, Eraser, Suite};
use rand::thread_rng;
use serde::{Deserialize, Serialize};

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
    pub fn from_file(path: &str, password: &str) -> Result<Config> {
        let content = std::fs::read_to_string(path)?;
        let pwbox = toml::from_str(&content)?;
        let pwbox = Eraser::new().add_suite::<Sodium>().restore(&pwbox)?;

        let decrypted = pwbox.open(password)?;
        let config = toml::from_slice::<Config>(&decrypted)?;

        Ok(config)
    }

    /// Prompts the user for information and returns a Config
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

    /// Encrypts and save Config to a file
    pub fn save_to_file(&self, path: &str, password: &str) -> Result<()> {
        // Encrypt config
        let pwbox = Sodium::build_box(&mut thread_rng()).seal(password, toml::to_string(self)?)?;
        let mut eraser = Eraser::new();
        eraser.add_suite::<Sodium>();
        let erased: ErasedPwBox = eraser.erase(&pwbox)?;

        let content = toml::to_string_pretty(&erased)?;
        std::fs::write(path, content)?;

        Ok(())
    }
}
