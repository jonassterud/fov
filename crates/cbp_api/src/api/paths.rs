// API endpoints for the Coinbase Exchange/Pro API
// https://docs.cloud.coinbase.com/exchange/reference

use crate::api::models::*;
use crate::api::API;
use anyhow::{anyhow, Result};
use reqwest::{
    header::{self, HeaderName},
    StatusCode,
};

impl API {
    /// Get a currency
    pub async fn currencies_currency_id(&self, currency_id: &str) -> Result<Currency> {
        let resp = self
            .client
            .get(format!("https://api.exchange.coinbase.com/currencies/{}", currency_id))
            .header(
                header::USER_AGENT,
                "Mozilla/5.0 (X11; Linux x86_64; rv:101.0) Gecko/20100101 Firefox/101.0",
            )
            .header(header::ACCEPT, "application/json")
            .send()
            .await?;

        match resp.status() {
            StatusCode::OK => Ok(resp.json::<Currency>().await?),
            _ => Err(anyhow!(
                "Coinbase Pro API Error.\nHTTP Code: {}\nResponse: {}",
                resp.status(),
                resp.text().await?
            )),
        }
    }

    /// Get all accounts for a profile
    pub async fn accounts(&self) -> Result<Vec<Account>> {
        let timestamp = self.access_timestamp()?;
        let resp = self
            .client
            .get("https://api.exchange.coinbase.com/accounts")
            .header(header::ACCEPT, "application/json")
            .header(HeaderName::from_bytes(b"CB-ACCESS-KEY")?, self.access_key())
            .header(HeaderName::from_bytes(b"CB-ACCESS-TIMESTAMP")?, &timestamp.clone())
            .header(HeaderName::from_bytes(b"CB-ACCESS-PASSPHRASE")?, self.access_passphrase())
            .header(
                HeaderName::from_bytes(b"CB-ACCESS-SIGN")?,
                self.access_sign(&timestamp.clone(), "GET", "/accounts", "")?,
            )
            .header(
                header::USER_AGENT,
                "Mozilla/5.0 (X11; Linux x86_64; rv:101.0) Gecko/20100101 Firefox/101.0",
            )
            .send()
            .await?;

        match resp.status() {
            StatusCode::OK => Ok(resp.json::<Vec<Account>>().await?),
            _ => Err(anyhow!(
                "Coinbase Pro API Error.\nHTTP Code: {}\nResponse: {}",
                resp.status(),
                resp.text().await?
            )),
        }
    }

    /// Get product ticker
    pub async fn products_product_id_ticker(&self, product_id: &str) -> Result<ProductTicker> {
        let resp = self
            .client
            .get(format!("https://api.exchange.coinbase.com/products/{}/ticker", product_id))
            .header(
                header::USER_AGENT,
                "Mozilla/5.0 (X11; Linux x86_64; rv:101.0) Gecko/20100101 Firefox/101.0",
            )
            .header(header::ACCEPT, "application/json")
            .send()
            .await?;

        match resp.status() {
            StatusCode::OK => Ok(resp.json::<ProductTicker>().await?),
            _ => Err(anyhow!(
                "Coinbase Pro API Error.\nHTTP Code: {}\nResponse: {}",
                resp.status(),
                resp.text().await?
            )),
        }
    }
}
