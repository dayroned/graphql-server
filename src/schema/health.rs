use async_graphql::Object;

#[derive(Default)]
pub struct HealthQuery;

#[Object]
impl HealthQuery {
    /// Returns `true` to signal the GraphQL server is working
    async fn health(&self) -> bool {
        true
    }
}
