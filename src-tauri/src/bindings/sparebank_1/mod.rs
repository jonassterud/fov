mod models;
mod paths;

use crate::portfolio::Asset;
use anyhow::{anyhow, Result};

pub struct SpareBank1 {}

impl SpareBank1 {
    /// Create a `SpareBank1` struct
    pub fn new() -> SpareBank1 {
        SpareBank1 {}
    }

    /// Get assets
    pub async fn get_assets(&self, access_token: &str) -> Result<Vec<Asset>> {
        let accounts = self.accounts(&access_token).await?.accounts;
        let mut assets = vec![];

        for account in accounts {
            assets.push(Asset {
                name: account.name,
                description: account.description,
                balance: account.balance,
                currency: account.currency_code,
                value: 0, // TODO (make it possible to choose currency in frontend)
            });
        }

        Ok(assets)
    }
}
