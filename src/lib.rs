extern crate reqwest;

use reqwest::{Method};
#[macro_use] extern crate serde_derive;

struct RootClient {
    client: reqwest::Client,
    api_key: &'static str,
    pub env: RootEnv
}

enum RootEnv {
    Sandbox,
    Production
}

#[derive(Deserialize)]
struct GadgetModel {
    make: String,
    name: String,
    value: i32
}

impl RootClient {
    fn new(api_key: &'static str, env: RootEnv) -> Self {
        RootClient { 
            api_key,
            env,
            client: reqwest::Client::new()
        }
    }

    fn url(&self, path: &str) -> String {
        let env = match self.env {
            RootEnv::Sandbox => "sandbox",
            RootEnv::Production => "api"
        };

        format!("https://{}.root.co.za/v1/{}", env, path)
    }

    fn gadget_models(&self) -> reqwest::Result<Vec<GadgetModel>> {
        let url = self.url("insurance/modules/root_gadgets/models");

        let models: Vec<GadgetModel> = self.client.get(&url)
            .send()?
            .json()?;

        Ok(models)  
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor() {
        let root = RootClient::new("api key", RootEnv::Sandbox);

        assert_eq!(root.api_key, "api key")
    }
    
    #[test]
    fn url_sandbox() {
        let root = RootClient::new("api key", RootEnv::Sandbox);

        let actual = root.url("path/endpoint");
        let expected = "https://sandbox.root.co.za/v1/path/endpoint";

        assert_eq!(actual, expected)
    }

    #[test]
    fn url_production() {
        let root = RootClient::new("api key", RootEnv::Production);

        let actual = root.url("path/endpoint");
        let expected = "https://api.root.co.za/v1/path/endpoint";

        assert_eq!(actual, expected);
    }
}
