use anyhow::{anyhow, Result};
use serde::Serialize;
use shared::{Asset, Config};

pub struct Portfolio {
    pub cbp_assets: Vec<Asset>,
    pub nn_assets: Vec<Asset>,
    pub sb1_assets: Vec<Asset>,
    pub cbp_api: Option<cbp_api::API>,
    pub nn_api: Option<nn_api::API>,
    pub sb1_api: Option<sb1_api::API>,
}

impl Portfolio {
    pub fn new(config: &Config) -> Portfolio {
        Portfolio {
            cbp_assets: vec![],
            nn_assets: vec![],
            sb1_assets: vec![],
            cbp_api: None, // for now
            nn_api: None,  // for now
            sb1_api: Some(sb1_api::API::new(config.sb1_access_token.clone())),
        }
    }

    pub async fn update_cbp_assets(&mut self) {
        todo!();
    }

    pub async fn update_nn_assets(&mut self) {
        todo!();
    }

    pub async fn update_sb1_assets(&mut self) -> Result<()> {
        if let Some(api) = &self.sb1_api {
            let asset_accounts = api
                .accounts(true, true, true, true, true, true, true)
                .await?;

            if let Some(assets) = asset_accounts.accounts {
                self.add_to_sb1_assets(assets);
            }
        }

        Ok(())
    }

    fn add_to_cbp_assets(&mut self, assets: Vec<impl Into<Asset> + Clone>) {
        self.cbp_assets
            .append(&mut assets.iter().map(|x| x.clone().into()).collect());
    }

    fn add_to_nn_assets(&mut self, assets: Vec<impl Into<Asset> + Clone>) {
        self.nn_assets
            .append(&mut assets.iter().map(|x| x.clone().into()).collect());
    }

    fn add_to_sb1_assets(&mut self, assets: Vec<impl Into<Asset> + Clone>) {
        self.sb1_assets
            .append(&mut assets.iter().map(|x| x.clone().into()).collect());
    }
}
