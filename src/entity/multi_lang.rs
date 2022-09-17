use async_graphql::{InputObject, SimpleObject};
use serde::Serialize;


#[derive(SimpleObject, InputObject, Serialize, Clone)]
#[graphql(input_name = "MultiLangInput")] 
pub struct MultiLang {
  pub en: String,
  pub ru: String
}

impl MultiLang {
  pub fn to_text(&self) -> Self {
    Self { 
      en: markdown_to_text::convert(&self.en),
      ru: markdown_to_text::convert(&self.ru)
    }
  }

  pub fn to_html(&self) -> Self {
    Self { 
      en: markdown::to_html(&self.en),
      ru: markdown::to_html(&self.ru)
    }
  }
}