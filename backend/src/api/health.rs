use axum::{
    extract::{Extension, Query},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{error, info};

use crate::AppState;

/// 创建健康检查相关路由
pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(quick_health_check))
        .route("/network", get(get_network_health))
        .route("/detailed", get(get_detailed_health_report))
        .route("/components", get(get_component_health))
}

/// 网络健康状况查询参数
#[derive(Debug, Deserialize)]
pub struct HealthQuery {
    /// 时间范围（秒）
    pub time_range: Option<u64>,
    /// 网络类型过滤
    pub network_type: Option<String>,
    /// 详细程度
    pub detail_level: Option<String>,
}

/// 网络健康状况响应
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    /// 整体健康评分 (0-100)
    pub overall_score: u8,
    /// 网络状态
    pub network_status: NetworkStatus,
    /// 各项指标
    pub metrics: HealthMetrics,
    /// 历史趋势
    pub trends: HealthTrends,
    /// 警告信息
    pub warnings: Vec<HealthWarning>,
    /// 数据更新时间
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

/// 网络状态枚举
#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum NetworkStatus {
    Healthy,
    Warning,
    Critical,
    Unknown,
}

/// 健康指标详情
#[derive(Debug, Serialize)]
pub struct HealthMetrics {
    /// 节点连通性 (0-100)
    pub connectivity_score: u8,
    /// 交易处理能力 (0-100)
    pub throughput_score: u8,
    /// 网络延迟评分 (0-100)
    pub latency_score: u8,
    /// 共识稳定性 (0-100)
    pub consensus_score: u8,
    /// 数据可用性 (0-100)
    pub availability_score: u8,
}

/// 健康趋势数据
#[derive(Debug, Serialize)]
pub struct HealthTrends {
    /// 24小时趋势
    pub daily_trend: i8, // -100 到 100
    /// 7天趋势
    pub weekly_trend: i8,
    /// 30天趋势
    pub monthly_trend: i8,
}

/// 健康警告
#[derive(Debug, Serialize)]
pub struct HealthWarning {
    /// 警告级别
    pub level: WarningLevel,
    /// 警告消息
    pub message: String,
    /// 影响的组件
    pub component: String,
    /// 建议操作
    pub recommendation: Option<String>,
    /// 警告时间
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// 警告级别
#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum WarningLevel {
    Info,
    Warning,
    Critical,
}

/// 详细健康报告
#[derive(Debug, Serialize)]
pub struct DetailedHealthReport {
    /// 基础健康信息
    #[serde(flatten)]
    pub health: HealthResponse,
    /// 详细的组件状态
    pub components: HashMap<String, ComponentHealth>,
    /// 性能统计
    pub performance_stats: PerformanceStats,
    /// 网络拓扑信息
    pub topology_info: TopologyInfo,
}

/// 组件健康状态
#[derive(Debug, Serialize)]
pub struct ComponentHealth {
    /// 组件名称
    pub name: String,
    /// 健康状态
    pub status: NetworkStatus,
    /// 健康评分
    pub score: u8,
    /// 最后检查时间
    pub last_check: chrono::DateTime<chrono::Utc>,
    /// 详细信息
    pub details: serde_json::Value,
}

/// 性能统计信息
#[derive(Debug, Serialize)]
pub struct PerformanceStats {
    /// 平均响应时间 (ms)
    pub avg_response_time: f64,
    /// 最大响应时间 (ms)
    pub max_response_time: f64,
    /// 成功率 (%)
    pub success_rate: f64,
    /// QPS (每秒查询数)
    pub queries_per_second: f64,
    /// 错误率 (%)
    pub error_rate: f64,
}

/// 网络拓扑信息
#[derive(Debug, Serialize)]
pub struct TopologyInfo {
    /// 活跃节点数
    pub active_nodes: u32,
    /// 总节点数
    pub total_nodes: u32,
    /// 网络分片数
    pub shard_count: u32,
    /// 平均连接数
    pub avg_connections: f64,
    /// 网络直径
    pub network_diameter: u32,
}

/// 获取网络健康状况
pub async fn get_network_health(
    Extension(app_state): Extension<AppState>,
    Query(params): Query<HealthQuery>,
) -> Result<Json<HealthResponse>, (StatusCode, Json<serde_json::Value>)> {
    info!("获取网络健康状况，参数: {:?}", params);

    let time_range = params.time_range.unwrap_or(3600); // 默认1小时
    let detail_level = params.detail_level.unwrap_or_else(|| "basic".to_string());

    match app_state
        .services
        .health_service
        .get_network_health(time_range, &detail_level)
        .await
    {
        Ok(health) => {
            info!("网络健康状况获取成功，评分: {}", health.overall_score);
            Ok(Json(health))
        }
        Err(e) => {
            error!("获取网络健康状况失败: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to get network health",
                    "message": e.to_string()
                })),
            ))
        }
    }
}

/// 获取详细健康报告
pub async fn get_detailed_health_report(
    Extension(app_state): Extension<AppState>,
    Query(params): Query<HealthQuery>,
) -> Result<Json<DetailedHealthReport>, (StatusCode, Json<serde_json::Value>)> {
    info!("获取详细健康报告，参数: {:?}", params);

    let time_range = params.time_range.unwrap_or(3600);

    match app_state
        .services
        .health_service
        .get_detailed_health_report(time_range)
        .await
    {
        Ok(report) => {
            info!("详细健康报告获取成功");
            Ok(Json(report))
        }
        Err(e) => {
            error!("获取详细健康报告失败: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to get detailed health report",
                    "message": e.to_string()
                })),
            ))
        }
    }
}

/// 获取组件健康状态
pub async fn get_component_health(
    Extension(app_state): Extension<AppState>,
) -> Result<Json<HashMap<String, ComponentHealth>>, (StatusCode, Json<serde_json::Value>)> {
    info!("获取组件健康状态");

    match app_state
        .services
        .health_service
        .get_component_health()
        .await
    {
        Ok(components) => {
            info!("组件健康状态获取成功，组件数: {}", components.len());
            Ok(Json(components))
        }
        Err(e) => {
            error!("获取组件健康状态失败: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to get component health",
                    "message": e.to_string()
                })),
            ))
        }
    }
}

/// 健康检查端点（轻量级）
pub async fn quick_health_check(
    Extension(app_state): Extension<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    match app_state
        .services
        .health_service
        .quick_health_check()
        .await
    {
        Ok(status) => Ok(Json(serde_json::json!({
            "status": "ok",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "quick_check": status
        }))),
        Err(e) => {
            error!("快速健康检查失败: {}", e);
            Err((
                StatusCode::SERVICE_UNAVAILABLE,
                Json(serde_json::json!({
                    "status": "error",
                    "error": e.to_string(),
                    "timestamp": chrono::Utc::now().to_rfc3339()
                })),
            ))
        }
    }
}