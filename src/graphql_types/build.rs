use async_graphql::{SimpleObject, ID};


#[derive(Debug, SimpleObject)]
#[graphql(complex)]
pub struct Build {
  pub id: ID,
  pub status: String,
  pub date: String,
  #[graphql(skip)]
  pub log: String
}

impl From<entity::Build> for Build {
  fn from(build: entity::Build) -> Self {
    Self {
      id: build.id.into(),
      status: build.status,
      date: build.date,
      log: build.log
    }
  }
}