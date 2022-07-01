use anyhow::{anyhow, Result};
use serde::Serialize;
use shared::Asset;

#[derive(Debug, Serialize, Clone)]
pub struct Portfolio {
    pub cbp_assets: Vec<Asset>,
    pub nn_assets: Vec<Asset>,
    pub sb1_assets: Vec<Asset>,
}

impl Portfolio {
    pub fn new() -> Portfolio {
        Portfolio {
            cbp_assets: vec![],
            nn_assets: vec![],
            sb1_assets: vec![],
        }
    }

    pub fn add_cbp_assets(&mut self, assets: Vec<impl Into<Asset> + Clone>) {
        self.cbp_assets
            .append(&mut assets.iter().map(|x| x.clone().into()).collect());
    }

    pub fn add_nn_assets(&mut self, assets: Vec<impl Into<Asset> + Clone>) {
        self.nn_assets
            .append(&mut assets.iter().map(|x| x.clone().into()).collect());
    }

    pub fn add_sb1_assets(&mut self, assets: Vec<impl Into<Asset> + Clone>) {
        self.cbp_assets
            .append(&mut assets.iter().map(|x| x.clone().into()).collect());
    }
}
