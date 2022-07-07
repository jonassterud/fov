use std::path::PathBuf;

use dialoguer::Password;
use server::Server;
use shared::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Open config and start server
    let server_handle = tokio::spawn(async {
        let path = PathBuf::from("/home/jonassterud/Documents/fov/src/secret.toml");
        let config: Config;

        if path.exists() {
            let password = Password::new().with_prompt("Password").interact().expect("Failed getting password");
            config = Config::from_file(path, &password).expect("Failed opening config");
        } else {
            config = Config::from_cli().expect("Failed creating config");
            let password = Password::new()
                .with_prompt("Enter new password")
                .interact()
                .expect("Failed getting password");
            config.save_to_file(path, &password).expect("Failed saving config");
        }

        Server::new(config).start().await.expect("Server failed");
    });

    server_handle.await?;

    Ok(())
}
