// API endpoints for the NOWNodes API
// https://documenter.getpostman.com/view/13630829/TVmFkLwy

use crate::api::models::*;
use crate::api::API;
use anyhow::{anyhow, Result};
use reqwest::{
    header::{self, HeaderName},
    StatusCode,
};

impl API {
    /// Get BTC UTXOs
    pub async fn btc_utxo(&self) -> Result<Vec<btc::Utxo>> {
        let resp = self
            .client
            .get(format!("https://btcbook.nownodes.io/api/v2/utxo/{}", self.btc_xpub))
            .header(HeaderName::from_bytes(b"api-key")?, self.key.clone())
            .header(header::ACCEPT, "application/json")
            .send()
            .await?;

        match resp.status() {
            StatusCode::OK => Ok(resp.json::<Vec<btc::Utxo>>().await?),
            _ => Err(anyhow!(
                "NOWNodes API Error.\nHTTP Code: {}\nResponse: {}",
                resp.status(),
                resp.text().await?
            )),
        }
    }

    /// Get LTC UTXOs
    ///
    /// # Arguments
    ///
    /// * `address` - Address or XPUB
    pub async fn ltc_utxo(&self) -> Result<Vec<ltc::Utxo>> {
        let resp = self
            .client
            .get(format!("https://ltcbook.nownodes.io/api/v2/utxo/{}", self.ltc_xpub))
            .header(HeaderName::from_bytes(b"api-key")?, self.key.clone())
            .header(header::ACCEPT, "application/json")
            .send()
            .await?;

        match resp.status() {
            StatusCode::OK => Ok(resp.json::<Vec<ltc::Utxo>>().await?),
            _ => Err(anyhow!(
                "NOWNodes API Error.\nHTTP Code: {}\nResponse: {}",
                resp.status(),
                resp.text().await?
            )),
        }
    }
}
