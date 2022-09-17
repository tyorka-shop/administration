use async_graphql::{ID, SimpleObject};

#[derive(SimpleObject)]
pub struct BlogPost {
  pub id: ID,
  pub src: String,
  pub url: String,
  pub color: String
}


#[derive(macros::Entity)]
#[table_name = "blog"]
pub struct Entity {
  pub id: String,
  pub src: String,
  pub url: String,
  pub color: String
}

impl From<&BlogPost> for Entity {
  fn from(post: &BlogPost) -> Self {
    Self {
      id: post.id.to_string(),
      src: post.src.clone(),
      url: post.url.clone(),
      color: post.color.clone()
    }
  }
}