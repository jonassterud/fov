mod models;
mod paths;

use crate::portfolio::Asset;
use anyhow::Result;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct CoinbasePro {}

impl CoinbasePro {
    /// Create a `CoinbasePro` struct
    pub fn new() -> CoinbasePro {
        CoinbasePro {}
    }

    /// Get assets
    pub async fn get_assets(
        &self,
        key: &str,
        secret: &str,
        passphrase: &str,
    ) -> Result<Vec<Asset>> {
        let accounts = self.accounts(key, secret, passphrase).await?;
        let mut assets = vec![];

        for account in accounts {
            // Skip accounts with a balance of 0
            if account.balance == "0.0000000000000000" {
                continue;
            }

            assets.push(Asset {
                tag: "Coinbase Pro".into(),
                name: account.id,
                description: "".into(),
                balance: account.balance.parse()?,
                currency: account.currency,
                value: 0, // TODO (make it possible to choose currency in frontend)
            });
        }

        Ok(assets)
    }

    /// Get UNIX timestamp in seconds as string
    fn access_timestamp(&self) -> Result<String> {
        Ok(SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs()
            .to_string())
    }

    /// Get access sign
    fn access_sign(
        &self,
        timestamp: &str,
        method: &str,
        request_path: &str,
        body: &str,
        secret: &str,
    ) -> Result<String> {
        let message = format!("{}{}{}{}", timestamp, method, request_path, body);
        let secret_decoded = base64::decode(secret)?;
        let mut hmac: Hmac<Sha256> = Hmac::new_from_slice(&secret_decoded)?;

        hmac.update(message.as_bytes());
        let out = base64::encode(hmac.finalize().into_bytes());

        Ok(out)
    }
}
