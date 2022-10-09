use std::time::Duration;

use async_graphql::futures_util::Stream;

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
}
