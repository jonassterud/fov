// API endpoints for the SpareBank 1 Accounts API
// https://developer.sparebank1.no/#/api/2682DF86994D4B348363BE9AC4644EFC

use crate::api::API;
use anyhow::{anyhow, Result};
use reqwest::{header, StatusCode};

impl API {
    pub async fn accounts_keys(&self, account_number: Vec<String>) -> Result<AccountKeysDTO> {
        // TODO: Add parameters to query in URL

        let resp = self
            .client
            .get("https://api.sparebank1.no/personal/banking/accounts/keys")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", self.access_token),
            )
            .header(
                header::ACCEPT,
                "application/vnd.sparebank1.v5+json;charset=utf-8",
            )
            .send()
            .await?;

        match resp.status() {
            StatusCode::OK => Ok(resp.json::<AccountKeysDTO>().await?),
            //TODO: StatusCode::400 => Err(ErrorsDTO...),
            _ => Err(anyhow!(
                "SpareBank 1 API Error.\nHTTP Code: {}\nResponse: {}",
                resp.status(),
                resp.text()
            )),
        }
    }

    pub async fn accounts_account_key(&self, account_key: String) -> Result<AccountDTO> {
        // TODO: Add parameters to query in URL

        let resp = self
            .client
            .get("https://api.sparebank1.no/personal/banking/accounts/{ACCOUNT_KEY_HERE}")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", self.access_token),
            )
            .header(
                header::ACCEPT,
                "application/vnd.sparebank1.v5+json;charset=utf-8",
            )
            .send()
            .await?;

        match resp.status() {
            StatusCode::OK => Ok(resp.json::<AccountDTO>().await?),
            //TODO: StatusCode::401 => Err(ErrorsDTO...),
            _ => Err(anyhow!(
                "SpareBank 1 API Error.\nHTTP Code: {}\nResponse: {}",
                resp.status(),
                resp.text()
            )),
        }
    }

    pub async fn accounts_account_key_details(&self, account_key: String) -> Result<AccountDetailsDTO> {
        // TODO: Add parameters to query in URL

        let resp = self
            .client
            .get("https://api.sparebank1.no/personal/banking/accounts/{ACCOUNT_KEY_HERE}/details")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", self.access_token),
            )
            .header(
                header::ACCEPT,
                "application/vnd.sparebank1.v5+json;charset=utf-8",
            )
            .send()
            .await?;

        match resp.status() {
            StatusCode::OK => Ok(resp.json::<AccountDetailsDTO>().await?),
            //TODO: StatusCode::404 => Err(ErrorsDTO...),
            _ => Err(anyhow!(
                "SpareBank 1 API Error.\nHTTP Code: {}\nResponse: {}",
                resp.status(),
                resp.text()
            )),
        }
    }

    pub async fn accounts_account_key_roles(&self, account_key: String) -> Result<AccountRolesDTO> {
        // TODO: Add parameters to query in URL

        let resp = self
            .client
            .get("https://api.sparebank1.no/personal/banking/accounts/{ACCOUNT_KEY_HERE}/roles")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", self.access_token),
            )
            .header(
                header::ACCEPT,
                "application/vnd.sparebank1.v5+json;charset=utf-8",
            )
            .send()
            .await?;

        match resp.status() {
            StatusCode::OK => Ok(resp.json::<AccountRolesDTO>().await?),
            //TODO: StatusCode::404 => Err(ErrorsDTO...),
            _ => Err(anyhow!(
                "SpareBank 1 API Error.\nHTTP Code: {}\nResponse: {}",
                resp.status(),
                resp.text()
            )),
        }
    }
    
    pub async fn accounts(
        &self,
        include_nok_accounts: bool,
        include_currency_accounts: bool,
        include_bsu_accounts: bool,
        include_hidden_accounts: bool,
        include_credit_card_accounts: bool,
        include_ask_accounts: bool,
        include_pension_accounts: bool,
    ) -> Result<AccountsDTO> {
        // TODO: Add parameters to query in URL

        let resp = self
            .client
            .get("https://api.sparebank1.no/personal/banking/accounts")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", self.access_token),
            )
            .header(
                header::ACCEPT,
                "application/vnd.sparebank1.v5+json;charset=utf-8",
            )
            .send()
            .await?;

        match resp.status() {
            StatusCode::OK => Ok(resp.json::<AccountsDTO>().await?),
            _ => Err(anyhow!(
                "SpareBank 1 API Error.\nHTTP Code: {}\nResponse: {}",
                resp.status(),
                resp.text()
            )),
        }
    }

    pub async fn accounts_account_key_cards(&self, account_key: String) -> Result<AccountRolesDTO> {
        // TODO: Add parameters to query in URL

        let resp = self
            .client
            .get("https://api.sparebank1.no/personal/banking/accounts/{ACCOUNT_KEY_HERE}/cards")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", self.access_token),
            )
            .header(
                header::ACCEPT,
                "application/vnd.sparebank1.v5+json;charset=utf-8",
            )
            .send()
            .await?;

        match resp.status() {
            StatusCode::OK => Ok(resp.json::<AccountRolesDTO>().await?),
            //TODO: StatusCode::404 => Err(ErrorsDTO...),
            _ => Err(anyhow!(
                "SpareBank 1 API Error.\nHTTP Code: {}\nResponse: {}",
                resp.status(),
                resp.text()
            )),
        }
    }

    pub async fn accounts_default(&self) -> Result<AccountDTO> {
        // TODO: Add parameters to query in URL

        let resp = self
            .client
            .get("https://api.sparebank1.no/personal/banking/accounts/default")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", self.access_token),
            )
            .header(
                header::ACCEPT,
                "application/vnd.sparebank1.v5+json;charset=utf-8",
            )
            .send()
            .await?;

        match resp.status() {
            StatusCode::OK => Ok(resp.json::<AccountDTO>().await?),
            _ => Err(anyhow!(
                "SpareBank 1 API Error.\nHTTP Code: {}\nResponse: {}",
                resp.status(),
                resp.text()
            )),
        }
    }

    pub async fn accounts_balance(body: String) -> Result<AccountBalanceDTO> {
        // TODO
    }
}
