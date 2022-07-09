// Models for the NOWNodes API
// https://documenter.getpostman.com/view/13630829/TVmFkLwy

use serde::Deserialize;

pub mod btc {
    use super::*;

    #[derive(Deserialize, Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct UTXO {
        pub txid: Option<String>,
        pub vout: Option<u32>,
        pub value: Option<String>,
        pub confirmations: Option<u32>,
        pub script_pub_key: Option<String>,
    }
}

pub mod ltc {
    use super::*;

    #[derive(Deserialize, Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct UTXO {
        pub txid: Option<String>,
        pub vout: Option<u32>,
        pub value: Option<String>,
        pub height: Option<u64>,
        pub confirmations: Option<u32>,
        pub coinbase: Option<bool>,
        pub script_pub_key: Option<String>,
    }
}
