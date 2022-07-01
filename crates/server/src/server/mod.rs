mod portfolio;

use anyhow::Result;
use portfolio::Portfolio;
use shared::Config;
use warp::Filter;

pub struct Server {
    portfolio: Portfolio,
    config: Config,
}

impl Server {
    pub fn new(config: Config) -> Server {
        Server {
            portfolio: Portfolio::new(&config),
            config: config,
        }
    }

    pub async fn start(mut self) -> Result<()> {
        // Update SpareBank 1 assets
        self.portfolio.update_sb1_assets().await?;

        // Create paths
        let cbp_assets = warp::path!("cbp" / "assets").map(move || {
            serde_json::to_string(&self.portfolio.cbp_assets).expect("Failed serializing Asset")
        });

        let nn_assets = warp::path!("nn" / "assets").map(move || {
            serde_json::to_string(&self.portfolio.nn_assets).expect("Failed serializing Asset")
        });

        let sb1_assets = warp::path!("sb1" / "assets").map(move || {
            serde_json::to_string(&self.portfolio.sb1_assets).expect("Failed serializing Asset")
        });

        // Serve paths and start server
        println!("Server running: 127.0.0.1:3030");

        let routes = warp::get().and(cbp_assets.or(nn_assets).or(sb1_assets));
        warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

        Ok(())
    }
}
