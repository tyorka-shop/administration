use async_graphql::{futures_util::Stream, Context};
use sqlx::SqlitePool;
use std::time::Duration;

use crate::publication_status::{PublicationStatus, PublicationStatusTrait};

pub struct Subscription;

#[async_graphql::Subscription]
impl Subscription {
    async fn interval(&self, #[graphql(default = 1)] n: i32) -> impl Stream<Item = i32> {
        let mut value = 0;
        async_stream::stream! {
            loop {
                futures_timer::Delay::new(Duration::from_secs(1)).await;
                value += n;
                yield value;
            }
        }
    }

    async fn is_draft<'a>(&self, ctx: &Context<'a>) -> impl Stream<Item = bool> + 'a {
        let db = ctx.data::<SqlitePool>().unwrap();
        let publication_status = ctx.data::<PublicationStatus>().unwrap();
        let initial = PublicationStatus::is_draft(&db).await.unwrap();
        async_stream::stream! {
            yield initial;
            loop {
                let is_draft = publication_status.recv();
                yield is_draft;
            }
        }
    }
}
