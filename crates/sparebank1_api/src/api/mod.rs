mod models;
mod paths;

/// API struct to connect to SpareBank 1 API
pub struct API {
    client: reqwest::Client,
    access_token: String,
}

impl API {
    /// Creates a new `API` struct
    ///
    /// # Arguments
    ///
    /// * `access_token` - OAuth access token
    pub fn new(access_token: &str) -> API {
        API {
            client: reqwest::Client::new(),
            access_token: access_token.into(),
        }
    }
}
