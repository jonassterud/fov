use crate::portfolio::{Portfolio, SharedPortfolio};
use anyhow::Result;
use pwbox::{sodium::Sodium, Eraser};
use tauri::State;

#[tauri::command]
pub async fn unlock_portfolio(
    state: State<'_, SharedPortfolio>,
    password: String,
) -> Result<(), ()> {
    let mut portfolio = state.0.lock().await;

    let content = std::fs::read("portfolio.toml").unwrap();
    let pwbox = toml::from_slice(&content).unwrap();
    let pwbox = Eraser::new().add_suite::<Sodium>().restore(&pwbox).unwrap();

    let decrypted = pwbox.open(password).unwrap();
    let decrypted_portfolio = toml::from_slice::<Portfolio>(&decrypted).unwrap();

    *portfolio = decrypted_portfolio;

    Ok(())
}
