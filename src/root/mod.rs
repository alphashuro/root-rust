extern crate reqwest;
extern crate serde;
extern crate serde_json;

use root::insurance::Insurance;

pub mod insurance;

type Json = serde_json::Value;

pub struct RootClient {
    insurance: Insurance
}

pub enum RootEnv {
    Sandbox,
    Production
}

pub enum Request {
    Get(&'static str),
    Post(&'static str, Json)
}

pub struct RootApi {
    client: reqwest::Client,
    api_key: &'static str,
    pub env: RootEnv
}

pub type RootResult<T> = reqwest::Result<T>;

impl RootClient {
    pub fn new(api_key: &'static str, env: RootEnv) -> Self {
        RootClient {
            insurance: Insurance {
                api: RootApi {
                    api_key,
                    env,
                    client: reqwest::Client::new(),
                }
            }
        }
    }
}

impl RootApi {
    fn url(&self, path: &str) -> String {
        let env = match self.env {
            RootEnv::Sandbox => "sandbox",
            RootEnv::Production => "api"
        };

        format!("https://{}.root.co.za/v1/{}", env, path)
    }

    fn request<T: serde::de::DeserializeOwned>(&self, request: Request) -> RootResult<T> {
        let mut response = match request {
            Request::Get(path) => self.client
                .get(&self.url(path))
                .basic_auth::<&str, &str>(self.api_key, None)
                .send()?,
            Request::Post(path, body) => self.client
                .post(&self.url(path))
                .json(&body)
                .basic_auth::<&str, &str>(self.api_key, None)
                .send()?
        };

        Ok(response.json()?)
    }
}
