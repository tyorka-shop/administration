mod mutations;
mod queries;
mod product_resolver;

use async_graphql::{EmptySubscription, SchemaBuilder};

pub use mutations::Mutations;
pub use queries::Queries;

pub type Schema = async_graphql::Schema<Queries, Mutations, EmptySubscription>;

pub fn build_schema() -> SchemaBuilder<Queries, Mutations, EmptySubscription> {
    async_graphql::Schema::build(Queries, Mutations, EmptySubscription)
}
