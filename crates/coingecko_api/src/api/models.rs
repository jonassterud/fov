// Models for the CoinGecko API
// https://www.coingecko.com/en/api/documentation

use serde::Deserialize;
use std::collections::HashMap;

// TODO: Probably a better way to do this

#[derive(Deserialize, Clone, Debug)]
pub struct SinglePriceNOK {
    pub nok: Option<f64>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SimplePrice {
    #[serde(flatten)]
    pub prices: Option<HashMap<String, SinglePriceNOK>>,
}
