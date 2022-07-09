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
    pub cbp_api: Option<cbp_api::API>,
    /// Nordnet API
    pub nn_api: Option<nn_api::API>,
    /// SpareBank 1 API
    pub sb1_api: Option<sb1_api::API>,
    /// Crypto API
    pub nwn_api: Option<nwn_api::API>,
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
            nn_api: None, // for now
            cbp_api: Some(cbp_api::API::new(
                &config.cbp_key,
                &config.cbp_secret,
                &config.cbp_passphrase,
            )),
            sb1_api: Some(sb1_api::API::new(&config.sb1_access_token)),
            nwn_api: Some(nwn_api::API::new(
                &config.nwn_key,
                &config.btc_xpub,
                &config.ltc_xpub,
            )),
        }
    }
}
