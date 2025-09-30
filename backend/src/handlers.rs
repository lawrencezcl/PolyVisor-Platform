// 处理器模块占位符
use axum::{extract::Extension, http::StatusCode, response::Json};
use serde_json::json;

use crate::AppState;

/// 通用错误处理器
pub async fn handle_error(
    _extension: Extension<AppState>,
    error: String,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(json!({
        "error": "Internal Server Error",
        "message": error,
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}