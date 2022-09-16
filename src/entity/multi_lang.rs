use async_graphql::{InputObject, SimpleObject};
use serde::Serialize;


#[derive(SimpleObject, InputObject, Serialize, Clone)]
#[graphql(input_name = "MultiLangInput")] 
pub struct MultiLang {
  pub en: String,
  pub ru: String
}