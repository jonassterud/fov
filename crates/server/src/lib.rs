mod portfolio;

use anyhow::{anyhow, Result};
pub use portfolio::Portfolio;
use warp::Filter;

pub async fn start(portfolio: Portfolio) -> Result<()> {
    let coinbase_pro_accounts_path = warp::path("coinbase_pro_accounts")
        .map(move || serde_json::to_string(&portfolio.coinbase_pro_accounts).unwrap());

    let sparebank1_accounts_path = warp::path("sparebank1_accounts")
        .map(move || serde_json::to_string(&portfolio.sparebank1_accounts).unwrap());

    let routes = warp::get().and(coinbase_pro_accounts_path.or(sparebank1_accounts_path));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
