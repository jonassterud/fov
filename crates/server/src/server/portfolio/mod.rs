mod assets;

use shared::{Asset, Config};

/// Portfolio to hold information on the different assets
pub struct Portfolio {
    /// Coinbase Pro assets
    pub cbp_assets: Vec<Asset>,
    /// Nordnet assets
    pub nn_assets: Vec<Asset>,
    /// SpareBank 1 assets
    pub sb1_assets: Vec<Asset>,
    /// Crypto assets
    pub crypto_assets: Vec<Asset>,
    /// Coinbase Pro API
    pub coinbasepro_api: Option<coinbasepro_api::API>,
    /// Nordnet API
    pub nordnet_api: Option<nordnet_api::API>,
    /// SpareBank 1 API
    pub sparebank1_api: Option<sparebank1_api::API>,
    /// NOWNodes API
    pub nownodes_api: Option<nownodes_api::API>,
}

impl Portfolio {
    /// Creates a new portfolio
    ///
    /// # Arguments
    ///
    /// * `config` - Reference to a `Config`, used to connect to the different APIs
    pub fn new(config: &Config) -> Portfolio {
        Portfolio {
            cbp_assets: vec![],
            nn_assets: vec![],
            sb1_assets: vec![],
            crypto_assets: vec![],
            nordnet_api: None, // for now
            coinbasepro_api: Some(coinbasepro_api::API::new(&config.cbp_key, &config.cbp_secret, &config.cbp_passphrase)),
            sparebank1_api: Some(sparebank1_api::API::new(&config.sb1_access_token)),
            nownodes_api: Some(nownodes_api::API::new(&config.nwn_key, &config.btc_xpub, &config.ltc_xpub)),
        }
    }
}
