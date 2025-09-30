use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    response::Json,
    routing::{get, post, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{error, info, warn};

use crate::AppState;

/// 创建零知识证明相关路由
pub fn create_routes() -> Router {
    Router::new()
        .route("/generate", post(generate_proof))
        .route("/verify", post(verify_proof))
        .route("/:proof_id", get(get_proof_status))
        .route("/:proof_id/cancel", delete(cancel_proof_generation))
        .route("/", get(get_proofs))
        .route("/statistics", get(get_proof_statistics))
}

/// 零知识证明类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofType {
    MetricSubmission,   // 指标提交证明
    PrivacyCompliance,  // 隐私合规证明
    DataIntegrity,     // 数据完整性证明
    ConsensusParticipation, // 共识参与证明
    NodeReliability,   // 节点可靠性证明
}

/// 证明生成请求
#[derive(Debug, Deserialize)]
pub struct ProofGenerationRequest {
    /// 证明类型
    pub proof_type: ProofType,
    /// 输入数据
    pub input_data: serde_json::Value,
    /// 隐私级别
    pub privacy_level: String,
    /// 请求方地址
    pub requester_address: String,
    /// 额外元数据
    pub metadata: Option<serde_json::Value>,
}

/// 证明生成响应
#[derive(Debug, Serialize)]
pub struct ProofGenerationResponse {
    /// 证明ID
    pub proof_id: String,
    /// 生成状态
    pub status: ProofGenerationStatus,
    /// 证明数据（如果已完成）
    pub proof_data: Option<ZKProofData>,
    /// 估计完成时间
    pub estimated_completion: Option<chrono::DateTime<chrono::Utc>>,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// 证明生成状态
#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ProofGenerationStatus {
    Pending,    // 等待处理
    Processing, // 正在生成
    Completed,  // 已完成
    Failed,     // 生成失败
    Expired,    // 已过期
}

/// 零知识证明数据
#[derive(Debug, Serialize)]
pub struct ZKProofData {
    /// 证明内容
    pub proof: String,
    /// 公共输入
    pub public_inputs: Vec<String>,
    /// 验证密钥
    pub verification_key: String,
    /// 证明元数据
    pub metadata: ProofMetadata,
}

/// 证明元数据
#[derive(Debug, Serialize)]
pub struct ProofMetadata {
    /// 证明算法
    pub algorithm: String,
    /// 安全参数
    pub security_parameter: u32,
    /// 证明大小（字节）
    pub proof_size: u64,
    /// 生成时间（毫秒）
    pub generation_time_ms: u64,
    /// 验证时间（毫秒）
    pub verification_time_ms: u64,
    /// 隐私保证级别
    pub privacy_guarantee: String,
}

/// 证明验证请求
#[derive(Debug, Deserialize)]
pub struct ProofVerificationRequest {
    /// 证明数据
    pub proof_data: ZKProofData,
    /// 验证上下文
    pub verification_context: Option<serde_json::Value>,
}

/// 证明验证响应
#[derive(Debug, Serialize)]
pub struct ProofVerificationResponse {
    /// 验证结果
    pub is_valid: bool,
    /// 验证状态
    pub verification_status: VerificationStatus,
    /// 验证详情
    pub verification_details: VerificationDetails,
    /// 验证时间
    pub verified_at: chrono::DateTime<chrono::Utc>,
}

/// 验证状态
#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum VerificationStatus {
    Valid,      // 验证通过
    Invalid,    // 验证失败
    Expired,    // 证明已过期
    Malformed,  // 证明格式错误
    Unknown,    // 未知错误
}

/// 验证详情
#[derive(Debug, Serialize)]
pub struct VerificationDetails {
    /// 验证算法
    pub algorithm_used: String,
    /// 验证耗时（毫秒）
    pub verification_time_ms: u64,
    /// 公共输入验证结果
    pub public_inputs_valid: bool,
    /// 证明结构验证结果
    pub proof_structure_valid: bool,
    /// 密码学验证结果
    pub cryptographic_verification: bool,
    /// 验证错误信息（if any）
    pub error_message: Option<String>,
}

/// 证明查询参数
#[derive(Debug, Deserialize)]
pub struct ProofQuery {
    /// 证明类型过滤
    pub proof_type: Option<ProofType>,
    /// 状态过滤
    pub status: Option<ProofGenerationStatus>,
    /// 请求方地址过滤
    pub requester: Option<String>,
    /// 时间范围开始
    pub from_time: Option<chrono::DateTime<chrono::Utc>>,
    /// 时间范围结束
    pub to_time: Option<chrono::DateTime<chrono::Utc>>,
    /// 分页限制
    pub limit: Option<u32>,
    /// 分页偏移
    pub offset: Option<u32>,
}

/// 证明列表响应
#[derive(Debug, Serialize)]
pub struct ProofListResponse {
    /// 证明列表
    pub proofs: Vec<ProofSummary>,
    /// 总数
    pub total_count: u64,
    /// 分页信息
    pub pagination: PaginationInfo,
}

/// 证明摘要
#[derive(Debug, Serialize)]
pub struct ProofSummary {
    /// 证明ID
    pub proof_id: String,
    /// 证明类型
    pub proof_type: ProofType,
    /// 状态
    pub status: ProofGenerationStatus,
    /// 请求方地址
    pub requester_address: String,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 完成时间
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    /// 证明大小
    pub proof_size: Option<u64>,
}

/// 分页信息
#[derive(Debug, Serialize)]
pub struct PaginationInfo {
    /// 当前页码
    pub current_page: u32,
    /// 每页大小
    pub page_size: u32,
    /// 总页数
    pub total_pages: u32,
    /// 是否有下一页
    pub has_next: bool,
    /// 是否有上一页
    pub has_prev: bool,
}

/// 证明统计信息
#[derive(Debug, Serialize)]
pub struct ProofStatistics {
    /// 总证明数
    pub total_proofs: u64,
    /// 按类型统计
    pub by_type: HashMap<ProofType, u64>,
    /// 按状态统计
    pub by_status: HashMap<ProofGenerationStatus, u64>,
    /// 平均生成时间（毫秒）
    pub avg_generation_time_ms: f64,
    /// 成功率
    pub success_rate: f64,
    /// 最近24小时统计
    pub last_24h_stats: DailyStats,
}

/// 日统计
#[derive(Debug, Serialize)]
pub struct DailyStats {
    /// 生成数量
    pub generated_count: u64,
    /// 验证数量
    pub verified_count: u64,
    /// 失败数量
    pub failed_count: u64,
    /// 平均响应时间
    pub avg_response_time_ms: f64,
}

/// 生成零知识证明
pub async fn generate_proof(
    Extension(app_state): Extension<AppState>,
    Json(request): Json<ProofGenerationRequest>,
) -> Result<Json<ProofGenerationResponse>, (StatusCode, Json<serde_json::Value>)> {
    info!("生成零知识证明请求，类型: {:?}，请求方: {}", request.proof_type, request.requester_address);

    // 验证请求参数
    if request.requester_address.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Invalid requester address",
                "message": "Requester address cannot be empty"
            })),
        ));
    }

    match app_state
        .services
        .zkproof_service
        .generate_proof(request)
        .await
    {
        Ok(response) => {
            info!("证明生成请求已提交，证明ID: {}", response.proof_id);
            Ok(Json(response))
        }
        Err(e) => {
            error!("证明生成失败: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to generate proof",
                    "message": e.to_string()
                })),
            ))
        }
    }
}

/// 验证零知识证明
pub async fn verify_proof(
    Extension(app_state): Extension<AppState>,
    Json(request): Json<ProofVerificationRequest>,
) -> Result<Json<ProofVerificationResponse>, (StatusCode, Json<serde_json::Value>)> {
    info!("验证零知识证明请求");

    match app_state
        .services
        .zkproof_service
        .verify_proof(request)
        .await
    {
        Ok(response) => {
            info!("证明验证完成，结果: {}", response.is_valid);
            Ok(Json(response))
        }
        Err(e) => {
            error!("证明验证失败: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to verify proof",
                    "message": e.to_string()
                })),
            ))
        }
    }
}

/// 获取证明状态
pub async fn get_proof_status(
    Extension(app_state): Extension<AppState>,
    Path(proof_id): Path<String>,
) -> Result<Json<ProofGenerationResponse>, (StatusCode, Json<serde_json::Value>)> {
    info!("获取证明状态，证明ID: {}", proof_id);

    if proof_id.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Invalid proof ID",
                "message": "Proof ID cannot be empty"
            })),
        ));
    }

    match app_state
        .services
        .zkproof_service
        .get_proof_status(&proof_id)
        .await
    {
        Ok(status) => {
            info!("证明状态获取成功，状态: {:?}", status.status);
            Ok(Json(status))
        }
        Err(e) => {
            error!("证明状态获取失败: {}", e);
            Err((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": "Proof not found",
                    "message": e.to_string()
                })),
            ))
        }
    }
}

/// 获取证明列表
pub async fn get_proofs(
    Extension(app_state): Extension<AppState>,
    Query(query): Query<ProofQuery>,
) -> Result<Json<ProofListResponse>, (StatusCode, Json<serde_json::Value>)> {
    info!("获取证明列表");

    let limit = query.limit.unwrap_or(50).min(1000); // 最大1000条
    let offset = query.offset.unwrap_or(0);

    match app_state
        .services
        .zkproof_service
        .get_proofs(query, limit, offset)
        .await
    {
        Ok(response) => {
            info!("证明列表获取成功，数量: {}", response.proofs.len());
            Ok(Json(response))
        }
        Err(e) => {
            error!("证明列表获取失败: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to get proofs",
                    "message": e.to_string()
                })),
            ))
        }
    }
}

/// 获取证明统计信息
pub async fn get_proof_statistics(
    Extension(app_state): Extension<AppState>,
) -> Result<Json<ProofStatistics>, (StatusCode, Json<serde_json::Value>)> {
    info!("获取证明统计信息");

    match app_state
        .services
        .zkproof_service
        .get_statistics()
        .await
    {
        Ok(stats) => {
            info!("证明统计信息获取成功，总数: {}", stats.total_proofs);
            Ok(Json(stats))
        }
        Err(e) => {
            error!("证明统计信息获取失败: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to get proof statistics",
                    "message": e.to_string()
                })),
            ))
        }
    }
}

/// 取消证明生成
pub async fn cancel_proof_generation(
    Extension(app_state): Extension<AppState>,
    Path(proof_id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    info!("取消证明生成，证明ID: {}", proof_id);

    if proof_id.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Invalid proof ID",
                "message": "Proof ID cannot be empty"
            })),
        ));
    }

    match app_state
        .services
        .zkproof_service
        .cancel_proof_generation(&proof_id)
        .await
    {
        Ok(_) => {
            info!("证明生成已取消，证明ID: {}", proof_id);
            Ok(Json(serde_json::json!({
                "status": "cancelled",
                "proof_id": proof_id,
                "message": "Proof generation has been cancelled",
                "cancelled_at": chrono::Utc::now()
            })))
        }
        Err(e) => {
            error!("取消证明生成失败: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to cancel proof generation",
                    "message": e.to_string()
                })),
            ))
        }
    }
}