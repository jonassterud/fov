mod portfolio;

use anyhow::Result;
use portfolio::Portfolio;
use shared::Config;
use warp::Filter;

/// The server struct
pub struct Server {
    /// Portfolio connected to this server
    portfolio: Portfolio,
}

impl Server {
    /// Creates a new server
    ///
    /// # Arguments
    ///
    /// * `config` - A `Config` to use for this server
    pub fn new(config: Config) -> Server {
        Server {
            portfolio: Portfolio::new(&config),
        }
    }

    /// Start the server
    pub async fn start(mut self) -> Result<()> {
        // Update SpareBank 1 assets
        self.portfolio.add_sparebank1_assets().await?;

        // Update Coinbase Pro assets
        self.portfolio.add_coinbasepro_assets().await?;

        // Update NOWNodes assets
        self.portfolio.add_nownodes_assets().await?;

        // Create API paths
        let assets = warp::path!("assets").map(move || serde_json::to_string(&self.portfolio.assets).unwrap());

        // Create website paths
        let html = warp::path!().and(warp::fs::file("./src/www/index.html"));
        let css = warp::path!("style.css").and(warp::fs::file("./src/www/style.css"));
        let js = warp::path!("script.js").and(warp::fs::file("./src/www/script.js"));

        // Serve paths and start server
        println!("Server running: http://127.0.0.1:3030");
        let routes = warp::get().and(assets).or(html).or(css).or(js);
        warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

        Ok(())
    }
}
