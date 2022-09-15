use async_graphql::{InputObject, SimpleObject};


#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "MultiLangInput")] 
pub struct MultiLang {
  pub en: String,
  pub ru: String
}