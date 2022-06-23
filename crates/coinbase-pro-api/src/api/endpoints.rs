use crate::api::models::*;
use crate::CoinbasePro;
use anyhow::{anyhow, Result};

impl CoinbasePro {
    pub async fn get_accounts(&self) -> Result<Vec<accounts::Account>> {
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
            Ok(resp.json::<Vec<accounts::Account>>().await?)
        } else {
            Err(anyhow!(
                "Failed connecting to API, with code: {:?}",
                resp.status()
            ))
        }
    }

    pub async fn get_product_ticker(&self, product_id: &str) -> Result<products::ProductTicker> {
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
            Ok(resp.json::<products::ProductTicker>().await?)
        } else {
            Err(anyhow!(
                "Failed connecting to API, with code: {:?}, on product_id: {:?}",
                resp.status(),
                product_id
            ))
        }
    }
}
