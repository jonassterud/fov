use super::Portfolio;
use anyhow::{anyhow, Context, Result};
use shared::Asset;

impl Portfolio {
    /// Add Coinbase Pro assets to the portfolio
    pub async fn add_coinbasepro_assets(&mut self) -> Result<()> {
        let api = self.coinbasepro_api.as_ref().context("no coinbasepro_api")?;

        let mut temp_assets: Vec<Asset> = vec![];

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

            temp_assets.push(asset);
        }

        self.assets.push(("Coinbase Pro".into(), temp_assets));

        Ok(())
    }

    /// Add Nordnet assets to the portfolio
    pub async fn add_nordnet_asset(&mut self) {
        todo!();
    }

    /// Add SpareBank 1 assets to the portfolio
    pub async fn add_sparebank1_assets(&mut self) -> Result<()> {
        let api = self.sparebank1_api.as_ref().context("no sparebank1_api")?;

        let mut temp_assets: Vec<Asset> = vec![];

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

            temp_assets.push(asset);
        }

        self.assets.push(("SpareBank 1".into(), temp_assets));

        Ok(())
    }

    pub async fn add_nownodes_assets(&mut self) -> Result<()> {
        let api = self.nownodes_api.as_ref().context("no nownodes_api")?;
        let cg_api = self.coingecko_api.as_ref().context("no coingecko_api")?;

        let mut temp_assets: Vec<Asset> = vec![];

        // Get crypto prices
        let prices_nok = cg_api.simple_price_nok(vec!["bitcoin", "litecoin"]).await?;

        // Add Bitcoin assets
        if let Ok(utxos) = api.btc_utxo().await {
            let mut asset = Asset {
                name: "Bitcoin".into(),
                description: "".into(),
                balance: 0.0,
                currency: "BTC".into(),
                value: 0.0,
            };

            for utxo in utxos {
                asset.balance += utxo.value.unwrap().parse::<f64>()? / 100000000.0;
            }

            // Calculate value
            let nok_price = prices_nok
                .clone()
                .prices
                .context("no prices")?
                .get("bitcoin")
                .context("no bitcoin")?
                .nok
                .context("no nok")?;

            asset.value = asset.balance * nok_price;

            temp_assets.push(asset);
        }

        // Add Litecoin assets
        if let Ok(utxos) = api.ltc_utxo().await {
            let mut asset = Asset {
                name: "Litecoin".into(),
                description: "".into(),
                balance: 0.0,
                currency: "LTC".into(),
                value: 0.0,
            };

            for utxo in utxos {
                asset.balance += utxo.value.unwrap().parse::<f64>()? / 100000000.0;
            }

            // Calculate value
            let nok_price = prices_nok
                .clone()
                .prices
                .context("no prices")?
                .get("bitcoin")
                .context("no bitcoin")?
                .nok
                .context("no nok")?;

            asset.value = asset.balance * nok_price;

            temp_assets.push(asset);
        }

        self.assets.push(("Crypto".into(), temp_assets));

        Ok(())
    }
}
