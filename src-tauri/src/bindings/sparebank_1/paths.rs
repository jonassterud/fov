use super::{models, SpareBank1};
use anyhow::{anyhow, Result};
use reqwest::{header, StatusCode};

impl SpareBank1 {
    pub async fn accounts(&self, access_token: &str) -> Result<models::AccountsDTO> {
        let client = reqwest::Client::new(); // TODO: big performance hit?

        let resp = client
            .get("https://api.sparebank1.no/personal/banking/accounts")
            .header(header::AUTHORIZATION, format!("Bearer {}", access_token))
            .header(
                header::ACCEPT,
                "application/vnd.sparebank1.v5+json;charset=utf-8",
            )
            .send()
            .await?;

        match resp.status() {
            StatusCode::OK => Ok(resp.json::<models::AccountsDTO>().await?),
            _ => Err(anyhow!(
                "SpareBank 1 API Error.\nHTTP Code: {}\nResponse: {}",
                resp.status(),
                resp.text().await?
            )),
        }
    }
}
