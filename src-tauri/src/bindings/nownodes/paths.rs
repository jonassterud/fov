use super::{models, NowNodes};
use anyhow::{anyhow, Result};
use reqwest::{
    header::{self, HeaderName},
    StatusCode,
};

impl NowNodes {
    pub async fn btc_utxo(&self, key: &str, address: &str) -> Result<Vec<models::BtcUtxo>> {
        let client = reqwest::Client::new(); // TODO: big performance hit?

        let resp = client
            .get(format!(
                "https://btcbook.nownodes.io/api/v2/utxo/{}",
                address
            ))
            .header(HeaderName::from_bytes(b"api-key")?, key)
            .header(header::ACCEPT, "application/json")
            .send()
            .await?;

        match resp.status() {
            StatusCode::OK => Ok(resp.json::<Vec<models::BtcUtxo>>().await?),
            _ => Err(anyhow!(
                "NowNodes Pro API Error.\nHTTP Code: {}\nResponse: {}",
                resp.status(),
                resp.text().await?
            )),
        }
    }

    pub async fn ltc_utxo(&self, key: &str, address: &str) -> Result<Vec<models::LtcUtxo>> {
        let client = reqwest::Client::new(); // TODO: big performance hit?

        let resp = client
            .get(format!(
                "https://ltcbook.nownodes.io/api/v2/utxo/{}",
                address
            ))
            .header(HeaderName::from_bytes(b"api-key")?, key)
            .header(header::ACCEPT, "application/json")
            .send()
            .await?;

        match resp.status() {
            StatusCode::OK => Ok(resp.json::<Vec<models::LtcUtxo>>().await?),
            _ => Err(anyhow!(
                "NowNodes Pro API Error.\nHTTP Code: {}\nResponse: {}",
                resp.status(),
                resp.text().await?
            )),
        }
    }
}
