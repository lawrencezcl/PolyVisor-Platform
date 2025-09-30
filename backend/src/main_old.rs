mod api;
mod config;
mod database;
mod services;
mod handlers;
mod middleware;
mod websocket;
mod graphql;

use anyhow::Result;
use axum::{
    extract::Extension,
    http::{HeaderValue, Method},
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::{info, warn};

use crate::{
    api::create_api_routes,
    config::AppConfig,
    database::Database,
    graphql::create_graphql_schema,
    services::Services,
    websocket::websocket_handler,
};

/// PolyVisor后端服务应用状态
#[derive(Clone)]
pub struct AppState {
    /// 数据库连接池
    pub database: Arc<Database>,
    /// 业务服务集合
    pub services: Arc<Services>,
    /// 应用配置
    pub config: Arc<AppConfig>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("\u{d83d\u{de00 启动 PolyVisor 后端服务...");

    // 加载配置
    let config = Arc::new(AppConfig::from_env()?);
    info!("\u{2699\u{fe0f 配置加载完成");

    // 初始化数据库连接
    let database = Arc::new(Database::new(&config.database_url).await?);
    info!("\u{d83d\u{dcbe 数据库连接建立");

    // 运行数据库迁移
    database.migrate().await?;
    info!("\u{d83d\u{dd04 数据库迁移完成");

    // 初始化业务服务
    let services = Arc::new(Services::new(database.clone(), config.clone()).await?);
    info!("\u{d83d\u{dee0\u{fe0f 业务服务初始化完成");

    // 创建应用状态
    let app_state = AppState {
        database: database.clone(),
        services: services.clone(),
        config: config.clone(),
    };

    // 创建GraphQL Schema
    let graphql_schema = create_graphql_schema(app_state.clone()).await;

    // 创建路由
    let app = create_app_router(app_state, graphql_schema).await;

    // 启动服务器
    let listener = tokio::net::TcpListener::bind(&format!(
        "{}:{}",
        config.server.host, config.server.port
    ))
    .await?;

    info!(
        "\u{d83c\u{df0d 服务器启动成功: http://{}:{}",
        config.server.host, config.server.port
    );

    info!("\u{d83d\u{dee1\u{fe0f API文档: http://{}:{}/docs", config.server.host, config.server.port);
    info!("\u{d83d\u{dd0d GraphQL Playground: http://{}:{}/graphql", config.server.host, config.server.port);

    axum::serve(listener, app).await?;

    Ok(())
}

/// 创建应用路由
async fn create_app_router(
    app_state: AppState,
    graphql_schema: async_graphql::Schema<
        graphql::QueryRoot,
        graphql::MutationRoot,
        graphql::SubscriptionRoot,
    >,
) -> Router {
    // CORS配置
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(Any)
        .allow_credentials(true);

    // 创建主路由
    Router::new()
        // 健康检查
        .route("/health", get(health_check))
        // API路由
        .nest("/api", create_api_routes())
        // GraphQL路由
        .route(
            "/graphql",
            get(graphql::graphql_playground).post(graphql::graphql_handler),
        )
        // WebSocket路由
        .route("/ws", get(websocket_handler))
        // 静态文件服务（文档等）
        .route("/docs", get(serve_docs))
        // 中间件
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(cors)
                .layer(Extension(app_state))
                .layer(Extension(graphql_schema)),
        )
}

/// 健康检查端点
async fn health_check(Extension(app_state): Extension<AppState>) -> axum::Json<serde_json::Value> {
    let health_status = serde_json::json!({
        "status": "healthy",
        "service": "polyvisor-backend",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "database": {
            "connected": app_state.database.is_connected().await,
        },
        "services": {
            "zkproof": "active",
            "privacy": "active",
            "data_collection": "active",
        }
    });

    axum::Json(health_status)
}

/// 提供API文档
async fn serve_docs() -> axum::response::Html<&'static str> {
    axum::response::Html(include_str!("../docs/api.html"))
}"