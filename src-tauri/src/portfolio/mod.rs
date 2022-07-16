mod asset;
mod config;

use tauri::async_runtime::Mutex;
use serde::{Deserialize, Serialize};

pub use asset::Asset;
pub use config::Config;

//#[derive(Debug)]
#[derive(Deserialize, Serialize)]
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
