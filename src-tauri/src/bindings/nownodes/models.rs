use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BtcUtxo {
    pub txid: String,
    pub vout: u32,
    pub value: String,
    pub confirmations: u32,
    pub script_pub_key: Option<String>, // missing?
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LtcUtxo {
    pub txid: String,
    pub vout: u32,
    pub value: String,
    pub height: u64,
    pub confirmations: u32,
    pub coinbase: Option<bool>, // missing?
    pub script_pub_key: Option<String>, // missing?
}
