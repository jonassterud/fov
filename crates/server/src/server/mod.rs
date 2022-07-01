mod portfolio;

use anyhow::Result;
use portfolio::Portfolio;
use warp::Filter;

#[derive(Debug)]
pub struct Server {
    portfolio: Portfolio,
}

impl Server {
    pub fn new() -> Server {
        Server {
            portfolio: Portfolio::new(),
        }
    }

    pub async fn start(self) -> Result<()> {
        let cbp_assets = warp::path!("cbp" / "assets").map(move || {
            serde_json::to_string(&self.portfolio.cbp_assets).expect("Failed serializing Asset")
        });

        let nn_assets = warp::path!("nn" / "assets").map(move || {
            serde_json::to_string(&self.portfolio.nn_assets).expect("Failed serializing Asset")
        });

        let sb1_assets = warp::path!("sb1" / "assets").map(move || {
            serde_json::to_string(&self.portfolio.sb1_assets).expect("Failed serializing Asset")
        });

        let routes = warp::get().and(cbp_assets.or(nn_assets).or(sb1_assets));

        warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

        Ok(())
    }
}
