extern crate reqwest;
#[macro_use] extern crate serde_derive;
extern crate serde;
#[macro_use] extern crate serde_json;

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

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Module {
    #[serde(rename = "root_gadgets")]
    Gadgets { make: String, model: String }
}

#[derive(Deserialize, Debug)]
pub struct Quote {
    quote_package_id: String,
    package_name: String,
    sum_assured: i32,
    base_premium: i32,
    suggested_premium: i32,
    module: Module,
    created_at: String
}

type RootResult<T> = reqwest::Result<T>;

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

    pub fn gadget_models(&self) -> RootResult<Vec<GadgetModel>> {
        let url = self.url("insurance/modules/root_gadgets/models");

        let models: Vec<GadgetModel> = self.client.request(reqwest::Method::Get, &url)
            .basic_auth::<&str, &str>(self.api_key, None)
            .send()?
            .json()?;

        Ok(models)  
    }

    pub fn gadget_quotes(&self, model: &str) -> RootResult<Vec<Quote>> {
        let url = self.url("insurance/quotes");

        let quotes: Vec<Quote> = self.client.request(reqwest::Method::Post, &url)
            .basic_auth::<&str, &str>(self.api_key, None)
            .json(&json!({
                "type": "root_gadgets",
                "model_name": model
            }))
            .send()?
            .json()?;

        Ok(quotes)
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
