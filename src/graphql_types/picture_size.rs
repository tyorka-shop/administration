use serde::Serialize;

#[derive(Debug, Serialize, async_graphql::SimpleObject)]
pub struct PictureSize {
  pub width: i64,
  pub height: i64,
}