use super::Portfolio;
use anyhow::{anyhow, Context, Result};
use shared::Asset;

impl Portfolio {
    /// Add Coinbase Pro assets to the portfolio
    pub async fn add_cbp_assets(&mut self) -> Result<()> {
        let api = self.cbp_api.as_ref().context("no cbp_api")?;

        for account in api.accounts().await? {
            // Skip accounts with a balance of 0
            if account.balance == Some("0.0000000000000000".into()) {
                continue;
            }

            // Calculate asset value
            let currency_id = account.currency.as_ref().context("no currency")?;
            let currency = api.currencies_currency_id(&currency_id).await?;
            let currency_details = currency.details.context("no details")?;
            let currency_type = currency_details._type.context("no type")?;

            let value = match currency_type.as_ref() {
                "crypto" => {
                    let product_id = format!("{}-USD", &currency.id.context("no id")?);
                    let ticker = api.products_product_id_ticker(&product_id).await?;
                    // TODO: Get actual USDNOK value instead of using "10.0"
                    let ticker_price: f64 = ticker.price.context("no price")?.parse::<f64>()?;
                    let account_balance = account.balance.as_ref().context("no balance")?.parse::<f64>()?;
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
                name: currency.name.context("no name")?,
                description: "".into(),
                balance: account.balance.context("no balance")?.parse()?,
                currency: account.currency.context("no currency")?,
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
        let api = self.sb1_api.as_ref().context("no sb1_api")?;

        for account in api
            .accounts(true, true, true, true, true, true, true)
            .await?
            .accounts
            .context("no accounts")?
        {
            let asset = Asset {
                name: account.name.context("no name")?,
                description: account.description.unwrap_or("".into()),
                balance: 0.0,
                currency: account.currency_code.context("no currency_code")?,
                value: account.balance.context("no balance")?,
            };

            self.sb1_assets.push(asset);
        }

        Ok(())
    }

    /// Add Bitcoin assets to the portfolio
    pub async fn add_btc_crypto_assets(&mut self) -> Result<()> {
        let api = self.nwn_api.as_ref().context("no nwn_api")?;

        let mut asset = Asset {
            name: "Bitcoin".into(),
            description: "".into(),
            balance: 0.0,
            currency: "BTC".into(),
            value: 0.0,
        };

        for utxo in api.btc_utxo().await? {
            asset.balance += utxo.value.unwrap().parse::<f64>()? / 100000000.0;
        }

        self.crypto_assets.push(asset);

        Ok(())
    }

    /// Add Litecoin assets to the portfolio
    pub async fn add_ltc_crypto_assets(&mut self) -> Result<()> {
        let api = self.nwn_api.as_ref().context("no nwn_api")?;

        let mut asset = Asset {
            name: "Litecoin".into(),
            description: "".into(),
            balance: 0.0,
            currency: "LTC".into(),
            value: 0.0,
        };

        for utxo in api.ltc_utxo().await? {
            asset.balance += utxo.value.unwrap().parse::<f64>()? / 100000000.0;
        }

        self.crypto_assets.push(asset);

        Ok(())
    }
}
