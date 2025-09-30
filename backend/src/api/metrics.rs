use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::AppState;

/// 创建网络指标相关路由
pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(get_metrics).post(submit_metric))
        .route("/:metric_type", get(get_metric_by_type))
        .route("/:metric_type/history", get(get_metric_history))
        .route("/batch", post(submit_metrics_batch))
}

/// 网络指标查询参数
#[derive(Debug, Deserialize)]
pub struct MetricsQuery {
    /// 指标类型过滤
    metric_type: Option<String>,
    /// 隐私级别过滤
    privacy_level: Option<String>,
    /// 开始时间（Unix时间戳）
    from: Option<i64>,
    /// 结束时间（Unix时间戳）
    to: Option<i64>,
    /// 最小质量评分
    min_quality: Option<u8>,
    /// 限制返回数量
    limit: Option<u32>,
    /// 偏移量
    offset: Option<u32>,
}

/// 网络指标响应
#[derive(Debug, Serialize)]
pub struct MetricResponse {
    pub id: Uuid,
    pub metric_type: String,
    pub value: f64,
    pub quality_score: u8,
    pub privacy_level: String,
    pub proof_id: Option<String>,
    pub data_sources: Option<serde_json::Value>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// 指标提交请求
#[derive(Debug, Deserialize)]
pub struct SubmitMetricRequest {
    pub metric_type: String,
    pub value: f64,
    pub quality_score: u8,
    pub privacy_level: String,
    pub proof: Option<ProofData>,
    pub data_sources: Option<serde_json::Value>,
}

/// 零知识证明数据
#[derive(Debug, Deserialize)]
pub struct ProofData {
    pub proof_value: String,
    pub public_inputs: Vec<u128>,
    pub verification_key: String,
    pub circuit_id: u32,
}

/// 批量提交请求
#[derive(Debug, Deserialize)]
pub struct BatchSubmitRequest {
    pub metrics: Vec<SubmitMetricRequest>,
}

/// 获取网络指标列表
pub async fn get_metrics(
    Query(params): Query<MetricsQuery>,
    Extension(app_state): Extension<AppState>,
) -> Result<Json<Vec<MetricResponse>>, StatusCode> {
    let limit = params.limit.unwrap_or(50).min(1000) as i64;
    let offset = params.offset.unwrap_or(0) as i64;
    
    // 构建查询条件
    let mut query = "SELECT id, metric_type, value, quality_score, privacy_level, proof_id, data_sources, created_at FROM network_metrics WHERE 1=1".to_string();
    let mut conditions = Vec::new();
    
    if let Some(metric_type) = &params.metric_type {
        conditions.push(format!("metric_type = '{}'", metric_type));
    }
    
    if let Some(privacy_level) = &params.privacy_level {
        conditions.push(format!("privacy_level = '{}'", privacy_level));
    }
    
    if let Some(min_quality) = params.min_quality {
        conditions.push(format!("quality_score >= {}", min_quality));
    }
    
    if let Some(from) = params.from {
        conditions.push(format!("created_at >= to_timestamp({})", from));
    }
    
    if let Some(to) = params.to {
        conditions.push(format!("created_at <= to_timestamp({})", to));
    }
    
    if !conditions.is_empty() {
        query.push_str(&format!(" AND {}", conditions.join(" AND ")));
    }
    
    query.push_str(&format!(" ORDER BY created_at DESC LIMIT {} OFFSET {}", limit, offset));
    
    match sqlx::query_as::<_, MetricRow>(&query)
        .fetch_all(app_state.database.pool())
        .await
    {
        Ok(rows) => {
            let metrics: Vec<MetricResponse> = rows
                .into_iter()
                .map(|row| MetricResponse {
                    id: row.id,
                    metric_type: row.metric_type,
                    value: row.value.to_f64().unwrap_or(0.0),
                    quality_score: row.quality_score as u8,
                    privacy_level: row.privacy_level,
                    proof_id: row.proof_id,
                    data_sources: row.data_sources,
                    created_at: row.created_at,
                })
                .collect();
            
            Ok(Json(metrics))
        }
        Err(e) => {
            tracing::error!("获取指标数据失败: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// 根据类型获取特定指标
pub async fn get_metric_by_type(
    Path(metric_type): Path<String>,
    Extension(app_state): Extension<AppState>,
) -> Result<Json<Option<MetricResponse>>, StatusCode> {
    match sqlx::query_as::<_, MetricRow>(
        "SELECT id, metric_type, value, quality_score, privacy_level, proof_id, data_sources, created_at 
         FROM network_metrics 
         WHERE metric_type = $1 
         ORDER BY created_at DESC 
         LIMIT 1"
    )
    .bind(&metric_type)
    .fetch_optional(app_state.database.pool())
    .await
    {
        Ok(Some(row)) => {
            let metric = MetricResponse {
                id: row.id,
                metric_type: row.metric_type,
                value: row.value.to_f64().unwrap_or(0.0),
                quality_score: row.quality_score as u8,
                privacy_level: row.privacy_level,
                proof_id: row.proof_id,
                data_sources: row.data_sources,
                created_at: row.created_at,
            };
            Ok(Json(Some(metric)))
        }
        Ok(None) => Ok(Json(None)),
        Err(e) => {
            tracing::error!("获取指标数据失败: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// 获取指标历史数据
pub async fn get_metric_history(
    Path(metric_type): Path<String>,
    Query(params): Query<MetricsQuery>,
    Extension(app_state): Extension<AppState>,
) -> Result<Json<Vec<MetricResponse>>, StatusCode> {
    let limit = params.limit.unwrap_or(100).min(1000) as i64;
    
    let mut query = "SELECT id, metric_type, value, quality_score, privacy_level, proof_id, data_sources, created_at FROM network_metrics WHERE metric_type = $1".to_string();
    
    if let Some(from) = params.from {
        query.push_str(&format!(" AND created_at >= to_timestamp({})", from));
    }
    
    if let Some(to) = params.to {
        query.push_str(&format!(" AND created_at <= to_timestamp({})", to));
    }
    
    query.push_str(&format!(" ORDER BY created_at DESC LIMIT {}", limit));
    
    match sqlx::query_as::<_, MetricRow>(&query)
        .bind(&metric_type)
        .fetch_all(app_state.database.pool())
        .await
    {
        Ok(rows) => {
            let metrics: Vec<MetricResponse> = rows
                .into_iter()
                .map(|row| MetricResponse {
                    id: row.id,
                    metric_type: row.metric_type,
                    value: row.value.to_f64().unwrap_or(0.0),
                    quality_score: row.quality_score as u8,
                    privacy_level: row.privacy_level,
                    proof_id: row.proof_id,
                    data_sources: row.data_sources,
                    created_at: row.created_at,
                })
                .collect();
            
            Ok(Json(metrics))
        }
        Err(e) => {
            tracing::error!("获取历史指标数据失败: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// 提交网络指标
pub async fn submit_metric(
    Extension(app_state): Extension<AppState>,
    Json(request): Json<SubmitMetricRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // 验证输入数据
    if request.metric_type.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    if request.quality_score > 100 {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    // 如果有证明数据，先验证证明
    let mut proof_id: Option<String> = None;
    if let Some(proof) = &request.proof {
        // 调用零知识证明服务验证
        match app_state.services.zkproof_service.verify_proof_data(&proof).await {
            Ok(is_valid) if is_valid => {
                proof_id = Some(generate_proof_id(&proof));
            }
            Ok(_) => {
                tracing::warn!("无效的零知识证明");
                return Err(StatusCode::BAD_REQUEST);
            }
            Err(e) => {
                tracing::error!("证明验证失败: {}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    }
    
    // 存储指标数据
    match sqlx::query(
        "INSERT INTO network_metrics (metric_type, value, quality_score, privacy_level, proof_id, data_sources) 
         VALUES ($1, $2, $3, $4, $5, $6) 
         RETURNING id"
    )
    .bind(&request.metric_type)
    .bind(sqlx::types::BigDecimal::from(request.value as i64))
    .bind(request.quality_score as i16)
    .bind(&request.privacy_level)
    .bind(&proof_id)
    .bind(&request.data_sources)
    .fetch_one(app_state.database.pool())
    .await
    {
        Ok(row) => {
            let id: Uuid = row.get("id");
            
            Ok(Json(serde_json::json!({
                "status": "success",
                "message": "指标提交成功",
                "metric_id": id,
                "proof_verified": proof_id.is_some()
            })))
        }
        Err(e) => {
            tracing::error!("保存指标数据失败: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// 批量提交指标
pub async fn submit_metrics_batch(
    Extension(app_state): Extension<AppState>,
    Json(request): Json<BatchSubmitRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    if request.metrics.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    if request.metrics.len() > 100 {
        return Err(StatusCode::BAD_REQUEST); // 限制批量大小
    }
    
    let mut successful_count = 0;
    let mut failed_count = 0;
    let mut results = Vec::new();
    
    for (index, metric) in request.metrics.iter().enumerate() {
        // 这里重用单个提交的逻辑
        match submit_single_metric(app_state.clone(), metric).await {
            Ok(result) => {
                successful_count += 1;
                results.push(serde_json::json!({
                    "index": index,
                    "status": "success",
                    "result": result
                }));
            }
            Err(e) => {
                failed_count += 1;
                results.push(serde_json::json!({
                    "index": index,
                    "status": "failed",
                    "error": format!("{:?}", e)
                }));
            }
        }
    }
    
    Ok(Json(serde_json::json!({
        "status": "completed",
        "successful_count": successful_count,
        "failed_count": failed_count,
        "results": results
    })))
}

// 辅助函数和结构体

#[derive(sqlx::FromRow)]
struct MetricRow {
    id: Uuid,
    metric_type: String,
    value: sqlx::types::BigDecimal,
    quality_score: i16,
    privacy_level: String,
    proof_id: Option<String>,
    data_sources: Option<serde_json::Value>,
    created_at: chrono::DateTime<chrono::Utc>,
}

/// 单个指标提交的内部函数
async fn submit_single_metric(
    app_state: AppState,
    request: &SubmitMetricRequest,
) -> Result<serde_json::Value, anyhow::Error> {
    // 验证和存储逻辑（简化版本）
    let result = sqlx::query(
        "INSERT INTO network_metrics (metric_type, value, quality_score, privacy_level, data_sources) 
         VALUES ($1, $2, $3, $4, $5) 
         RETURNING id"
    )
    .bind(&request.metric_type)
    .bind(sqlx::types::BigDecimal::from(request.value as i64))
    .bind(request.quality_score as i16)
    .bind(&request.privacy_level)
    .bind(&request.data_sources)
    .fetch_one(app_state.database.pool())
    .await?;
    
    let id: Uuid = result.get("id");
    
    Ok(serde_json::json!({
        "metric_id": id,
        "submitted_at": chrono::Utc::now()
    }))
}

/// 生成证明ID
fn generate_proof_id(proof: &ProofData) -> String {
    use sha2::{Sha256, Digest};
    
    let mut hasher = Sha256::new();
    hasher.update(&proof.proof_value);
    hasher.update(&proof.circuit_id.to_be_bytes());
    
    format!("{:x}", hasher.finalize())
}"