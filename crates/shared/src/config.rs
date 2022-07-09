use anyhow::Result;
use pwbox::{sodium::Sodium, ErasedPwBox, Eraser, Suite};
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Config file for the application
#[derive(Deserialize, Serialize)]
pub struct Config {
    /// OAuth access token for the SpareBank 1 API (sb1_api)
    pub sb1_access_token: String,
    /// API key for the Coinbase Pro API (cbp_api)
    pub cbp_key: String,
    /// API key secret for the Coinbase Pro API (cbp_api)
    pub cbp_secret: String,
    /// API key passphrase for the Coinbase Pro API (cbp_api)
    pub cbp_passphrase: String,
    /// API key for NOWNodes (crypto_api)
    pub nwn_key: String,
    /// Bitcoin XPUB
    pub btc_xpub: String,
    /// Litecoin XPUB
    pub ltc_xpub: String,
}

impl Config {
    /// Returns a new empty `Config`
    pub fn new_empty() -> Config {
        Config {
            sb1_access_token: "".into(),
            cbp_key: "".into(),
            cbp_secret: "".into(),
            cbp_passphrase: "".into(),
            nwn_key: "".into(),
            btc_xpub: "".into(),
            ltc_xpub: "".into(),
        }
    }

    /// Create a `Config` based on a TOML configuration file and the current configuration
    ///
    /// # Arguments
    ///
    /// * `config_path` - path to the config (config.toml)
    /// * `secret_path` - path to the encrypted config (secret.toml)
    pub fn open(config_path: &PathBuf, secret_path: &PathBuf, password: &str) -> Result<Config> {
        // Open unencrypted config
        let config_content = std::fs::read(config_path)?;
        let config_unencrypted = toml::from_slice::<Config>(&config_content)?;

        // Open encrypted config
        let config_encrypted: Config;
        if secret_path.exists() {
            let secret_content = std::fs::read(secret_path)?;
            let pwbox = toml::from_slice(&secret_content)?;
            let pwbox = Eraser::new().add_suite::<Sodium>().restore(&pwbox)?;

            let decrypted = pwbox.open(password)?;
            config_encrypted = toml::from_slice::<Config>(&decrypted)?;
        } else {
            config_encrypted = Config::new_empty();
        }

        let out = Config::combine(config_encrypted, config_unencrypted);

        // Save encrypted config
        out.save_to_file(secret_path, password)?;

        // Reset unencrypted config
        std::fs::write(config_path, toml::to_string_pretty(&Config::new_empty())?)?;

        Ok(out)
    }

    /// Combine two `Config`'s but prioritize one over the other
    ///
    /// # Arguments
    ///
    /// * `og` - the config that should not be prioritized
    /// * `new` - the config that should be prioritized
    fn combine(og: Config, new: Config) -> Config {
        // TODO: Refactor
        let sb1_access_token = if new.sb1_access_token == "" {
            og.sb1_access_token
        } else {
            new.sb1_access_token
        };
        let cbp_key = if new.cbp_key == "" { og.cbp_key } else { new.cbp_key };
        let cbp_secret = if &new.cbp_secret == "" { og.cbp_secret } else { new.cbp_secret };
        let cbp_passphrase = if new.cbp_passphrase == "" {
            og.cbp_passphrase
        } else {
            new.cbp_passphrase
        };
        let nwn_key = if new.nwn_key == "" { og.nwn_key } else { new.nwn_key };
        let btc_xpub = if &new.btc_xpub == "" { og.btc_xpub } else { new.btc_xpub };
        let ltc_xpub = if new.ltc_xpub == "" { og.ltc_xpub } else { new.ltc_xpub };

        Config {
            sb1_access_token,
            cbp_key,
            cbp_secret,
            cbp_passphrase,
            nwn_key,
            btc_xpub,
            ltc_xpub,
        }
    }

    /// Encrypts and save `Config` to a file
    ///
    /// # Arguments
    ///
    /// * `path` - Where to save the file
    /// * `password` - Password to use for encryption
    fn save_to_file(&self, path: &PathBuf, password: &str) -> Result<()> {
        let pwbox = Sodium::build_box(&mut thread_rng()).seal(password, toml::to_string(self)?)?;
        let mut eraser = Eraser::new();
        eraser.add_suite::<Sodium>();
        let erased: ErasedPwBox = eraser.erase(&pwbox)?;

        let content = toml::to_string_pretty(&erased)?;
        std::fs::write(path, content)?;

        Ok(())
    }
}
