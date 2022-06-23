use server::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // TODO: Move this into a seperate thread
    Server::new().start().await?;

    // TODO: Start web app?
    // ...

    Ok(())
}
