pub mod metrics;
pub mod health;
pub mod privacy;
pub mod proofs;
pub mod contributors;

use axum::{
    routing::{get, post},
    Router,
};

/// 创建API路由
pub fn create_api_routes() -> Router {
    Router::new()
        .nest("/v1", create_v1_routes())
}

/// 创建V1版本的API路由
fn create_v1_routes() -> Router {
    Router::new()
        // 网络指标相关路由
        .nest("/metrics", metrics::create_routes())
        // 网络健康度相关路由  
        .nest("/health", health::create_routes())
        // 隐私设置相关路由
        .nest("/privacy", privacy::create_routes())
        // 零知识证明相关路由
        .nest("/proofs", proofs::create_routes())
        // 贡献者相关路由
        .nest("/contributors", contributors::create_routes())
}"