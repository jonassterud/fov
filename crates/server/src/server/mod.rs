mod portfolio;

use anyhow::Result;
use portfolio::Portfolio;

#[derive(Debug)]
pub struct Server {}

impl Server {
    pub fn new() -> Server {
        Server {}
    }

    pub async fn start(self) -> Result<()> {
        todo!("Start server");
    }
}
