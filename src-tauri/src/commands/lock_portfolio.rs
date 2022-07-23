use crate::portfolio::SharedPortfolio;
use anyhow::Result;
use tauri::State;

#[tauri::command]
pub async fn lock_portfolio(state: State<'_, SharedPortfolio>, password: String) -> Result<(), ()> {
    let portfolio = state.0.lock().await;
    portfolio.save("portfolio.toml", &password).unwrap();

    Ok(())
}
