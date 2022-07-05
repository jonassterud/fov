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
            let currency = api
                .currencies_currency_id(&account.currency.clone().unwrap())
                .await?;
            let value = match currency.details.unwrap()._type.unwrap().as_ref() {
                "crypto" => {
                    let product_id = format!("{}-USD", &currency.id.unwrap());
                    let ticker = api.products_product_id_ticker(&product_id).await?;
                    // TODO: Get actual USDNOK value instead of using "10.0"
                    let asset_price: f64 = ticker.price.unwrap().parse::<f64>()?
                        * account.balance.clone().unwrap().parse::<f64>()?
                        * 10.0;

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
                name: currency.name.unwrap(),
                description: None,
                balance: account.balance.unwrap().parse()?,
                currency: account.currency.unwrap(),
                value: value,
            };

            self.cbp_assets.push(asset);
        }

        Ok(())
    }

    /// Add Nordnet assets to the portfolio
    pub async fn update_nn_assets(&mut self) {
        todo!();
    }

    /// Add SpareBank 1 assets to the portfolio
    pub async fn add_sb1_assets(&mut self) -> Result<()> {
        let api = self.sb1_api.as_ref().ok_or(anyhow!("sb1_api is None"))?;

        for account in api
            .accounts(true, true, true, true, true, true, true)
            .await?
            .accounts
            .unwrap()
        {
            let asset = Asset {
                name: account.name.unwrap(),
                description: account.description,
                balance: 0.0,
                currency: account.currency_code.unwrap(),
                value: account.balance.unwrap(),
            };

            self.sb1_assets.push(asset);
        }

        Ok(())
    }
}
