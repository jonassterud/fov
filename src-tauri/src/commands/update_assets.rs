use crate::portfolio::SharedPortfolio;
use tauri::State;

#[tauri::command]
pub fn update_assets(state: State<'_, SharedPortfolio>) {
    let portfolio = state.0.lock().unwrap();
    // ...
}