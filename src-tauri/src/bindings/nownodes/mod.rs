mod models;
mod paths;

use crate::portfolio::Asset;
use anyhow::Result;

pub struct NowNodes {}

impl NowNodes {
    /// Create a `NowNodes` struct
    pub fn new() -> NowNodes {
        NowNodes {}
    }

    /// Get assets
    pub async fn get_assets(
        &self,
        key: &str,
        btc_addresses: Vec<String>,
        ltc_addresses: Vec<String>,
    ) -> Result<Vec<Asset>> {
        let mut assets = vec![];

        for btc_address in btc_addresses {
            let mut btc_asset = Asset {
                tag: "NOWNodes".into(),
                name: "Bitcoin".into(),
                description: "".into(),
                balance: 0.0,
                currency: "BTC".into(),
                value: 0,
            };

            for utxo in self.btc_utxo(key, &btc_address).await? {
                btc_asset.balance += utxo.value.parse::<f64>()? / 100000000.0;
            }

            assets.push(btc_asset);
        }

        for ltc_address in ltc_addresses {
            let mut ltc_asset = Asset {
                tag: "NOWNodes".into(),
                name: "Litecoin".into(),
                description: "".into(),
                balance: 0.0,
                currency: "LTC".into(),
                value: 0,
            };

            for utxo in self.ltc_utxo(key, &ltc_address).await? {
                ltc_asset.balance += utxo.value.parse::<f64>()? / 100000000.0;
            }

            assets.push(ltc_asset);
        }

        // TODO: Calculate value

        Ok(assets)
    }
}
