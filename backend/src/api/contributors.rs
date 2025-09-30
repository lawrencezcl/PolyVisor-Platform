use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    response::Json,
    routing::{get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{error, info};

use crate::AppState;

/// 创建贡献者相关路由
pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(get_contributors).post(register_contributor))
        .route("/:address", get(get_contributor).put(update_contributor))
        .route("/:address/contributions", get(get_contributions))
        .route("/leaderboard", get(get_leaderboard))
}

/// 数据贡献者信息
#[derive(Debug, Serialize)]
pub struct ContributorInfo {
    /// 贡献者地址
    pub address: String,
    /// 显示名称
    pub display_name: Option<String>,
    /// 注册时间
    pub registered_at: chrono::DateTime<chrono::Utc>,
    /// 贡献者类型
    pub contributor_type: ContributorType,
    /// 信誉评分
    pub reputation_score: u32,
    /// 贡献统计
    pub contribution_stats: ContributionStats,
    /// 验证状态
    pub verification_status: VerificationStatus,
}

/// 贡献者类型
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContributorType {
    Individual,    // 个人贡献者
    Organization,  // 组织贡献者
    Validator,     // 验证节点
    DataProvider,  // 数据提供商
    Researcher,    // 研究机构
}

/// 贡献统计
#[derive(Debug, Serialize)]
pub struct ContributionStats {
    /// 总贡献数
    pub total_contributions: u64,
    /// 数据质量平均分
    pub avg_data_quality: f64,
    /// 最后贡献时间
    pub last_contribution_at: Option<chrono::DateTime<chrono::Utc>>,
    /// 按类型统计
    pub contributions_by_type: HashMap<String, u64>,
    /// 月度贡献趋势
    pub monthly_trends: Vec<MonthlyContribution>,
}

/// 月度贡献数据
#[derive(Debug, Serialize)]
pub struct MonthlyContribution {
    /// 月份（YYYY-MM格式）
    pub month: String,
    /// 贡献数量
    pub contribution_count: u64,
    /// 平均质量分
    pub avg_quality_score: f64,
}

/// 验证状态
#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum VerificationStatus {
    Unverified,  // 未验证
    Pending,     // 验证中
    Verified,    // 已验证
    Rejected,    // 验证被拒
}

/// 贡献者注册请求
#[derive(Debug, Deserialize)]
pub struct ContributorRegistrationRequest {
    /// 贡献者地址
    pub address: String,
    /// 显示名称
    pub display_name: Option<String>,
    /// 贡献者类型
    pub contributor_type: ContributorType,
    /// 联系信息
    pub contact_info: Option<ContactInfo>,
    /// 验证文档
    pub verification_documents: Option<Vec<VerificationDocument>>,
}

/// 联系信息
#[derive(Debug, Deserialize, Serialize)]
pub struct ContactInfo {
    /// 邮箱
    pub email: Option<String>,
    /// 网站
    pub website: Option<String>,
    /// 社交媒体链接
    pub social_links: Option<HashMap<String, String>>,
}

/// 验证文档
#[derive(Debug, Deserialize, Serialize)]
pub struct VerificationDocument {
    /// 文档类型
    pub document_type: String,
    /// 文档Hash
    pub document_hash: String,
    /// 文档描述
    pub description: String,
}

/// 贡献记录
#[derive(Debug, Serialize)]
pub struct ContributionRecord {
    /// 贡献ID
    pub contribution_id: String,
    /// 贡献者地址
    pub contributor_address: String,
    /// 贡献类型
    pub contribution_type: String,
    /// 数据质量评分
    pub quality_score: u8,
    /// 贡献时间
    pub contributed_at: chrono::DateTime<chrono::Utc>,
    /// 贡献数据摘要
    pub data_summary: DataSummary,
    /// 隐私级别
    pub privacy_level: String,
    /// 奖励信息
    pub reward_info: Option<RewardInfo>,
}

/// 数据摘要
#[derive(Debug, Serialize)]
pub struct DataSummary {
    /// 数据类型
    pub data_type: String,
    /// 数据大小（字节）
    pub data_size: u64,
    /// 数据条目数
    pub record_count: u64,
    /// 时间范围
    pub time_range: Option<TimeRange>,
}

/// 时间范围
#[derive(Debug, Serialize)]
pub struct TimeRange {
    /// 开始时间
    pub start_time: chrono::DateTime<chrono::Utc>,
    /// 结束时间
    pub end_time: chrono::DateTime<chrono::Utc>,
}

/// 奖励信息
#[derive(Debug, Serialize)]
pub struct RewardInfo {
    /// 基础奖励
    pub base_reward: u64,
    /// 质量奖金
    pub quality_bonus: u64,
    /// 奖励状态
    pub reward_status: RewardStatus,
    /// 发放时间
    pub distributed_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// 奖励状态
#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RewardStatus {
    Pending,     // 待发放
    Distributed, // 已发放
    Failed,      // 发放失败
}

/// 贡献者排行榜
#[derive(Debug, Serialize)]
pub struct ContributorLeaderboard {
    /// 排行榜类型
    pub leaderboard_type: LeaderboardType,
    /// 时间范围
    pub time_period: String,
    /// 排行榜条目
    pub entries: Vec<LeaderboardEntry>,
    /// 生成时间
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

/// 排行榜类型
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LeaderboardType {
    TotalContributions,  // 总贡献数
    QualityScore,       // 质量评分
    RecentActivity,     // 最近活跃度
    Reputation,         // 信誉评分
}

/// 排行榜条目
#[derive(Debug, Serialize)]
pub struct LeaderboardEntry {
    /// 排名
    pub rank: u32,
    /// 贡献者地址
    pub contributor_address: String,
    /// 显示名称
    pub display_name: Option<String>,
    /// 得分/数值
    pub score: f64,
    /// 变化（相比上期）
    pub change: i32,
}

/// 贡献者查询参数
#[derive(Debug, Deserialize)]
pub struct ContributorQuery {
    /// 贡献者类型过滤
    pub contributor_type: Option<ContributorType>,
    /// 验证状态过滤
    pub verification_status: Option<VerificationStatus>,
    /// 最小信誉评分
    pub min_reputation: Option<u32>,
    /// 活跃时间范围（天）
    pub active_within_days: Option<u32>,
    /// 排序字段
    pub sort_by: Option<String>,
    /// 排序方向
    pub sort_order: Option<String>,
    /// 分页参数
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

/// 注册新贡献者
pub async fn register_contributor(
    Extension(app_state): Extension<AppState>,
    Json(request): Json<ContributorRegistrationRequest>,
) -> Result<Json<ContributorInfo>, (StatusCode, Json<serde_json::Value>)> {
    info!("注册新贡献者，地址: {}", request.address);

    // 验证请求参数
    if request.address.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Invalid address",
                "message": "Contributor address cannot be empty"
            })),
        ));
    }

    match app_state
        .services
        .contributor_service
        .register_contributor(request)
        .await
    {
        Ok(contributor_info) => {
            info!("贡献者注册成功，地址: {}", contributor_info.address);
            Ok(Json(contributor_info))
        }
        Err(e) => {
            error!("贡献者注册失败: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to register contributor",
                    "message": e.to_string()
                })),
            ))
        }
    }
}

/// 获取贡献者信息
pub async fn get_contributor(
    Extension(app_state): Extension<AppState>,
    Path(address): Path<String>,
) -> Result<Json<ContributorInfo>, (StatusCode, Json<serde_json::Value>)> {
    info!("获取贡献者信息，地址: {}", address);

    if address.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Invalid address",
                "message": "Contributor address cannot be empty"
            })),
        ));
    }

    match app_state
        .services
        .contributor_service
        .get_contributor(&address)
        .await
    {
        Ok(contributor) => {
            info!("贡献者信息获取成功，地址: {}", address);
            Ok(Json(contributor))
        }
        Err(e) => {
            error!("贡献者信息获取失败: {}", e);
            Err((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": "Contributor not found",
                    "message": e.to_string()
                })),
            ))
        }
    }
}

/// 获取贡献者列表
pub async fn get_contributors(
    Extension(app_state): Extension<AppState>,
    Query(query): Query<ContributorQuery>,
) -> Result<Json<Vec<ContributorInfo>>, (StatusCode, Json<serde_json::Value>)> {
    info!("获取贡献者列表");

    let limit = query.limit.unwrap_or(50).min(1000);
    let offset = query.offset.unwrap_or(0);

    match app_state
        .services
        .contributor_service
        .get_contributors(query, limit, offset)
        .await
    {
        Ok(contributors) => {
            info!("贡献者列表获取成功，数量: {}", contributors.len());
            Ok(Json(contributors))
        }
        Err(e) => {
            error!("贡献者列表获取失败: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to get contributors",
                    "message": e.to_string()
                })),
            ))
        }
    }
}

/// 获取贡献记录
pub async fn get_contributions(
    Extension(app_state): Extension<AppState>,
    Path(address): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<ContributionRecord>>, (StatusCode, Json<serde_json::Value>)> {
    info!("获取贡献记录，地址: {}", address);

    let limit = params
        .get("limit")
        .and_then(|l| l.parse::<u32>().ok())
        .unwrap_or(50)
        .min(1000);
    let offset = params
        .get("offset")
        .and_then(|o| o.parse::<u32>().ok())
        .unwrap_or(0);

    match app_state
        .services
        .contributor_service
        .get_contributions(&address, limit, offset)
        .await
    {
        Ok(contributions) => {
            info!("贡献记录获取成功，数量: {}", contributions.len());
            Ok(Json(contributions))
        }
        Err(e) => {
            error!("贡献记录获取失败: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to get contributions",
                    "message": e.to_string()
                })),
            ))
        }
    }
}

/// 获取贡献者排行榜
pub async fn get_leaderboard(
    Extension(app_state): Extension<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ContributorLeaderboard>, (StatusCode, Json<serde_json::Value>)> {
    info!("获取贡献者排行榜");

    let leaderboard_type = params
        .get("type")
        .and_then(|t| serde_json::from_str::<LeaderboardType>(&format!(""{}"", t)).ok())
        .unwrap_or(LeaderboardType::TotalContributions);

    let time_period = params
        .get("period")
        .map(|p| p.clone())
        .unwrap_or_else(|| "monthly".to_string());

    let limit = params
        .get("limit")
        .and_then(|l| l.parse::<u32>().ok())
        .unwrap_or(100);

    match app_state
        .services
        .contributor_service
        .get_leaderboard(leaderboard_type, &time_period, limit)
        .await
    {
        Ok(leaderboard) => {
            info!("排行榜获取成功，条目数: {}", leaderboard.entries.len());
            Ok(Json(leaderboard))
        }
        Err(e) => {
            error!("排行榜获取失败: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to get leaderboard",
                    "message": e.to_string()
                })),
            ))
        }
    }
}

/// 更新贡献者信息
pub async fn update_contributor(
    Extension(app_state): Extension<AppState>,
    Path(address): Path<String>,
    Json(updates): Json<serde_json::Value>,
) -> Result<Json<ContributorInfo>, (StatusCode, Json<serde_json::Value>)> {
    info!("更新贡献者信息，地址: {}", address);

    if address.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Invalid address",
                "message": "Contributor address cannot be empty"
            })),
        ));
    }

    match app_state
        .services
        .contributor_service
        .update_contributor(&address, updates)
        .await
    {
        Ok(contributor) => {
            info!("贡献者信息更新成功，地址: {}", address);
            Ok(Json(contributor))
        }
        Err(e) => {
            error!("贡献者信息更新失败: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to update contributor",
                    "message": e.to_string()
                })),
            ))
        }
    }
}