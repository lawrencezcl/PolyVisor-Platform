// 工具函数模块
use anyhow::Result;
use sha2::{Sha256, Digest};

/// 计算数据哈希
pub fn hash_data(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

/// 序列化公共输入
pub fn serialize_public_inputs(inputs: &[u128]) -> Vec<u8> {
    let mut result = Vec::new();
    for input in inputs {
        result.extend_from_slice(&input.to_be_bytes());
    }
    result
}

/// 反序列化公共输入
pub fn deserialize_public_inputs(data: &[u8]) -> Result<Vec<u128>> {
    if data.len() % 16 != 0 {
        return Err(anyhow::anyhow!("Invalid public inputs data length"));
    }

    let mut inputs = Vec::new();
    for chunk in data.chunks(16) {
        let mut bytes = [0u8; 16];
        bytes.copy_from_slice(chunk);
        inputs.push(u128::from_be_bytes(bytes));
    }

    Ok(inputs)
}

/// 验证数据完整性
pub fn verify_data_integrity(data: &[u8], expected_hash: &[u8; 32]) -> bool {
    let actual_hash = hash_data(data);
    &actual_hash == expected_hash
}

/// 生成随机nonce
pub fn generate_nonce() -> [u8; 32] {
    use rand::RngCore;
    let mut nonce = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut nonce);
    nonce
}

/// 格式化证明大小
pub fn format_proof_size(size: usize) -> String {
    if size < 1024 {
        format!("{} bytes", size)
    } else if size < 1024 * 1024 {
        format!("{:.1} KB", size as f64 / 1024.0)
    } else {
        format!("{:.1} MB", size as f64 / (1024.0 * 1024.0))
    }
}

/// 计算质量评分
pub fn calculate_quality_score(
    data_freshness: u8,     // 数据新鲜度 (0-100)
    source_reliability: u8, // 数据源可靠性 (0-100)
    consensus_level: u8,    // 共识程度 (0-100)
) -> u8 {
    // 加权平均计算质量评分
    let weighted_score = (data_freshness as f64 * 0.3) +
                        (source_reliability as f64 * 0.4) +
                        (consensus_level as f64 * 0.3);
    
    weighted_score.round() as u8
}

/// 时间戳工具
pub mod time {
    use chrono::{DateTime, Utc};

    /// 获取当前Unix时间戳
    pub fn current_timestamp() -> u64 {
        Utc::now().timestamp() as u64
    }

    /// 时间戳转换为可读格式
    pub fn timestamp_to_string(timestamp: u64) -> String {
        DateTime::from_timestamp(timestamp as i64, 0)
            .unwrap_or_else(Utc::now)
            .format("%Y-%m-%d %H:%M:%S UTC")
            .to_string()
    }

    /// 计算时间差（秒）
    pub fn time_diff_seconds(start: u64, end: u64) -> u64 {
        end.saturating_sub(start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_data() {
        let data = b"test data";
        let hash1 = hash_data(data);
        let hash2 = hash_data(data);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_serialize_deserialize_public_inputs() {
        let inputs = vec![100u128, 200u128, 300u128];
        let serialized = serialize_public_inputs(&inputs);
        let deserialized = deserialize_public_inputs(&serialized).unwrap();
        assert_eq!(inputs, deserialized);
    }

    #[test]
    fn test_quality_score_calculation() {
        let score = calculate_quality_score(90, 85, 95);
        assert!(score > 0 && score <= 100);
    }
}