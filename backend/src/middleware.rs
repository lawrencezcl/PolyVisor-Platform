// 中间件模块占位符
use axum::{http::Request, middleware::Next, response::Response};
use tracing::info;

/// 请求日志中间件
pub async fn request_logging<B>(
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, axum::http::StatusCode> {
    let method = request.method().clone();
    let uri = request.uri().clone();
    
    info!("处理请求: {} {}", method, uri);
    
    let response = next.run(request).await;
    
    info!("请求完成: {} {} -> {}", method, uri, response.status());
    
    Ok(response)
}