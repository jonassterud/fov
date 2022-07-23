use crate::portfolio::SharedPortfolio;
use anyhow::Result;
use pwbox::{sodium::Sodium, ErasedPwBox, Eraser, Suite};
use rand::thread_rng;
use tauri::State;

#[tauri::command]
pub async fn lock_portfolio(state: State<'_, SharedPortfolio>, password: String) -> Result<(), ()> {
    let portfolio = state.0.lock().await;

    let pwbox = Sodium::build_box(&mut thread_rng())
        .seal(password, toml::to_string(&*portfolio).unwrap())
        .unwrap();
    let mut eraser = Eraser::new();
    eraser.add_suite::<Sodium>();
    let erased: ErasedPwBox = eraser.erase(&pwbox).unwrap();

    let content = toml::to_string_pretty(&erased).unwrap();
    std::fs::write("portfolio.toml", content).unwrap();

    Ok(())
}
