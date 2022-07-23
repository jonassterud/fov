use super::{models, CoinbasePro};
use anyhow::{anyhow, Result};
use reqwest::{
    header::{self, HeaderName},
    StatusCode,
};

impl CoinbasePro {
    pub async fn accounts(
        &self,
        key: &str,
        secret: &str,
        passphrase: &str,
    ) -> Result<Vec<models::Account>> {
        let client = reqwest::Client::new(); // TODO: big performance hit?

        let timestamp = self.access_timestamp()?;
        let resp = client
            .get("https://api.exchange.coinbase.com/accounts")
            .header(header::ACCEPT, "application/json")
            .header(HeaderName::from_bytes(b"CB-ACCESS-KEY")?, key)
            .header(
                HeaderName::from_bytes(b"CB-ACCESS-TIMESTAMP")?,
                &timestamp.clone(),
            )
            .header(HeaderName::from_bytes(b"CB-ACCESS-PASSPHRASE")?, passphrase)
            .header(
                HeaderName::from_bytes(b"CB-ACCESS-SIGN")?,
                self.access_sign(&timestamp.clone(), "GET", "/accounts", "", secret)?,
            )
            .header(
                header::USER_AGENT,
                "Mozilla/5.0 (X11; Linux x86_64; rv:101.0) Gecko/20100101 Firefox/101.0",
            )
            .send()
            .await?;

        match resp.status() {
            StatusCode::OK => Ok(resp.json::<Vec<models::Account>>().await?),
            _ => Err(anyhow!(
                "Coinbase Pro API Error.\nHTTP Code: {}\nResponse: {}",
                resp.status(),
                resp.text().await?
            )),
        }
    }
}
