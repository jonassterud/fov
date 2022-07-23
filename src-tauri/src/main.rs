#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod bindings;
mod commands;
mod portfolio;

use commands::*;
use portfolio::SharedPortfolio;

fn main() {
    // TODO: The portfolio is used inside a `tauri::State<>`. Is this safe?
    let shared_portfolio = SharedPortfolio::new();

    // Create configuration directories if they dont exist
    let mut config_dir = dirs::config_dir().unwrap();
    config_dir.push("fov");

    if !config_dir.is_dir() {
        std::fs::create_dir(config_dir).unwrap();
    }

    // Start app
    tauri::Builder::default()
        .manage(shared_portfolio)
        .invoke_handler(tauri::generate_handler!(
            set_sparebank_1_access_token,
            set_coinbase_pro_key,
            set_coinbase_pro_secret,
            set_coinbase_pro_passphrase,
            set_nownodes_key,
            set_btc_addresses,
            set_ltc_addresses,
            get_assets,
            update_assets,
            lock_portfolio,
            unlock_portfolio,
        ))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
