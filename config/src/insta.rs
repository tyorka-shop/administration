use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstaConfig {
    pub instagram_id: String,
    pub access_token: String,
}
