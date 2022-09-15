use random::make_secret_key;
use serde::{Deserialize, Serialize};

mod random;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub port: String,
    pub secret: String,
    pub database_uri: String,
    pub cors_allowed_origins: Vec<String>,
    pub images_folder: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: "3000".into(),
            secret: make_secret_key(),
            cors_allowed_origins: vec!["http://localhost:3000".into()],
            database_uri: "sqlite:./store/db.sqlite".into(),
            images_folder: "./store/images".into(),
        }
    }
}

pub fn load(name: &str) -> Config {
    confy::load::<Config>(name).unwrap()
}
