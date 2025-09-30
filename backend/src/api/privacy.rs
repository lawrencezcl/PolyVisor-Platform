use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{error, info, warn};

use crate::AppState;

/// 创建隐私设置相关路由
pub fn create_routes() -> Router {
    Router::new()
        .route("/settings", post(set_privacy_settings))
        .route("/settings/:address", get(get_privacy_settings))
        .route("/audit/:address", get(get_privacy_audit_log))
        .route("/compliance", get(generate_compliance_report))
        .route("/deletion/:address", post(request_data_deletion))
}

/// 隐私级别枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PrivacyLevel {
    Public,    // 公开数据
    Protected, // 受保护数据（部分匿名化）
    Private,   // 私有数据（完全匿名化）
    Sensitive, // 敏感数据（零知识证明）
}

/// 隐私设置请求
#[derive(Debug, Deserialize)]
pub struct PrivacySettingsRequest {
    /// 用户地址
    pub user_address: String,
    /// 数据类型的隐私设置
    pub data_privacy_settings: HashMap<String, PrivacyLevel>,
    /// 数据保留时间（秒）
    pub data_retention_period: Option<u64>,
    /// 是否允许数据分析
    pub allow_analytics: bool,
    /// 是否允许数据共享
    pub allow_sharing: bool,
}

/// 隐私设置响应
#[derive(Debug, Serialize)]
pub struct PrivacySettingsResponse {
    /// 用户地址
    pub user_address: String,
    /// 当前隐私设置
    pub current_settings: PrivacySettings,
    /// 设置更新时间
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// 生效时间
    pub effective_from: chrono::DateTime<chrono::Utc>,
}

/// 隐私设置详情
#[derive(Debug, Serialize)]
pub struct PrivacySettings {
    /// 数据类型隐私级别映射
    pub data_privacy_levels: HashMap<String, PrivacyLevel>,
    /// 数据保留策略
    pub retention_policy: RetentionPolicy,
    /// 分析权限
    pub analytics_permissions: AnalyticsPermissions,
    /// 共享权限
    pub sharing_permissions: SharingPermissions,
    /// 匿名化配置
    pub anonymization_config: AnonymizationConfig,
}

/// 数据保留策略
#[derive(Debug, Serialize)]
pub struct RetentionPolicy {
    /// 默认保留时间（秒）
    pub default_retention_period: u64,
    /// 按数据类型的保留时间
    pub type_specific_retention: HashMap<String, u64>,
    /// 自动清理设置
    pub auto_cleanup_enabled: bool,
}

/// 分析权限设置
#[derive(Debug, Serialize)]
pub struct AnalyticsPermissions {
    /// 是否允许趋势分析
    pub allow_trend_analysis: bool,
    /// 是否允许聚合统计
    pub allow_aggregation: bool,
    /// 是否允许机器学习
    pub allow_ml_training: bool,
    /// 允许的分析类型
    pub allowed_analysis_types: Vec<String>,
}

/// 共享权限设置
#[derive(Debug, Serialize)]
pub struct SharingPermissions {
    /// 是否允许与研究机构共享
    pub allow_research_sharing: bool,
    /// 是否允许与合作伙伴共享
    pub allow_partner_sharing: bool,
    /// 是否允许公开聚合数据
    pub allow_public_aggregates: bool,
    /// 共享数据的最小聚合级别
    pub min_aggregation_level: u32,
}

/// 匿名化配置
#[derive(Debug, Serialize)]
pub struct AnonymizationConfig {
    /// K-匿名性参数
    pub k_anonymity_level: u32,
    /// 差分隐私参数
    pub differential_privacy_epsilon: f64,
    /// 数据泛化级别
    pub generalization_level: u8,
    /// 噪声添加策略
    pub noise_strategy: NoiseStrategy,
}

/// 噪声策略
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NoiseStrategy {
    Laplace,    // 拉普拉斯噪声
    Gaussian,   // 高斯噪声
    Exponential, // 指数噪声
    None,       // 无噪声
}

/// 隐私审计记录
#[derive(Debug, Serialize)]
pub struct PrivacyAuditRecord {
    /// 审计ID
    pub audit_id: String,
    /// 用户地址
    pub user_address: String,
    /// 操作类型
    pub operation_type: String,
    /// 数据类型
    pub data_type: String,
    /// 隐私级别
    pub privacy_level: PrivacyLevel,
    /// 操作时间
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// 操作结果
    pub result: AuditResult,
    /// 额外信息
    pub metadata: serde_json::Value,
}

/// 审计结果
#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AuditResult {
    Success,
    Failed,
    Blocked,
    Warning,
}

/// 隐私合规报告
#[derive(Debug, Serialize)]
pub struct PrivacyComplianceReport {
    /// 报告生成时间
    pub generated_at: chrono::DateTime<chrono::Utc>,
    /// 合规状态
    pub compliance_status: ComplianceStatus,
    /// 用户隐私设置统计
    pub privacy_settings_stats: PrivacyStatsummary,
    /// 数据处理合规性
    pub data_processing_compliance: DataProcessingCompliance,
    /// 建议改进项
    pub recommendations: Vec<ComplianceRecommendation>,
}

/// 合规状态
#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ComplianceStatus {
    Compliant,
    PartiallyCompliant,
    NonCompliant,
}

/// 隐私设置统计
#[derive(Debug, Serialize)]
pub struct PrivacyStatsummary {
    /// 总用户数
    pub total_users: u64,
    /// 各隐私级别用户分布
    pub privacy_level_distribution: HashMap<PrivacyLevel, u64>,
    /// 数据保留策略分布
    pub retention_policy_distribution: HashMap<String, u64>,
}

/// 数据处理合规性
#[derive(Debug, Serialize)]
pub struct DataProcessingCompliance {
    /// 匿名化处理率
    pub anonymization_rate: f64,
    /// 数据最小化实施率
    pub data_minimization_rate: f64,
    /// 同意管理合规性
    pub consent_management_score: u8,
    /// 数据删除及时性
    pub deletion_timeliness_score: u8,
}

/// 合规建议
#[derive(Debug, Serialize)]
pub struct ComplianceRecommendation {
    /// 建议类别
    pub category: String,
    /// 建议内容
    pub recommendation: String,
    /// 优先级
    pub priority: RecommendationPriority,
    /// 预计影响
    pub impact: String,
}

/// 建议优先级
#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RecommendationPriority {
    High,
    Medium,
    Low,
}

/// 设置用户隐私偏好
pub async fn set_privacy_settings(
    Extension(app_state): Extension<AppState>,
    Json(request): Json<PrivacySettingsRequest>,
) -> Result<Json<PrivacySettingsResponse>, (StatusCode, Json<serde_json::Value>)> {
    info!("设置用户隐私偏好，用户: {}", request.user_address);

    // 验证请求参数
    if request.user_address.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Invalid user address",
                "message": "User address cannot be empty"
            })),
        ));
    }

    match app_state
        .services
        .privacy_service
        .update_privacy_settings(request)
        .await
    {
        Ok(response) => {
            info!("隐私设置更新成功，用户: {}", response.user_address);
            Ok(Json(response))
        }
        Err(e) => {
            error!("隐私设置更新失败: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to update privacy settings",
                    "message": e.to_string()
                })),
            ))
        }
    }
}

/// 获取用户隐私设置
pub async fn get_privacy_settings(
    Extension(app_state): Extension<AppState>,
    Path(user_address): Path<String>,
) -> Result<Json<PrivacySettings>, (StatusCode, Json<serde_json::Value>)> {
    info!("获取用户隐私设置，用户: {}", user_address);

    if user_address.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Invalid user address",
                "message": "User address cannot be empty"
            })),
        ));
    }

    match app_state
        .services
        .privacy_service
        .get_privacy_settings(&user_address)
        .await
    {
        Ok(settings) => {
            info!("隐私设置获取成功，用户: {}", user_address);
            Ok(Json(settings))
        }
        Err(e) => {
            error!("隐私设置获取失败: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to get privacy settings",
                    "message": e.to_string()
                })),
            ))
        }
    }
}

/// 获取隐私审计日志
pub async fn get_privacy_audit_log(
    Extension(app_state): Extension<AppState>,
    Path(user_address): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<PrivacyAuditRecord>>, (StatusCode, Json<serde_json::Value>)> {
    info!("获取隐私审计日志，用户: {}", user_address);

    let limit = params
        .get("limit")
        .and_then(|l| l.parse::<u32>().ok())
        .unwrap_or(100);
    let offset = params
        .get("offset")
        .and_then(|o| o.parse::<u32>().ok())
        .unwrap_or(0);

    match app_state
        .services
        .privacy_service
        .get_audit_log(&user_address, limit, offset)
        .await
    {
        Ok(records) => {
            info!("隐私审计日志获取成功，记录数: {}", records.len());
            Ok(Json(records))
        }
        Err(e) => {
            error!("隐私审计日志获取失败: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to get privacy audit log",
                    "message": e.to_string()
                })),
            ))
        }
    }
}

/// 生成隐私合规报告
pub async fn generate_compliance_report(
    Extension(app_state): Extension<AppState>,
) -> Result<Json<PrivacyComplianceReport>, (StatusCode, Json<serde_json::Value>)> {
    info!("生成隐私合规报告");

    match app_state
        .services
        .privacy_service
        .generate_compliance_report()
        .await
    {
        Ok(report) => {
            info!("隐私合规报告生成成功");
            Ok(Json(report))
        }
        Err(e) => {
            error!("隐私合规报告生成失败: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to generate compliance report",
                    "message": e.to_string()
                })),
            ))
        }
    }
}

/// 请求数据删除
pub async fn request_data_deletion(
    Extension(app_state): Extension<AppState>,
    Path(user_address): Path<String>,
    Json(request): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    info!("处理数据删除请求，用户: {}", user_address);

    let data_types = request
        .get("data_types")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect::<Vec<String>>()
        })
        .unwrap_or_else(|| vec!["all".to_string()]);

    match app_state
        .services
        .privacy_service
        .request_data_deletion(&user_address, &data_types)
        .await
    {
        Ok(deletion_id) => {
            info!("数据删除请求已提交，删除ID: {}", deletion_id);
            Ok(Json(serde_json::json!({
                "status": "accepted",
                "deletion_id": deletion_id,
                "message": "Data deletion request has been submitted and will be processed",
                "estimated_completion": chrono::Utc::now() + chrono::Duration::days(30)
            })))
        }
        Err(e) => {
            error!("数据删除请求失败: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to process data deletion request",
                    "message": e.to_string()
                })),
            ))
        }
    }
}