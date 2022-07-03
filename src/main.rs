use server::Server;
use shared::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Open config and start server
    let server_handle = tokio::spawn(async {
        let config = Config::from_file("/home/jonassterud/Documents/fov/src/secret.toml").expect("Failed opening config");
        Server::new(config).start().await.expect("Server failed");
    });

    // Start web app
    // ...

    server_handle.await?;

    Ok(())
}
