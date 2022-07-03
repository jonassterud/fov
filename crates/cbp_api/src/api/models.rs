// Models for the Coinbase Exchange/Pro API
// https://docs.cloud.coinbase.com/exchange/reference

use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Account {
    pub id: Option<String>,
    pub currency: Option<String>,
    pub balance: Option<String>,
    pub available: Option<String>,
    pub hold: Option<String>,
    pub profile_id: Option<String>,
    pub trading_enabled: Option<bool>,
}

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
