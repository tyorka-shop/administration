use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Post {
    pub id: String,
    pub caption: Option<String>,
    pub media_type: String,
    pub media_url: Option<String>,
    pub permalink: String,
    pub thumbnail_url: Option<String>,
    pub timestamp: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Response {
    pub data: Vec<Post>,
}
