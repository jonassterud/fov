//#[derive(Debug)]
pub struct Config {
    pub sparebank_1_access_token: Option<String>,
    pub coinbase_pro_key: Option<String>,
    pub coinbase_pro_secret: Option<String>,
    pub coinbase_pro_passphrase: Option<String>,
    pub nownodes_key: Option<String>,
    pub btc_addresses: Option<Vec<String>>,
    pub ltc_addresses: Option<Vec<String>>,
}

impl Config {
    /// Create an empty `Config`
    pub fn new() -> Config {
        Config {
            sparebank_1_access_token: None,
            coinbase_pro_key: None,
            coinbase_pro_secret: None,
            coinbase_pro_passphrase: None,
            nownodes_key: None,
            btc_addresses: None,
            ltc_addresses: None,
        }
    }
}
