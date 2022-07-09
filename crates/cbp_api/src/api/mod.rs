use anyhow::Result;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

mod models;
mod paths;

/// API struct to connect to Coinbase Pro API
pub struct API {
    client: reqwest::Client,
    key: String,
    secret: String,
    passphrase: String,
}

impl API {
    /// Creates a new `API` struct
    ///
    /// # Arguments
    ///
    /// * `key` - API key
    /// * `secret` - API secret
    /// * ``passphrase - API passphrase
    pub fn new(key: &str, secret: &str, passphrase: &str) -> API {
        API {
            client: reqwest::Client::new(),
            key: key.into(),
            secret: secret.into(),
            passphrase: passphrase.into(),
        }
    }

    /// Create value for the `CB-ACCESS-SIGN` header
    ///
    /// # Arguments
    ///
    /// * `timestamp` - UNIX timestamp in seconds (which can be generated using `access_timestamp`)
    /// * `method` - HTTP method
    /// * `request_path` - endpoint
    /// * `body` - body to send
    pub fn access_sign(&self, timestamp: &str, method: &str, request_path: &str, body: &str) -> Result<String> {
        let message = format!("{}{}{}{}", timestamp, method, request_path, body);
        let secret_decoded = base64::decode(&self.secret)?;
        let mut hmac: Hmac<Sha256> = Hmac::new_from_slice(&secret_decoded)?;

        hmac.update(message.as_bytes());
        let out = base64::encode(hmac.finalize().into_bytes());

        Ok(out)
    }

    /// Get UNIX timestamp in seconds as a String
    pub fn access_timestamp(&self) -> Result<String> {
        Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs().to_string())
    }

    /// Get the key
    pub fn access_key(&self) -> &str {
        &self.key
    }

    /// Get the passphrase
    pub fn access_passphrase(&self) -> &str {
        &self.passphrase
    }
}
