use serde::Serialize;

#[derive(Serialize)]
//#[derive(Debug)]
pub struct Asset {
    pub name: String,
    pub description: String,
    pub balance: f64,
    pub currency: String,
    pub value: i64,
}
