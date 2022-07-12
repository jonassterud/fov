use crate::portfolio::SharedPortfolio;
use tauri::State;

#[tauri::command]
pub fn get_assets(state: State<'_, SharedPortfolio>) -> String {
    let portfolio = state.0.lock().unwrap();
    serde_json::to_string(&portfolio.assets).unwrap()    
}