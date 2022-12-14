mod mutations;
mod product_resolver;
mod build_reolver;
mod queries;
mod subscriptions;

use async_graphql::{EmptySubscription, SchemaBuilder};

pub use mutations::Mutations;
pub use queries::Queries;

pub type Schema = async_graphql::Schema<Queries, Mutations, EmptySubscription>;

pub fn build_schema() -> SchemaBuilder<Queries, Mutations, EmptySubscription> {
    async_graphql::Schema::build(Queries, Mutations, EmptySubscription)
}
