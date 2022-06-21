mod schema;

use anyhow::{anyhow, Result};

pub struct Nordnet {
    pub client: reqwest::Client,
}

impl Nordnet {
    pub async fn new(username: &str, password: &str) -> Result<Nordnet> {
        let client = reqwest::Client::new();

        let url = "https://www.nordnet.se/api/2";
        let resp = client.get(url).send().await?;

        if resp.status() == reqwest::StatusCode::OK {
            let schema = resp.json::<schema::Status>().await?;

            if schema.system_running {
                let nordnet_struct = Nordnet {
                    client: client,
                };

                nordnet_struct.login(username, password, schema.timestamp).await?;

                Ok(nordnet_struct)
            } else {
                Err(anyhow!(
                    "API temporarily stopped, with message: {}",
                    schema.message
                ))
            }
        } else {
            Err(anyhow!(
                "Failed connecting to API, with code: {:?}",
                resp.status()
            ))
        }
    }

    pub async fn login(&self, username: &str, password: &str, timestamp: i64) -> Result<()> {
        // Login using "Create session (login)", found here: https://www.nordnet.se/externalapi/docs/api

        let url = "https://www.nordnet.se/api/2/login";

        let mut auth = format!("{}:{}:{}", base64::encode(username), base64::encode(password), base64::encode(timestamp.to_string()));
        todo!("encode");
        todo!("base64 again");

        let mut form = std::collections::HashMap::new();
        form.insert("auth", auth);
        form.insert("service", "SERVICE_NAME_HERE".to_string());

        let resp = self.client.post(url).form(&form).send().await?;

        Ok(())
    }
}

#[tokio::test]
async fn connect() {
    Nordnet::new("username", "password").await.unwrap();
}
