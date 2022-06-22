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

    pub async fn get_accounts(&self) -> Result<Vec<Account>> {
        let url = "https://api.exchange.coinbase.com/accounts";

        let timestamp = self.get_access_timestamp()?;

        let resp = self
            .client
            .get(url)
            .header(
                reqwest::header::HeaderName::from_bytes(b"CB-ACCESS-KEY")?,
                self.get_access_key(),
            )
            .header(
                reqwest::header::HeaderName::from_bytes(b"CB-ACCESS-SIGN")?,
                self.get_access_sign(&timestamp.clone(), "GET", "/accounts", "")?,
            )
            .header(
                reqwest::header::HeaderName::from_bytes(b"CB-ACCESS-TIMESTAMP")?,
                &timestamp.clone(),
            )
            .header(
                reqwest::header::HeaderName::from_bytes(b"CB-ACCESS-PASSPHRASE")?,
                self.get_access_passphrase(),
            )
            .header(
                reqwest::header::USER_AGENT,
                "Mozilla/5.0 (X11; Linux x86_64; rv:101.0) Gecko/20100101 Firefox/101.0",
            )
            .header(reqwest::header::ACCEPT, "application/json")
            .send()
            .await?;

        if resp.status() == reqwest::StatusCode::OK {
            Ok(resp.json::<Vec<Account>>().await?)
        } else {
            Err(anyhow!(
                "Failed connecting to API, with code: {:?}",
                resp.status()
            ))
        }
    }

    pub async fn get_product_ticker(&self, product_id: &str) -> Result<ProductTicker> {
        let url = format!(
            "https://api.exchange.coinbase.com/products/{}/ticker",
            product_id
        );

        let resp = self
            .client
            .get(url)
            .header(
                reqwest::header::USER_AGENT,
                "Mozilla/5.0 (X11; Linux x86_64; rv:101.0) Gecko/20100101 Firefox/101.0",
            )
            .header(reqwest::header::ACCEPT, "application/json")
            .send()
            .await?;

        if resp.status() == reqwest::StatusCode::OK {
            Ok(resp.json::<ProductTicker>().await?)
        } else {
            Err(anyhow!(
                "Failed connecting to API, with code: {:?}, on product_id: {:?}",
                resp.status(), product_id
            ))
        }
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
