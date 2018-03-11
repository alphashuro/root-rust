extern crate reqwest;
#[macro_use] extern crate serde_derive;

pub struct RootClient {
    client: reqwest::Client,
    api_key: &'static str,
    pub env: RootEnv
}

pub enum RootEnv {
    Sandbox,
    Production
}

#[derive(Debug, Deserialize)]
pub struct GadgetModel {
    make: String,
    name: String,
    value: i32
}

impl RootClient {
    pub fn new(api_key: &'static str, env: RootEnv) -> Self {
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

    pub fn get_gadget_models(&self) -> reqwest::Result<Vec<GadgetModel>> {
        let url = self.url("insurance/modules/root_gadgets/models");

        let models: Vec<GadgetModel> = self.client.get(&url)
            .basic_auth::<&str, &str>(self.api_key, None)
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
