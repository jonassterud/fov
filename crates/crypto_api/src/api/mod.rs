mod models;
mod paths;

/// API struct
pub struct API {
    client: reqwest::Client,
    key: String,
    btc_xpub: String,
    ltc_xpub: String,
}

impl API {
    /// Creates a new `API` struct
    ///
    /// # Arguments
    ///
    /// * `key` - API key
    pub fn new(key: String, btc_xpub: String, ltc_xpub: String) -> API {
        API {
            client: reqwest::Client::new(),
            key: key,
            btc_xpub: btc_xpub,
            ltc_xpub: ltc_xpub,
        }
    }
}