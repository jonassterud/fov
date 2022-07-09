use std::path::PathBuf;

use dialoguer::Password;
use server::Server;
use shared::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Open config and start server
    let server_handle = tokio::spawn(async {
        let config_path = PathBuf::from("./src/config.toml");
        let secret_path = PathBuf::from("./src/secret.toml");

        let password = Password::new().with_prompt("Enter password").interact().unwrap();
        let config = Config::open(&config_path, &secret_path, &password).unwrap();

        Server::new(config).start().await.expect("Server failed");
    });

    server_handle.await?;

    Ok(())
}
