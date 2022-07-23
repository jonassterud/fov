use crate::portfolio::{Portfolio, SharedPortfolio};
use anyhow::Result;
use tauri::State;

#[tauri::command]
pub async fn unlock_portfolio(
    state: State<'_, SharedPortfolio>,
    password: String,
) -> Result<(), ()> {
    let mut portfolio = state.0.lock().await;
    let unlocked_portfolio = Portfolio::open("portfolio.toml", &password).unwrap();
    *portfolio = unlocked_portfolio;

    Ok(())
}
