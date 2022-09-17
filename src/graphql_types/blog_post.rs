use async_graphql::{ID, SimpleObject};

#[derive(SimpleObject)]
pub struct BlogPost {
  pub id: ID,
  pub src: String,
  pub url: String,
  pub color: String
}


impl From<&BlogPost> for entity::BlogPost {
  fn from(post: &BlogPost) -> Self {
    Self {
      id: post.id.to_string(),
      src: post.src.clone(),
      url: post.url.clone(),
      color: post.color.clone()
    }
  }
}

impl From<&entity::BlogPost> for BlogPost {
  fn from(post: &entity::BlogPost) -> Self {
    Self {
      id: ID::from(post.id.clone()),
      src: post.src.clone(),
      url: post.url.clone(),
      color: post.color.clone()
    }
  }
}