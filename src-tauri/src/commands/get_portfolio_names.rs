use crate::portfolio::Portfolio;
use anyhow::Result;

#[tauri::command]
pub async fn get_portfolio_names() -> Result<String, ()> {
    let path = Portfolio::get_save_path().unwrap();
    let mut names = vec![];

    for file in std::fs::read_dir(path).unwrap() {
        let file_name = file.unwrap().file_name();
        let file_name = file_name.to_string_lossy();
        let file_name = file_name.split(".toml").next().unwrap();

        names.push(String::from(file_name));
    }

    let json = serde_json::to_string(&names).unwrap();

    Ok(json)
}
