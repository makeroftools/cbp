use async_graphql::{{EmptySubscription, Schema}};
use std::sync::Arc;

pub use query::Query;
pub use mutation::Mutation;

mod query;
mod mutation;

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn create_schema(client: Arc<BinanceClient>) -> AppSchema {
    Schema::build(Query, Mutation, EmptySubscription)
        .data(client)
        .finish()
}
