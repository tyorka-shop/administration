use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct User {
  pub email: String,
}