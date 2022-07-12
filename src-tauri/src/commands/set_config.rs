use crate::portfolio::SharedPortfolio;
use anyhow::Result;
use tauri::State;

#[tauri::command]
pub async fn set_sparebank_1_access_token(
    state: State<'_, SharedPortfolio>,
    value: String,
) -> Result<(), ()> {
    let mut portfolio = state.0.lock().await;
    portfolio.config.sparebank_1_access_token = Some(value);

    Ok(())
}

#[tauri::command]
pub async fn set_coinbase_pro_key(
    state: State<'_, SharedPortfolio>,
    value: String,
) -> Result<(), ()> {
    let mut portfolio = state.0.lock().await;
    portfolio.config.coinbase_pro_key = Some(value);

    Ok(())
}

#[tauri::command]
pub async fn set_coinbase_pro_secret(
    state: State<'_, SharedPortfolio>,
    value: String,
) -> Result<(), ()> {
    let mut portfolio = state.0.lock().await;
    portfolio.config.coinbase_pro_secret = Some(value);

    Ok(())
}

#[tauri::command]
pub async fn set_coinbase_pro_passphrase(
    state: State<'_, SharedPortfolio>,
    value: String,
) -> Result<(), ()> {
    let mut portfolio = state.0.lock().await;
    portfolio.config.coinbase_pro_passphrase = Some(value);

    Ok(())
}

#[tauri::command]
pub async fn set_nownodes_key(state: State<'_, SharedPortfolio>, value: String) -> Result<(), ()> {
    let mut portfolio = state.0.lock().await;
    portfolio.config.nownodes_key = Some(value);

    Ok(())
}

#[tauri::command]
pub async fn set_btc_addresses(
    state: State<'_, SharedPortfolio>,
    value: Vec<String>,
) -> Result<(), ()> {
    let mut portfolio = state.0.lock().await;
    portfolio.config.btc_addresses = Some(value);

    Ok(())
}

#[tauri::command]
pub async fn set_ltc_addresses(
    state: State<'_, SharedPortfolio>,
    value: Vec<String>,
) -> Result<(), ()> {
    let mut portfolio = state.0.lock().await;
    portfolio.config.ltc_addresses = Some(value);

    Ok(())
}
