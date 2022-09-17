#[derive(macros::Entity)]
#[table_name = "blog"]
pub struct BlogPost {
  pub id: String,
  pub src: String,
  pub url: String,
  pub color: String
}
