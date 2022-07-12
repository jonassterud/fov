use crate::bindings::*;
use crate::portfolio::SharedPortfolio;
use anyhow::Result;
use tauri::State;

#[tauri::command]
pub async fn update_assets(state: State<'_, SharedPortfolio>) -> Result<(), ()> {
    let mut portfolio = state.0.lock().await;

    // SpareBank 1
    if portfolio.config.sparebank_1_access_token.is_some() {
        let access_token = portfolio.config.sparebank_1_access_token.as_ref().unwrap();
        let mut assets = SpareBank1::new().get_assets(access_token).await.unwrap();

        portfolio.assets.append(&mut assets);
    }

    // Coinbase Pro
    if portfolio.config.coinbase_pro_key.is_some()
        && portfolio.config.coinbase_pro_secret.is_some()
        && portfolio.config.coinbase_pro_passphrase.is_some()
    {
        let key = portfolio.config.coinbase_pro_key.as_ref().unwrap();
        let secret = portfolio.config.coinbase_pro_secret.as_ref().unwrap();
        let passphrase = portfolio.config.coinbase_pro_passphrase.as_ref().unwrap();

        // ...
    }

    // NOWNodes
    if portfolio.config.nownodes_key.is_some() {
        let key = portfolio.config.nownodes_key.as_ref().unwrap();

        // ...
    }

    // Nordnet
    /*
    if portfolio.config.nordnet_key.is_some() {
        let key = portfolio.config.coinbase_pro_key.as_ref().unwrap();
        let secret = portfolio.config.coinbase_pro_secret.as_ref().unwrap();
        let passphrase = portfolio.config.coinbase_pro_passphrase.as_ref().unwrap();

        // ...
    }
    */

    Ok(())
}
