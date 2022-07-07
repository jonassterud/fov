use super::Portfolio;
use anyhow::{anyhow, Result};
use shared::Asset;

impl Portfolio {
    /// Add Coinbase Pro assets to the portfolio
    pub async fn add_cbp_assets(&mut self) -> Result<()> {
        let api = self.cbp_api.as_ref().ok_or(anyhow!("cbp_api is None"))?;

        for account in api.accounts().await? {
            // Skip accounts with a balance of 0
            if account.balance == Some("0.0000000000000000".to_string()) {
                continue;
            }

            // Calculate asset value
            let currency_id = account.currency.as_ref().ok_or(anyhow!("Account missing currency"))?;
            let currency = api.currencies_currency_id(&currency_id).await?;
            let currency_details = currency.details.ok_or(anyhow!("Currency missing details"))?;
            let currency_type = currency_details._type.ok_or(anyhow!("Details missing type"))?;

            let value = match currency_type.as_ref() {
                "crypto" => {
                    let product_id = format!("{}-USD", &currency.id.ok_or(anyhow!("Currency missing id"))?);
                    let ticker = api.products_product_id_ticker(&product_id).await?;
                    // TODO: Get actual USDNOK value instead of using "10.0"
                    let ticker_price: f64 = ticker.price.ok_or(anyhow!("Ticker missing price"))?.parse::<f64>()?;
                    let account_balance = account.balance.as_ref().ok_or(anyhow!("Account missing balance"))?.parse::<f64>()?;
                    let asset_price = ticker_price * account_balance * 10.0;

                    Ok(asset_price)
                }
                "fiat" => {
                    Ok(0.0)
                    //todo!()
                }
                _ => Err(anyhow!("Unknown currency type")),
            }?;

            // Create and add asset
            let asset = Asset {
                name: currency.name.ok_or(anyhow!("Currency missing name"))?,
                description: None,
                balance: account.balance.ok_or(anyhow!("Account missing balance"))?.parse()?,
                currency: account.currency.ok_or(anyhow!("Account missing currency"))?,
                value: value,
            };

            self.cbp_assets.push(asset);
        }

        Ok(())
    }

    /// Add Nordnet assets to the portfolio
    pub async fn _add_nn_asset(&mut self) {
        todo!();
    }

    /// Add SpareBank 1 assets to the portfolio
    pub async fn add_sb1_assets(&mut self) -> Result<()> {
        let api = self.sb1_api.as_ref().ok_or(anyhow!("sb1_api is None"))?;

        for account in api
            .accounts(true, true, true, true, true, true, true)
            .await?
            .accounts
            .ok_or(anyhow!("AccountsDTO missing accounts"))?
        {
            let asset = Asset {
                name: account.name.ok_or(anyhow!("Account missing name"))?,
                description: account.description,
                balance: 0.0,
                currency: account.currency_code.ok_or(anyhow!("Account missing currency_code"))?,
                value: account.balance.ok_or(anyhow!("Account missing balance"))?,
            };

            self.sb1_assets.push(asset);
        }

        Ok(())
    }
}
