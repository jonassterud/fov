mod asset;
mod config;

use serde::{Deserialize, Serialize};
use tauri::async_runtime::Mutex;
use pwbox::{sodium::Sodium, ErasedPwBox, Eraser, Suite};
use rand::thread_rng;
use anyhow::Result;

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

    /// Open portfolio at the given path
    pub fn open(path: &str, password: &str) -> Result<Portfolio> {
        let content = std::fs::read(path)?;
        let pwbox = toml::from_slice(&content)?;
        let pwbox = Eraser::new().add_suite::<Sodium>().restore(&pwbox)?;

        let decrypted = pwbox.open(password)?;
        let portfolio = toml::from_slice::<Portfolio>(&decrypted)?;

        Ok(portfolio)
    }

    /// Save portfolio at the given path
    pub fn save(&self, path: &str, password: &str) -> Result<()> {
        let pwbox = Sodium::build_box(&mut thread_rng())
            .seal(password, toml::to_string(self)?)?;
        let mut eraser = Eraser::new();
        eraser.add_suite::<Sodium>();
        let erased: ErasedPwBox = eraser.erase(&pwbox)?;

        let content = toml::to_string_pretty(&erased)?;
        std::fs::write(path, content)?;
        
        Ok(())
    }
}

impl SharedPortfolio {
    /// Create an empty `SharedPortfolio`
    pub fn new() -> SharedPortfolio {
        SharedPortfolio(Mutex::new(Portfolio::new()))
    }
}
