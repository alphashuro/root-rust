#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

pub mod root;

pub use root::{RootClient, RootEnv};

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
