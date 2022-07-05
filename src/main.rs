use server::Server;
use shared::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // TODO
    // Open a CLI setup, where you paste in API keys,
    // they get encrypted with a password, etc, etc.

    // Open config and start server
    let server_handle = tokio::spawn(async {
        let path = "/home/jonassterud/Documents/fov/src/secret.toml";
        let mut config: Config;
        if std::path::Path::new(path).exists() {
            config = Config::from_file(path)
                .expect("Failed opening config");
        } else {
            config = Config::from_cli().expect("Failed creating config");
            std::fs::write(path, toml::to_string_pretty(&config).unwrap()).expect("Failed writing config");
        }

        Server::new(config).start().await.expect("Server failed");
    });

    server_handle.await?;
    
    Ok(())
}
