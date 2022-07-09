mod models;
mod paths;

/// API struct to connect to Coinbase Pro API
pub struct API {
    client: reqwest::Client,
}

impl API {
    pub fn new() -> API {
        API {
            client: reqwest::Client::new(),
        }
    }
}
