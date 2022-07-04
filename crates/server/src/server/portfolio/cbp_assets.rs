use super::Portfolio;
use anyhow::{anyhow, Result};
use shared::Asset;

// TODO: Remove Into trait, and just do it directly in the update_cbp_assets function.
// Do this for all asset types.

impl Portfolio {
    /// Update Coinbase Pro assets
    pub async fn update_cbp_assets(&mut self) -> Result<()> {
        if let Some(api) = &self.cbp_api {
            let raw_accounts = api.accounts().await?;
            let mut accounts = vec![];

            for mut account in raw_accounts {
                // Filter out assets with a balance of 0.0
                let balance: f64 = account
                    .balance
                    .clone()
                    .unwrap_or("0.0".to_string())
                    .parse()?;
                if balance > 0.0 {
                    // Get name of asset
                    let currency = api
                        .currencies_currency_id(
                            &account.currency.clone().expect("Account missing name"),
                        )
                        .await?;
                    account.id = currency.name;

                    // Get value of asset
                    let value = match currency.details.unwrap()._type.unwrap().as_ref() {
                        "crypto" => {
                            let product_id = format!("{}-USD", &currency.id.unwrap());
                            let ticker = api.products_product_id_ticker(&product_id).await?;
                            let ticker_price: f64 = ticker.price.unwrap().parse()?;
                            let asset_price_usd = ticker_price * balance;
                            let asset_price_nok = asset_price_usd * 10.0; // TODO: get USDNOK

                            asset_price_nok
                        },
                        "fiat" => {
                            0.0 // TODO
                        },
                        _ => {
                            panic!("here");
                        }
                    };

                    // Add asset to accounts
                    accounts.push(account);
                }
            }

            self.add_to_cbp_assets(accounts);

            Ok(())
        } else {
            Err(anyhow!(
                "Failed updating cbp_assets because no cbp_api could be found."
            ))
        }
    }

    /// Transform and add Coinbase Pro assets to portfolio
    ///
    /// # Arguments
    ///
    /// * `assets` - A vector containing elements that implement the `Into<Asset>` trait
    fn add_to_cbp_assets(&mut self, assets: Vec<impl Into<Asset> + Clone>) {
        self.cbp_assets
            .append(&mut assets.iter().map(|x| x.clone().into()).collect());
    }
}
