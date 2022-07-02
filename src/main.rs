use server::Server;
use shared::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // TODO: Move this into a seperate thread
    // Open config and start server
    let config = Config::from_file("/home/jonassterud/Documents/fov/src/secret.toml")?;
    Server::new(config).start().await?;

    // Start web app
    // ...

    Ok(())
}
