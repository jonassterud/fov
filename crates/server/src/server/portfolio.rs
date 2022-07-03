use anyhow::Result;
use shared::{Asset, Config};

/// Portfolio to hold information on the different assets
pub struct Portfolio {
    /// Coinbase Pro assets
    pub cbp_assets: Vec<Asset>,
    /// Nordnet assets
    pub nn_assets: Vec<Asset>,
    /// SpareBank 1 assets
    pub sb1_assets: Vec<Asset>,
    /// Coinbase Pro API
    pub cbp_api: Option<cbp_api::API>,
    /// Nordnet API
    pub nn_api: Option<nn_api::API>,
    /// SpareBank 1 API
    pub sb1_api: Option<sb1_api::API>,
}

impl Portfolio {
    /// Creates a new portfolio
    ///
    /// # Arguments
    ///
    /// * `config` - Reference to a `Config`, used to connect to the different APIs
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

    /// Update Coinbase Pro assets
    pub async fn update_cbp_assets(&mut self) {
        todo!();
    }

    /// Update Nordnet assets
    pub async fn update_nn_assets(&mut self) {
        todo!();
    }

    /// Update SpareBank 1 assets
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

    /// Transform and add Coinbase Pro assets to portfolio
    ///
    /// # Arguments
    ///
    /// * `assets` - A vector containing elements that implement the `Into<Asset>` trait
    fn add_to_cbp_assets(&mut self, assets: Vec<impl Into<Asset> + Clone>) {
        self.cbp_assets
            .append(&mut assets.iter().map(|x| x.clone().into()).collect());
    }

    /// Transform and add Nordnet assets to portfolio
    ///
    /// # Arguments
    ///
    /// * `assets` - A vector containing elements that implement the `Into<Asset>` trait
    fn add_to_nn_assets(&mut self, assets: Vec<impl Into<Asset> + Clone>) {
        self.nn_assets
            .append(&mut assets.iter().map(|x| x.clone().into()).collect());
    }

    /// Transform and add SpareBank 1 assets to portfolio
    ///
    /// # Arguments
    ///
    /// * `assets` - A vector containing elements that implement the `Into<Asset>` trait
    fn add_to_sb1_assets(&mut self, assets: Vec<impl Into<Asset> + Clone>) {
        self.sb1_assets
            .append(&mut assets.iter().map(|x| x.clone().into()).collect());
    }
}
