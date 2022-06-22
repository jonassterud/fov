pub mod models;

use anyhow::{anyhow, Result};
use models::accounts::*;

pub struct SpareBank1 {
    pub client: reqwest::Client,
    access_token: String,
}

impl SpareBank1 {
    pub async fn new(access_token: String) -> Result<SpareBank1> {
        let client = reqwest::Client::new();

        let url = "https://api.sparebank1.no/common/helloworld";
        let resp = client
            .get(url)
            .header(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", access_token),
            )
            .header(
                reqwest::header::ACCEPT,
                "application/vnd.sparebank1.v1+json;charset=utf-8",
            )
            .send()
            .await?;

        if resp.status() == reqwest::StatusCode::OK {
            Ok(SpareBank1 {
                client: client,
                access_token: access_token,
            })
        } else {
            Err(anyhow!(
                "Failed connecting to API, with code: {:?}",
                resp.status()
            ))
        }
    }

    pub async fn get_account_keys(&self) -> Result<AccountKeysDTO> {
        todo!()
    }

    pub async fn get_account_details(&self) -> Result<AccountDetailsDTO> {
        todo!()
    }

    pub async fn get_account(&self) -> Result<AccountDTO> {
        todo!()
    }

    pub async fn get_account_roles(&self) -> Result<AccountRolesDTO> {
        todo!()
    }

    pub async fn get_accounts(&self) -> Result<AccountsDTO> {
        let url = "https://api.sparebank1.no/personal/banking/accounts";

        let resp = self
            .client
            .get(url)
            .header(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", self.access_token),
            )
            .header(
                reqwest::header::ACCEPT,
                "application/vnd.sparebank1.v5+json;charset=utf-8",
            )
            .send()
            .await?;

        if resp.status() == reqwest::StatusCode::OK {
            Ok(resp.json::<AccountsDTO>().await?)
        } else {
            Err(anyhow!(
                "Failed connecting to API, with code: {:?}",
                resp.status()
            ))
        }
    }

    pub async fn get_account_cards(&self) -> Result<AccountRolesDTO> {
        todo!()
    }

    pub async fn get_default_payment_account(&self) -> Result<AccountDTO> {
        todo!()
    }

    pub async fn get_account_balance(&self) -> Result<AccountBalanceRequestDTO> {
        todo!()
    }
}
