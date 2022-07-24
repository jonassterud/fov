use crate::portfolio::SharedPortfolio;
use anyhow::Result;
use tauri::State;

#[tauri::command]
pub async fn lock_portfolio(state: State<'_, SharedPortfolio>, name: String, password: String) -> Result<(), ()> {
    let portfolio = state.0.lock().await;
    portfolio.save(&name, &password).unwrap();

    Ok(())
}
