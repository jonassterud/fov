// API endpoints for the CoinGecko API
// https://www.coingecko.com/en/api/documentation

use crate::api::models::*;
use crate::api::API;
use anyhow::{anyhow, Result};
use reqwest::{header, StatusCode};

impl API {
    /// Get a currency
    pub async fn simple_price_nok(&self, ids: Vec<&str>) -> Result<SimplePrice> {
        let resp = self
            .client
            .get(format!(
                "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=nok",
                ids.join(",").to_string()
            ))
            .header(
                header::USER_AGENT,
                "Mozilla/5.0 (X11; Linux x86_64; rv:101.0) Gecko/20100101 Firefox/101.0",
            )
            .header(header::ACCEPT, "application/json")
            .send()
            .await?;

        match resp.status() {
            StatusCode::OK => Ok(resp.json::<SimplePrice>().await?),
            _ => Err(anyhow!(
                "CoinGecko API Error.\nHTTP Code: {}\nResponse: {}",
                resp.status(),
                resp.text().await?
            )),
        }
    }
}
