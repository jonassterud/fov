// Models for the Coinbase Exchange/Pro API
// https://docs.cloud.coinbase.com/exchange/reference

use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct CurrencyDetails {
    #[serde(rename = "type")]
    pub _type: Option<String>,
    pub symbol: Option<String>,
    pub network_confirmations: Option<i32>,
    pub sort_order: Option<i32>,
    pub crypto_address_link: Option<String>,
    pub crypto_transaction_link: Option<String>,
    pub push_payment_methods: Option<Vec<String>>,
    pub group_types: Option<Vec<String>>,
    pub display_name: Option<String>,
    pub processing_time_seconds: Option<f32>,
    pub min_withdrawal_amount: Option<f64>,
    pub max_withdrawal_amount: Option<f64>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CurrencyNetworks {
    pub id: Option<String>,
    pub name: Option<String>,
    pub status: Option<String>,
    pub contract_address: Option<String>,
    pub crypto_address_link: Option<String>,
    pub crypto_transaction_link: Option<String>,
    pub min_withdrawal_amount: Option<f64>,
    pub max_withdrawal_amount: Option<f64>,
    pub network_confirmations: Option<i32>,
    pub processing_time_seconds: Option<i32>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Currency {
    pub id: Option<String>,
    pub name: Option<String>,
    pub min_size: Option<String>,
    pub status: Option<String>,
    pub message: Option<String>,
    pub max_precision: Option<String>,
    pub convertible_to: Option<Vec<String>>,
    pub details: Option<CurrencyDetails>,
    pub default_network: Option<String>,
    pub supported_networks: Option<Vec<CurrencyNetworks>>,
}

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
