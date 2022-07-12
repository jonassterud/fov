use crate::portfolio::SharedPortfolio;
use anyhow::Result;
use tauri::State;

#[tauri::command]
pub async fn get_assets(state: State<'_, SharedPortfolio>) -> Result<String, ()> {
    let portfolio = state.0.lock().await;
    let json = serde_json::to_string(&portfolio.assets).unwrap();

    Ok(json)
}
