#[derive(macros::Entity, Debug, Clone)]
#[table_name = "build"]
pub struct Build {
    pub id: String,
    pub status: String,
    pub log: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Build {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            status: "PENDING".to_string(),
            log: "".to_string(),
            created_at: chrono::NaiveDateTime::from_timestamp(0, 0),
            updated_at: chrono::NaiveDateTime::from_timestamp(0, 0),
        }
    }
}
