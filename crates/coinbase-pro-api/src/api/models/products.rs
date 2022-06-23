use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ProductTicker {
    pub ask: String,
    pub bid: String,
    pub volume: String,
    pub trade_id: i32,
    pub price: String,
    pub size: String,
    pub time: String,
}
