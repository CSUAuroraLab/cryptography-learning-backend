pub struct Query;

#[async_graphql::Object]
impl Query {
    async fn hello(&self) -> String {
        "hello world".to_string()
    }
}