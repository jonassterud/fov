mod models;
mod models_impl;
mod paths;

pub struct API {
    client: reqwest::Client,
    access_token: String,
}

impl API {
    pub fn new(access_token: String) -> API {
        API {
            client: reqwest::Client::new(),
            access_token: access_token,
        }
    }
}
