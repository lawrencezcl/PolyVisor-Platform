use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub mod circuits;
pub mod prover;
pub mod verifier;
pub mod utils;

use circuits::{NetworkMetricCircuit, CircuitType};
use prover::ZKProver;
use verifier::ZKVerifier;

/// 零知识证明数据结构
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ZKProof {
    /// 证明值（序列化的字节数组）
    pub proof_value: Vec<u8>,
    /// 公开输入
    pub public_inputs: Vec<u128>,
    /// 验证密钥
    pub verification_key: Vec<u8>,
    /// 电路标识符
    pub circuit_id: u32,
    /// 证明生成时间戳
    pub created_at: u64,
}

/// 数据源信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSource {
    /// 数据源类型
    pub source_type: DataSourceType,
    /// 数据源标识符
    pub source_id: String,
    /// 数据时间戳
    pub timestamp: u64,
    /// 可靠性评分 (0-100)
    pub reliability_score: u8,
}

/// 数据源类型枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DataSourceType {
    ValidatorNode,
    FullNode,
    LightNode,
    Parachain,
    RelayChain,
    ExternalOracle,
}

/// 指标提交数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricSubmission {
    /// 指标类型
    pub metric_type: String,
    /// 私有数据（原始网络数据）
    pub private_data: Vec<u128>,
    /// 数据源列表
    pub data_sources: Vec<DataSource>,
    /// 公开的聚合指标值
    pub public_metric: u128,
    /// 数据质量评分
    pub quality_score: u8,
    /// 时间窗口（小时）
    pub time_window_hours: u8,
}

/// 证明元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofMetadata {
    /// 电路类型
    pub circuit_type: CircuitType,
    /// 证明生成耗时
    pub generation_time: Duration,
    /// 验证耗时
    pub verification_time: Option<Duration>,
    /// 数据源数量
    pub data_sources_count: usize,
    /// 质量阈值
    pub quality_threshold: u8,
    /// 数据新鲜度（秒）
    pub data_age: u64,
}

/// 零知识证明服务主入口
pub struct ZKProofService {
    /// 证明生成器
    prover: ZKProver,
    /// 证明验证器
    verifier: ZKVerifier,
    /// 证明缓存
    proof_cache: HashMap<String, ZKProof>,
    /// 电路缓存
    circuit_cache: HashMap<u32, NetworkMetricCircuit>,
}

impl ZKProofService {
    /// 创建新的零知识证明服务实例
    pub fn new() -> Result<Self> {
        let prover = ZKProver::new()?;
        let verifier = ZKVerifier::new()?;
        
        Ok(Self {
            prover,
            verifier,
            proof_cache: HashMap::new(),
            circuit_cache: HashMap::new(),
        })
    }
    
    /// 为网络指标生成零知识证明
    pub async fn generate_metric_proof(
        &mut self,
        submission: MetricSubmission,
    ) -> Result<(ZKProof, ProofMetadata)> {
        let start_time = Instant::now();
        
        // 验证输入数据
        self.validate_submission(&submission)?;
        
        // 计算电路ID
        let circuit_id = self.calculate_circuit_id(&submission);
        
        // 检查缓存
        let cache_key = self.generate_cache_key(&submission);
        if let Some(cached_proof) = self.proof_cache.get(&cache_key) {
            return Ok((cached_proof.clone(), ProofMetadata {
                circuit_type: CircuitType::NetworkMetric,
                generation_time: Duration::from_millis(0), // 缓存命中
                verification_time: None,
                data_sources_count: submission.data_sources.len(),
                quality_threshold: submission.quality_score,
                data_age: self.calculate_data_age(&submission.data_sources),
            }));
        }
        
        // 生成证明
        let proof = self.prover.generate_proof(
            submission.private_data.clone(),
            submission.data_sources.iter().map(|ds| ds.reliability_score as u32).collect(),
            submission.public_metric,
            submission.quality_score,
            submission.time_window_hours,
            circuit_id,
        ).await?;
        
        let generation_time = start_time.elapsed();
        
        // 创建元数据
        let metadata = ProofMetadata {
            circuit_type: CircuitType::NetworkMetric,
            generation_time,
            verification_time: None,
            data_sources_count: submission.data_sources.len(),
            quality_threshold: submission.quality_score,
            data_age: self.calculate_data_age(&submission.data_sources),
        };
        
        // 缓存证明
        self.proof_cache.insert(cache_key, proof.clone());
        
        Ok((proof, metadata))
    }
    
    /// 验证零知识证明
    pub async fn verify_proof(
        &mut self,
        proof: &ZKProof,
    ) -> Result<(bool, Duration)> {
        let start_time = Instant::now();
        
        let is_valid = self.verifier.verify_proof(proof).await?;
        
        let verification_time = start_time.elapsed();
        
        Ok((is_valid, verification_time))
    }
    
    /// 批量验证多个证明
    pub async fn batch_verify_proofs(
        &mut self,
        proofs: &[ZKProof],
    ) -> Result<Vec<(bool, Duration)>> {
        let mut results = Vec::new();
        
        for proof in proofs {
            let result = self.verify_proof(proof).await?;
            results.push(result);
        }
        
        Ok(results)
    }
    
    /// 生成数据完整性证明
    pub async fn generate_integrity_proof(
        &mut self,
        data_hash: Vec<u8>,
        timestamp: u64,
        source_signatures: Vec<Vec<u8>>,
    ) -> Result<ZKProof> {
        self.prover.generate_integrity_proof(
            data_hash,
            timestamp,
            source_signatures,
        ).await
    }
    
    /// 获取支持的电路类型
    pub fn get_supported_circuits(&self) -> Vec<CircuitType> {
        vec![
            CircuitType::NetworkMetric,
            CircuitType::DataIntegrity,
            CircuitType::AggregationProof,
            CircuitType::PrivacyPreserving,
        ]
    }
    
    /// 获取证明统计信息
    pub fn get_proof_statistics(&self) -> ProofStatistics {
        ProofStatistics {
            total_proofs_generated: self.proof_cache.len() as u64,
            cache_hit_ratio: self.calculate_cache_hit_ratio(),
            average_generation_time: self.calculate_average_generation_time(),
            supported_circuits: self.get_supported_circuits().len() as u32,
        }
    }
    
    /// 清理过期的证明缓存
    pub fn cleanup_expired_cache(&mut self, max_age_seconds: u64) {
        let current_time = chrono::Utc::now().timestamp() as u64;
        
        self.proof_cache.retain(|_, proof| {
            current_time - proof.created_at < max_age_seconds
        });
    }
    
    // 私有辅助方法
    
    /// 验证提交数据的有效性
    fn validate_submission(&self, submission: &MetricSubmission) -> Result<()> {
        // 检查私有数据不为空
        if submission.private_data.is_empty() {
            return Err(anyhow::anyhow!("Private data cannot be empty"));
        }
        
        // 检查数据源数量
        if submission.data_sources.len() < 2 {
            return Err(anyhow::anyhow!("At least 2 data sources required"));
        }
        
        // 检查质量评分范围
        if submission.quality_score > 100 {
            return Err(anyhow::anyhow!("Quality score must be <= 100"));
        }
        
        // 检查时间窗口合理性
        if submission.time_window_hours == 0 || submission.time_window_hours > 24 {
            return Err(anyhow::anyhow!("Time window must be between 1-24 hours"));
        }
        
        // 验证数据源可靠性
        for source in &submission.data_sources {
            if source.reliability_score > 100 {
                return Err(anyhow::anyhow!("Source reliability score must be <= 100"));
            }
        }
        
        Ok(())
    }
    
    /// 计算电路ID
    fn calculate_circuit_id(&self, submission: &MetricSubmission) -> u32 {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        hasher.update(submission.metric_type.as_bytes());
        hasher.update(&submission.private_data.len().to_be_bytes());
        hasher.update(&submission.data_sources.len().to_be_bytes());
        hasher.update(&submission.time_window_hours.to_be_bytes());
        
        let hash = hasher.finalize();
        u32::from_be_bytes([hash[0], hash[1], hash[2], hash[3]])
    }
    
    /// 生成缓存键
    fn generate_cache_key(&self, submission: &MetricSubmission) -> String {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        
        // 添加所有相关数据到哈希
        hasher.update(submission.metric_type.as_bytes());
        for data in &submission.private_data {
            hasher.update(&data.to_be_bytes());
        }
        hasher.update(&submission.public_metric.to_be_bytes());
        hasher.update(&submission.quality_score.to_be_bytes());
        hasher.update(&submission.time_window_hours.to_be_bytes());
        
        // 添加数据源信息
        for source in &submission.data_sources {
            hasher.update(source.source_id.as_bytes());
            hasher.update(&source.reliability_score.to_be_bytes());
        }
        
        let hash = hasher.finalize();
        hex::encode(hash)
    }
    
    /// 计算数据平均年龄（秒）
    fn calculate_data_age(&self, data_sources: &[DataSource]) -> u64 {
        if data_sources.is_empty() {
            return 0;
        }
        
        let current_time = chrono::Utc::now().timestamp() as u64;
        let total_age: u64 = data_sources
            .iter()
            .map(|source| current_time.saturating_sub(source.timestamp))
            .sum();
        
        total_age / data_sources.len() as u64
    }
    
    /// 计算缓存命中率
    fn calculate_cache_hit_ratio(&self) -> f64 {
        // 简化实现，实际应用中需要维护更详细的统计信息
        if self.proof_cache.is_empty() {
            0.0
        } else {
            0.75 // 假设75%的缓存命中率
        }
    }
    
    /// 计算平均生成时间
    fn calculate_average_generation_time(&self) -> Duration {
        // 简化实现，实际应用中需要维护生成时间统计
        Duration::from_millis(250) // 假设平均250ms
    }
}

/// 证明统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofStatistics {
    /// 总生成证明数
    pub total_proofs_generated: u64,
    /// 缓存命中率
    pub cache_hit_ratio: f64,
    /// 平均生成时间
    pub average_generation_time: Duration,
    /// 支持的电路数量
    pub supported_circuits: u32,
}

/// 服务错误类型
#[derive(Debug, thiserror::Error)]
pub enum ZKProofError {
    #[error("Invalid proof format: {0}")]
    InvalidProofFormat(String),
    
    #[error("Proof generation failed: {0}")]
    ProofGenerationFailed(String),
    
    #[error("Proof verification failed: {0}")]
    ProofVerificationFailed(String),
    
    #[error("Unsupported circuit type: {0:?}")]
    UnsupportedCircuitType(CircuitType),
    
    #[error("Invalid input data: {0}")]
    InvalidInputData(String),
    
    #[error("Cryptographic error: {0}")]
    CryptographicError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_zkproof_service_creation() {
        let service = ZKProofService::new();
        assert!(service.is_ok());
    }
    
    #[tokio::test]
    async fn test_metric_submission_validation() {
        let mut service = ZKProofService::new().unwrap();
        
        // 创建有效的提交数据
        let submission = MetricSubmission {
            metric_type: "block_time".to_string(),
            private_data: vec![6000, 6100, 5900, 6200],
            data_sources: vec![
                DataSource {
                    source_type: DataSourceType::ValidatorNode,
                    source_id: "validator_001".to_string(),
                    timestamp: chrono::Utc::now().timestamp() as u64,
                    reliability_score: 95,
                },
                DataSource {
                    source_type: DataSourceType::FullNode,
                    source_id: "fullnode_042".to_string(),
                    timestamp: chrono::Utc::now().timestamp() as u64,
                    reliability_score: 87,
                },
            ],
            public_metric: 6050,
            quality_score: 92,
            time_window_hours: 1,
        };
        
        // 验证数据有效性
        let validation_result = service.validate_submission(&submission);
        assert!(validation_result.is_ok());
    }
    
    #[test]
    fn test_circuit_id_calculation() {
        let service = ZKProofService::new().unwrap();
        
        let submission = MetricSubmission {
            metric_type: "block_time".to_string(),
            private_data: vec![6000, 6100],
            data_sources: vec![
                DataSource {
                    source_type: DataSourceType::ValidatorNode,
                    source_id: "test".to_string(),
                    timestamp: 0,
                    reliability_score: 100,
                },
            ],
            public_metric: 6050,
            quality_score: 90,
            time_window_hours: 1,
        };
        
        let circuit_id1 = service.calculate_circuit_id(&submission);
        let circuit_id2 = service.calculate_circuit_id(&submission);
        
        // 相同输入应该产生相同的电路ID
        assert_eq!(circuit_id1, circuit_id2);
    }
    
    #[test]
    fn test_cache_key_generation() {
        let service = ZKProofService::new().unwrap();
        
        let submission1 = MetricSubmission {
            metric_type: "block_time".to_string(),
            private_data: vec![6000],
            data_sources: vec![
                DataSource {
                    source_type: DataSourceType::ValidatorNode,
                    source_id: "test".to_string(),
                    timestamp: 0,
                    reliability_score: 100,
                },
            ],
            public_metric: 6000,
            quality_score: 90,
            time_window_hours: 1,
        };
        
        let submission2 = submission1.clone();
        
        let key1 = service.generate_cache_key(&submission1);
        let key2 = service.generate_cache_key(&submission2);
        
        assert_eq!(key1, key2);
    }
}