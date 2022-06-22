mod portfolio;

use coinbase_pro_api::*;
use portfolio::Portfolio;
use serde::Deserialize;
use sparebank1_api::*;

#[derive(Debug, Deserialize)]
pub struct Secret {
    pub sparebank1_access_token: String,
    pub sparebank1_refresh_token: String,
    pub coinbase_pro_key: String,
    pub coinbase_pro_secret: String,
    pub coinbase_pro_passphrase: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut portfolio = Portfolio::new();

    let content = std::fs::read_to_string("src/secret.toml")?;
    let secret = toml::from_str::<Secret>(&content)?;

    // Add SpareBank 1 accounts
    let sparebank1 = SpareBank1::new(secret.sparebank1_access_token).await?;
    let sparebank1_accounts = sparebank1.get_accounts().await?;

    portfolio.add_sparebank1_accounts(sparebank1_accounts)?;

    // Add Coinbase Pro accounts
    let coinbase_pro = CoinbasePro::new(
        secret.coinbase_pro_key,
        secret.coinbase_pro_secret,
        secret.coinbase_pro_passphrase,
    )?;
    let coinbase_pro_accounts = coinbase_pro.get_accounts().await?;

    portfolio
        .add_coinbase_pro_accounts(coinbase_pro_accounts, &coinbase_pro)
        .await?;


    println!("{:?}", portfolio);
    Ok(())
}
