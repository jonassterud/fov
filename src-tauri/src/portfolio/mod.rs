mod asset;
mod config;

use anyhow::Result;
use pwbox::{sodium::Sodium, ErasedPwBox, Eraser, Suite};
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::async_runtime::Mutex;

pub use asset::Asset;
pub use config::Config;

//#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct Portfolio {
    pub name: Option<String>,
    pub assets: Vec<Asset>,
    pub config: Config,
}

pub struct SharedPortfolio(pub Mutex<Portfolio>);

impl Portfolio {
    pub fn get_save_path() -> Result<PathBuf> {
        let mut save_path = dirs::config_dir().unwrap();
        save_path.push("fov");

        if !save_path.is_dir() {
            std::fs::create_dir(&save_path)?;
        }

        Ok(save_path)
    }

    /// Create an empty `Portfolio`
    pub fn new() -> Portfolio {
        Portfolio {
            name: None,
            assets: vec![],
            config: Config::new(),
        }
    }

    /// Open portfolio with the given name
    pub fn open(name: &str, password: &str) -> Result<Portfolio> {
        let mut path = Portfolio::get_save_path()?;
        path.push(format!("{}.toml", name));
        let content = std::fs::read(path)?;
        let pwbox = toml::from_slice(&content)?;
        let pwbox = Eraser::new().add_suite::<Sodium>().restore(&pwbox)?;

        let decrypted = pwbox.open(password)?;
        let portfolio = toml::from_slice::<Portfolio>(&decrypted)?;

        Ok(portfolio)
    }

    /// Save portfolio
    pub fn save(&self, password: &str) -> Result<()> {
        let mut path = Portfolio::get_save_path()?;
        path.push(format!("{}.toml", &self.name.as_ref().unwrap()));
        let pwbox = Sodium::build_box(&mut thread_rng()).seal(password, toml::to_string(self)?)?;
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
