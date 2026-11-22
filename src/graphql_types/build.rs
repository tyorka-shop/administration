use async_graphql::{ComplexObject, Context, Result, SimpleObject, ID};

use crate::builder::{self, BuildState, Builder};

use super::BuildStatus;

#[derive(Debug, SimpleObject)]
#[graphql(complex)]
pub struct Build {
    pub id: ID,
    pub status: BuildStatus,
    pub date: String,
    #[graphql(skip)]
    astro_id: u64,
}

impl From<builder::Build> for Build {
    fn from(build: builder::Build) -> Self {
        Self {
            id: build.id.to_string().into(),
            status: match build.state {
                BuildState::Running | BuildState::Stopping => BuildStatus::Pending,
                BuildState::Succeeded => BuildStatus::Done,
                BuildState::Failed | BuildState::Cancelled => BuildStatus::Failure,
            },
            date: build.started_at.naive_utc().to_string(),
            astro_id: build.id,
        }
    }
}

#[ComplexObject]
impl Build {
    async fn log(&self, ctx: &Context<'_>) -> Result<String> {
        let logs = ctx.data::<Builder>()?.logs(self.astro_id).await?;
        let clean_log = strip_ansi_escapes::strip(logs)?;
        Ok(String::from_utf8_lossy(&clean_log).into_owned())
    }
}
