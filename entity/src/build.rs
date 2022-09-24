
#[derive(macros::Entity, Debug, Clone)]
#[table_name = "build"]
pub struct Build {
  pub id: String,
  pub status: String,
  pub date: String,
  pub log: String
}

impl Build {
  pub fn new() -> Self {
    Self {
      id: uuid::Uuid::new_v4().to_string(),
      status: "PENDING".to_string(),
      date: chrono::Utc::now().to_rfc3339(),
      log: "".to_string()
    }
  }
}