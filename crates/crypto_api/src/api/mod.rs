use anyhow::Result;

mod models;
mod paths;

/// API struct
pub struct API {
    client: reqwest::Client,
}

impl API {
    /// Creates a new `API` struct
    pub fn new() -> API {
        API {
            client: reqwest::Client::new(),
        }
    }
}