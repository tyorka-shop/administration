pub use insta::InstaConfig;
use random::make_secret_key;
use serde::{Deserialize, Serialize};

mod random;
mod insta;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub addr: String,
    pub secret: String,
    pub database_uri: String,
    pub cors_allowed_origins: Vec<String>,
    pub images_folder: String,
    pub public_site_folder: String,
    #[serde(default = "default_builder_url")]
    pub builder_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub builder_token: Option<String>,
    pub insta: Option<InstaConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            addr: "127.0.0.1:3000".into(),
            secret: make_secret_key(),
            cors_allowed_origins: vec!["http://localhost:3000".into()],
            database_uri: "sqlite:./store/db.sqlite".into(),
            images_folder: "./store/images".into(),
            public_site_folder: "/public".into(),
            builder_url: default_builder_url(),
            builder_token: None,
            insta: None,
        }
    }
}

fn default_builder_url() -> String {
    "http://127.0.0.1:3000".into()
}

pub fn load(name: &str) -> Config {
    let mut config = confy::load::<Config>(name).unwrap();
    if let Ok(url) = std::env::var("BUILDER_URL") {
        config.builder_url = url;
    }
    if let Ok(token) = std::env::var("BUILDER_TOKEN") {
        config.builder_token = Some(token);
    }
    config
}
