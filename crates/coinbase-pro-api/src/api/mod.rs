mod endpoints;
pub mod models;

use anyhow::{anyhow, Result};
use hmac::{Hmac, Mac};
use models::*;
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct CoinbasePro {
    pub client: reqwest::Client,
    key: String,
    secret: String,
    passphrase: String,
}

impl CoinbasePro {
    pub fn new(key: String, secret: String, passphrase: String) -> Result<CoinbasePro> {
        let client = reqwest::Client::new();

        Ok(CoinbasePro {
            client: client,
            key: key,
            secret: secret,
            passphrase: passphrase,
        })
    }

    pub fn get_access_key(&self) -> &str {
        &self.key
    }

    pub fn get_access_sign(
        &self,
        timestamp: &str,
        method: &str,
        request_path: &str,
        body: &str,
    ) -> Result<String> {
        let message = format!("{}{}{}{}", timestamp, method, request_path, body);
        let secret = base64::decode(&self.secret)?;
        let mut hmac: Hmac<Sha256> = Hmac::new_from_slice(&secret)?;
        hmac.update(message.as_bytes());
        let out = base64::encode(hmac.finalize().into_bytes());

        Ok(out)
    }

    pub fn get_access_timestamp(&self) -> Result<String> {
        Ok(SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs()
            .to_string())
    }

    pub fn get_access_passphrase(&self) -> &str {
        &self.passphrase
    }
}
