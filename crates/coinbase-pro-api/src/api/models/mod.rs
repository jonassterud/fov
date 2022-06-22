#![allow(non_snake_case)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Account {
    pub id: String,
    pub currency: String,
    pub balance: String,
    pub available: String,
    pub hold: String,
    pub profile_id: String,
    pub trading_enabled: bool,
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
