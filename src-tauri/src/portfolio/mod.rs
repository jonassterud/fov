mod asset;
mod config;

use std::sync::Mutex;

pub use asset::Asset;
pub use config::Config;

//#[derive(Debug)]
pub struct Portfolio {
    pub assets: Vec<Asset>,
    pub config: Config,
}

pub struct SharedPortfolio(pub Mutex<Portfolio>);

impl Portfolio {
    /// Create an empty `Portfolio`
    pub fn new() -> Portfolio {
        Portfolio {
            assets: vec![],
            config: Config::new(),
        }
    }
}

impl SharedPortfolio {
    /// Create an empty `SharedPortfolio`
    pub fn new() -> SharedPortfolio {
        SharedPortfolio(Mutex::new(Portfolio::new()))
    }
}
