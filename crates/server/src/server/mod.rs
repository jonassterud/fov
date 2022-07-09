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

        // Update Crypto assets
        self.portfolio.add_bitcoin_assets().await?;
        self.portfolio.add_ltc_assets().await?;

        // Create API paths
        let cbp_assets = warp::path!("coinbasepro" / "assets").map(move || serde_json::to_string(&self.portfolio.cbp_assets).unwrap());
        let nn_assets = warp::path!("nordnet" / "assets").map(move || serde_json::to_string(&self.portfolio.nn_assets).unwrap());
        let sb1_assets = warp::path!("sparebank1" / "assets").map(move || serde_json::to_string(&self.portfolio.sb1_assets).unwrap());
        let crypto_assets = warp::path!("nownodes" / "assets").map(move || serde_json::to_string(&self.portfolio.crypto_assets).unwrap());

        // Create website paths
        let html = warp::path!().and(warp::fs::file("/home/jonassterud/Documents/fov/src/www/index.html"));
        let css = warp::path!("style.css").and(warp::fs::file("/home/jonassterud/Documents/fov/src/www/style.css"));
        let js = warp::path!("script.js").and(warp::fs::file("/home/jonassterud/Documents/fov/src/www/script.js"));

        // Serve paths and start server
        println!("Server running: http://127.0.0.1:3030");
        let routes = warp::get().and(cbp_assets.or(nn_assets).or(sb1_assets).or(crypto_assets).or(html).or(css).or(js));
        warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

        Ok(())
    }
}
