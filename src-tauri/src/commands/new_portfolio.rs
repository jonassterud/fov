use crate::portfolio::{SharedPortfolio, Portfolio};
use anyhow::Result;
use tauri::State;

#[tauri::command]
pub async fn new_portfolio(
    state: State<'_, SharedPortfolio>,
    name: String,
) -> Result<(), ()> {
    let mut portfolio = state.0.lock().await;
    *portfolio = Portfolio::new();
    portfolio.name = Some(name);

    Ok(())
}
