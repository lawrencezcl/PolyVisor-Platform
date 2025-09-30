// GraphQL相关模块占位符
use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};
use crate::AppState;

pub type QueryRoot = Query;
pub type MutationRoot = EmptyMutation;
pub type SubscriptionRoot = EmptySubscription;

pub struct Query;

#[Object]
impl Query {
    async fn hello(&self) -> &str {
        "Hello from GraphQL!"
    }
}

/// 创建GraphQL Schema
pub async fn create_graphql_schema(
    _app_state: AppState,
) -> Schema<QueryRoot, MutationRoot, SubscriptionRoot> {
    Schema::build(Query, EmptyMutation, EmptySubscription)
        .finish()
}

/// GraphQL Playground处理器
pub async fn graphql_playground() -> axum::response::Html<&'static str> {
    axum::response::Html(async_graphql::http::playground_source(
        async_graphql::http::GraphQLPlaygroundConfig::new("/graphql"),
    ))
}

/// GraphQL处理器
pub async fn graphql_handler(
    axum::extract::Extension(schema): axum::extract::Extension<Schema<QueryRoot, MutationRoot, SubscriptionRoot>>,
    req: async_graphql_axum::GraphQLRequest,
) -> async_graphql_axum::GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}