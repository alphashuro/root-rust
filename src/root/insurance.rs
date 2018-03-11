extern crate reqwest;
extern crate serde;

use root::{RootClient, RootResult, Request};

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

impl RootClient {
    pub fn gadget_models(&self) -> RootResult<Vec<GadgetModel>> {
        self.request(
            Request::Get("insurance/modules/root_gadgets/models")
        )
    }

    pub fn gadget_quotes(&self, model: &str) -> RootResult<Vec<Quote>> {
        self.request(
            Request::Post("insurance/quotes", json!({
                "type": "root_gadgets",
                "model_name": model
            })
        ))
    }
}

