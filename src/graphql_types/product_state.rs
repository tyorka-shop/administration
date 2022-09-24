use async_graphql::Enum;
use serde::Serialize;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Serialize)]
pub enum ProductState {
    Draft
}

impl std::str::FromStr for ProductState {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
      match s {
          "DRAFT" => Ok(ProductState::Draft),
          _ => Err(format!("'{}' is not a valid value for ProductState", s)),
      }
  }
}