use crate::portfolio::SharedPortfolio;
use tauri::State;

#[tauri::command]
pub fn set_sparebank_1_access_token(state: State<'_, SharedPortfolio>, value: String) {
    let mut portfolio = state.0.lock().unwrap();
    portfolio.config.sparebank_1_access_token = Some(value);
}

#[tauri::command]
pub fn set_coinbase_pro_key(state: State<'_, SharedPortfolio>, value: String) {
    let mut portfolio = state.0.lock().unwrap();
    portfolio.config.coinbase_pro_key = Some(value);
}

#[tauri::command]
pub fn set_coinbase_pro_secret(state: State<'_, SharedPortfolio>, value: String) {
    let mut portfolio = state.0.lock().unwrap();
    portfolio.config.coinbase_pro_secret = Some(value);
}

#[tauri::command]
pub fn set_coinbase_pro_passphrase(state: State<'_, SharedPortfolio>, value: String) {
    let mut portfolio = state.0.lock().unwrap();
    portfolio.config.coinbase_pro_passphrase = Some(value);
}

#[tauri::command]
pub fn set_nownodes_key(state: State<'_, SharedPortfolio>, value: String) {
    let mut portfolio = state.0.lock().unwrap();
    portfolio.config.nownodes_key = Some(value);
}

#[tauri::command]
pub fn set_btc_addresses(state: State<'_, SharedPortfolio>, value: Vec<String>) {
    let mut portfolio = state.0.lock().unwrap();
    portfolio.config.btc_addresses = Some(value);
}

#[tauri::command]
pub fn set_ltc_addresses(state: State<'_, SharedPortfolio>, value: Vec<String>) {
    let mut portfolio = state.0.lock().unwrap();
    portfolio.config.ltc_addresses = Some(value);
}
