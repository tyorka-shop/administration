use async_graphql::{SimpleObject, ID};

use super::BuildStatus;


#[derive(Debug, SimpleObject)]
#[graphql(complex)]
pub struct Build {
  pub id: ID,
  pub status: BuildStatus,
  pub date: String,
  #[graphql(skip)]
  pub log: String
}

impl From<entity::Build> for Build {
  fn from(build: entity::Build) -> Self {
    Self {
      id: build.id.into(),
      status: build.status.parse().unwrap(),
      date: build.created_at.to_string(),
      log: build.log
    }
  }
}