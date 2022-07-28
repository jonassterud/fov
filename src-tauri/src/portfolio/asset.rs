use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
//#[derive(Debug)]
pub struct Asset {
    pub tag: String,
    pub name: String,
    pub description: String,
    pub balance: f64,
    pub currency: String,
    pub value: i64,
}
