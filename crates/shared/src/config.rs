use anyhow::Result;
use dialoguer::{Confirm, Password};
use pwbox::{sodium::Sodium, ErasedPwBox, Eraser, Suite};
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Config file for the application
#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    /// OAuth access token for the SpareBank 1 API (sb1_api)
    pub sb1_access_token: Option<String>,
    /// API key for the Coinbase Pro API (cbp_api)
    pub cbp_key: Option<String>,
    /// API key secret for the Coinbase Pro API (cbp_api)
    pub cbp_secret: Option<String>,
    /// API key passphrase for the Coinbase Pro API (cbp_api)
    pub cbp_passphrase: Option<String>,
    /// API key for NOWNodes (crypto_api)
    pub nwn_key: Option<String>,
    /// Bitcoin XPUB
    pub btc_xpub: Option<String>,
    /// Litecoin XPUB
    pub ltc_xpub: Option<String>,
}

impl Config {
    /// Returns a new empty `Config`
    pub fn new_empty() -> Config {
        Config {
            sb1_access_token: None,
            cbp_key: None,
            cbp_secret: None,
            cbp_passphrase: None,
            nwn_key: None,
            btc_xpub: None,
            ltc_xpub: None,
        }
    }

    /// Returns a `Config` based on a TOML configuration file
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the config file
    /// * `password` - Password to decrypt file
    pub fn from_file(path: PathBuf, password: &str) -> Result<Config> {
        let content = std::fs::read_to_string(path)?;
        let pwbox = toml::from_str(&content)?;
        let pwbox = Eraser::new().add_suite::<Sodium>().restore(&pwbox)?;

        let decrypted = pwbox.open(password)?;
        let config = toml::from_slice::<Config>(&decrypted)?;

        println!("{:?}", config);

        Ok(config)
    }

    /// Prompts the user for information and returns a `Config`
    pub fn from_cli() -> Result<Config> {
        let sb1_active = Confirm::new().with_prompt("Enable SpareBank 1 API?").default(true).interact()?;
        let cbp_active = Confirm::new().with_prompt("Enable Coinbase Pro API?").default(true).interact()?;
        let nn_active = Confirm::new().with_prompt("Enable Nordnet API?").default(true).interact()?;
        let nwn_active = Confirm::new().with_prompt("Enable NOWNodes API?").default(true).interact()?;

        let mut config = Config::new_empty();

        if sb1_active {
            config.sb1_access_token = Some(Password::new().with_prompt("SpareBank 1 access token").interact()?);
        }

        if cbp_active {
            config.cbp_key = Some(Password::new().with_prompt("Coinbase Pro API key").interact()?);
            config.cbp_secret = Some(Password::new().with_prompt("Coinbase Pro API secret").interact()?);
            config.cbp_passphrase = Some(Password::new().with_prompt("Coinbase Pro API passphrase").interact()?);
        }

        if nn_active {
            // ...
        }

        if nwn_active {
            config.nwn_key = Some(Password::new().with_prompt("NOWNodes API key").interact()?);
            config.btc_xpub = Some(Password::new().with_prompt("Bitcoin XPUB").interact()?);
            config.ltc_xpub = Some(Password::new().with_prompt("Litecoin XPUB").interact()?);
        }

        Ok(config)
    }

    /// Encrypts and save `Config` to a file
    ///
    /// # Arguments
    ///
    /// * `path` - Where to save the file
    /// * `password` - Password to use for encryption
    pub fn save_to_file(&self, path: PathBuf, password: &str) -> Result<()> {
        let pwbox = Sodium::build_box(&mut thread_rng()).seal(password, toml::to_string(self)?)?;
        let mut eraser = Eraser::new();
        eraser.add_suite::<Sodium>();
        let erased: ErasedPwBox = eraser.erase(&pwbox)?;

        let content = toml::to_string_pretty(&erased)?;
        std::fs::write(path, content)?;

        Ok(())
    }
}
