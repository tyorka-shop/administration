use async_graphql::Enum;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
pub enum BuildStatus {
    Pending,
    Done,
    Failure,
}

impl std::str::FromStr for BuildStatus {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
      match s {
          "PENDING" => Ok(BuildStatus::Pending),
          "DONE" => Ok(BuildStatus::Done),
          "FAILURE" => Ok(BuildStatus::Failure),
          "FAILED" => Ok(BuildStatus::Failure),
          _ => Err(format!("'{}' is not a valid value for BuildStatus", s)),
      }
  }
}