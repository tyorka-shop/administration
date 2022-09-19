use async_graphql::{ComplexObject, Result};

use crate::graphql_types::Build;

#[ComplexObject]
impl Build {
    async fn log(&self) -> Result<String> {
        let clean_log = strip_ansi_escapes::strip(&self.log).unwrap();
        Ok(String::from_utf8_lossy(&clean_log).to_string())
    }
}
