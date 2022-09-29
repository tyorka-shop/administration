use insta::InstaConfig;
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
    pub insta: Option<InstaConfig>
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
            insta: None
        }
    }
}

pub fn load(name: &str) -> Config {
    confy::load::<Config>(name).unwrap()
}
