use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Asset {
    pub name: String,
    pub description: Option<String>,
    pub balance: f64,
    pub currency: String,
}
