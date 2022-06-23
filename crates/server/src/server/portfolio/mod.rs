mod asset;

use asset::Asset;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Portfolio {
    pub cbp_assets: Vec<Asset>,
    pub nn_assets: Vec<Asset>,
    pub sb1_assets: Vec<Asset>,
}

impl Portfolio {
    pub fn add_cbp_assets(assets: Vec<T>) -> Result<()> {
        Ok(())
    }

    pub fn add_nn_assets(assets: Vec<T>) -> Result<()> {
        Ok(())
    }

    pub fn add_sb1_assets(assets: Vec<T>) -> Result<()> {
        Ok(())
    }
}
