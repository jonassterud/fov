mod asset;

use asset::Asset;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Portfolio {
    pub cbp_assets: Vec<Asset>,
    pub nn_assets: Vec<Asset>,
    pub sb1_assets: Vec<Asset>,
}
