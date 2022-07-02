// API endpoints for the SpareBank 1 Accounts API
// https://developer.sparebank1.no/#/api/2682DF86994D4B348363BE9AC4644EFC

use crate::api::models::*;
use crate::api::API;
use anyhow::{anyhow, Result};
use reqwest::{header, StatusCode, Url};

impl API {
    /// Retrieve account keys for account numbers
    /// 
    /// # Arguments
    /// 
    /// * `account_number` - List of account numbers
    pub async fn accounts_keys(&self, account_number: Vec<String>) -> Result<AccountKeysDTO> {
        let url = Url::parse_with_params(
            "https://api.sparebank1.no/personal/banking/accounts/keys",
            account_number.iter().map(|e| ("accountNumber", e)),
        )?;

        let resp = self
            .client
            .get(url)
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
            StatusCode::BAD_REQUEST => Err(anyhow!("{:?}", resp.json::<ErrorsDTO>().await?)),
            _ => Err(anyhow!(
                "SpareBank 1 API Error.\nHTTP Code: {}\nResponse: {}",
                resp.status(),
                resp.text().await?
            )),
        }
    }

    /// Get account
    /// 
    /// # Arguments
    /// 
    /// * `account_key` - Account key
    pub async fn accounts_account_key(&self, account_key: String) -> Result<AccountDTO> {
        let url = format!(
            "https://api.sparebank1.no/personal/banking/accounts/{}",
            account_key
        );

        let resp = self
            .client
            .get(url)
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
            StatusCode::UNAUTHORIZED => Err(anyhow!("{:?}", resp.json::<ErrorsDTO>().await?)),
            _ => Err(anyhow!(
                "SpareBank 1 API Error.\nHTTP Code: {}\nResponse: {}",
                resp.status(),
                resp.text().await?
            )),
        }
    }

    /// Get additional account details
    /// 
    /// # Arguments
    /// 
    /// * `account_key` - Account key
    pub async fn accounts_account_key_details(
        &self,
        account_key: String,
    ) -> Result<AccountDetailsDTO> {
        let url = format!(
            "https://api.sparebank1.no/personal/banking/accounts/{}/details",
            account_key
        );

        let resp = self
            .client
            .get(url)
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
            StatusCode::NOT_FOUND => Err(anyhow!("{:?}", resp.json::<ErrorsDTO>().await?)),
            _ => Err(anyhow!(
                "SpareBank 1 API Error.\nHTTP Code: {}\nResponse: {}",
                resp.status(),
                resp.text().await?
            )),
        }
    }

    /// Get account roles
    /// 
    /// # Arguments
    /// 
    /// * `account_key` - Account key
    pub async fn accounts_account_key_roles(&self, account_key: String) -> Result<AccountRolesDTO> {
        let url = format!(
            "https://api.sparebank1.no/personal/banking/accounts/{}/roles",
            account_key
        );

        let resp = self
            .client
            .get(url)
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
            StatusCode::NOT_FOUND => Err(anyhow!("{:?}", resp.json::<ErrorsDTO>().await?)),
            _ => Err(anyhow!(
                "SpareBank 1 API Error.\nHTTP Code: {}\nResponse: {}",
                resp.status(),
                resp.text().await?
            )),
        }
    }

    /// Get account cards
    /// 
    /// # Arguments
    /// 
    /// * `account_key` - Account key
    pub async fn accounts_account_key_cards(&self, account_key: String) -> Result<AccountRolesDTO> {
        let url = format!(
            "https://api.sparebank1.no/personal/banking/accounts/{}/cards",
            account_key
        );

        let resp = self
            .client
            .get(url)
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
            StatusCode::NOT_FOUND => Err(anyhow!("{:?}", resp.json::<ErrorsDTO>().await?)),
            _ => Err(anyhow!(
                "SpareBank 1 API Error.\nHTTP Code: {}\nResponse: {}",
                resp.status(),
                resp.text().await?
            )),
        }
    }

    /// List accounts entities
    /// 
    /// # Arguments
    /// 
    /// * `include_nok_accounts` - Specify whether the response should contain NOK accounts
    /// * `include_currency_accounts` - Specify whether the response should contain currency accounts
    /// * `include_bsu_accounts` - Specify whether the response should contain BSU accounts
    /// * `include_hidden_accounts` - Specify whether the response should contain Hidden accounts
    /// * `include_credit_card_accounts` - Specify whether the response should contain credit card accounts
    /// * `include_ask_accounts` - Specify whether the response should contain ASK accounts
    /// * `include_pension_accounts` - Specify whether the response should contain pension accounts
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
        //#![rustfmt::skip]
        let url = Url::parse_with_params(
            "https://api.sparebank1.no/personal/banking/accounts",
            &[
                ("includeNokAccounts", include_nok_accounts.to_string()),
                (
                    "includeCurrencyAccounts",
                    include_currency_accounts.to_string(),
                ),
                ("includeBsuAccounts", include_bsu_accounts.to_string()),
                ("includeHiddenAccounts", include_hidden_accounts.to_string()),
                (
                    "includeCreditCardAccounts",
                    include_credit_card_accounts.to_string(),
                ),
                ("includeAskAccounts", include_ask_accounts.to_string()),
                (
                    "includePensionAccounts",
                    include_pension_accounts.to_string(),
                ),
            ],
        )?;

        let resp = self
            .client
            .get(url)
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
                resp.text().await?
            )),
        }
    }

    /// Get default payment account
    pub async fn accounts_default(&self) -> Result<AccountDTO> {
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
                resp.text().await?
            )),
        }
    }

    /// Retrieve account balance
    pub async fn accounts_balance(_body: String) -> Result<AccountBalanceDTO> {
        todo!()
    }
}
