use crate::portfolio::{Portfolio, SharedPortfolio};
use anyhow::Result;
use tauri::State;

#[tauri::command]
pub async fn load_portfolio(
    state: State<'_, SharedPortfolio>,
    name: String,
    password: String,
) -> Result<(), ()> {
    let mut portfolio = state.0.lock().await;
    let unlocked_portfolio = Portfolio::open(&name, &password).unwrap();
    *portfolio = unlocked_portfolio;

    Ok(())
}
