#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod analytics {
    use ink::storage::Mapping;
    use ink::prelude::{vec::Vec, string::String};
    
    /// PolyVisor Analytics合约的主要存储结构
    #[ink(storage)]
    pub struct Analytics {
        /// 网络指标存储：指标类型 -> 指标值
        metrics: Mapping<MetricType, MetricValue>,
        /// 零知识证明存储：证明ID -> 证明数据
        proofs: Mapping<u64, ZKProof>,
        /// 用户隐私级别设置：账户ID -> 隐私级别
        privacy_levels: Mapping<AccountId, PrivacyLevel>,
        /// 可信数据节点列表
        trusted_nodes: Vec<AccountId>,
        /// 数据贡献者统计信息
        contributors: Mapping<AccountId, ContributorInfo>,
        /// 合约所有者
        owner: AccountId,
    }
    
    /// 网络指标类型枚举
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum MetricType {
        /// 平均区块时间 (毫秒)
        AverageBlockTime,
        /// 交易量 (每秒交易数)
        TransactionVolume,
        /// 验证者在线率 (百分比)
        ValidatorUptime,
        /// 网络拥堵度 (百分比)
        NetworkCongestion,
        /// 链活跃度评分
        ChainActivity,
        /// Gas使用情况
        GasUsage,
        /// 网络延迟 (毫秒)
        NetworkLatency,
    }
    
    /// 网络指标值结构体
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct MetricValue {
        /// 指标数值
        pub value: u128,
        /// 时间戳
        pub timestamp: u64,
        /// 关联的证明ID
        pub proof_id: u64,
        /// 隐私级别
        pub privacy_level: PrivacyLevel,
        /// 数据质量评分 (0-100)
        pub data_quality_score: u8,
        /// 数据源节点
        pub source_node: AccountId,
    }
    
    /// 隐私保护级别
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum PrivacyLevel {
        /// 最大隐私保护
        Maximum,
        /// 高隐私保护
        High,
        /// 中等隐私保护
        Medium,
        /// 低隐私保护
        Low,
        /// 最小隐私保护
        Minimal,
    }
    
    /// 零知识证明结构体
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct ZKProof {
        /// 证明值 (字节数组)
        pub proof_value: Vec<u8>,
        /// 公开输入
        pub public_inputs: Vec<u128>,
        /// 验证密钥
        pub verification_key: Vec<u8>,
        /// 电路ID
        pub circuit_id: u32,
    }
    
    /// 数据贡献者信息
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct ContributorInfo {
        /// 总贡献次数
        pub total_contributions: u32,
        /// 平均数据质量评分
        pub data_quality_average: u8,
        /// 最后贡献时间
        pub last_contribution: u64,
        /// 声誉评分
        pub reputation_score: u32,
        /// 验证通过的证明数量
        pub verification_count: u32,
    }
    
    /// 网络健康度评分结构体
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct NetworkHealthScore {
        /// 总体健康度评分 (0-100)
        pub overall_score: u32,
        /// 区块时间评分
        pub block_time_score: u32,
        /// 交易量评分
        pub transaction_score: u32,
        /// 验证者评分
        pub validator_score: u32,
        /// 拥堵度评分
        pub congestion_score: u32,
        /// 最后更新时间
        pub last_updated: u64,
        /// 数据新鲜度评分
        pub data_freshness: u8,
    }
    
    /// 合约错误类型
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum AnalyticsError {
        /// 无效的零知识证明
        InvalidProof,
        /// 未授权的节点
        UnauthorizedNode,
        /// 无效的指标类型
        InvalidMetricType,
        /// 数据源不足
        InsufficientDataSources,
        /// 数据质量过低
        DataQualityTooLow,
        /// 节点未注册
        NodeNotRegistered,
        /// 权限不足
        InsufficientPermission,
    }
    
    /// 合约事件
    #[ink(event)]
    pub struct MetricSubmitted {
        #[ink(topic)]
        pub metric_type: MetricType,
        pub value: u128,
        pub quality_score: u8,
        #[ink(topic)]
        pub contributor: AccountId,
        pub timestamp: u64,
    }
    
    #[ink(event)]
    pub struct PrivacyLevelUpdated {
        #[ink(topic)]
        pub user: AccountId,
        pub new_level: PrivacyLevel,
        pub timestamp: u64,
    }
    
    #[ink(event)]
    pub struct TrustedNodeAdded {
        #[ink(topic)]
        pub node: AccountId,
        pub timestamp: u64,
    }
    
    impl Analytics {
        /// 构造函数：初始化合约
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            Self {
                metrics: Mapping::default(),
                proofs: Mapping::default(),
                privacy_levels: Mapping::default(),
                trusted_nodes: Vec::new(),
                contributors: Mapping::default(),
                owner: caller,
            }
        }
        
        /// 提交网络指标数据（需要零知识证明）
        #[ink(message)]
        pub fn submit_metric(
            &mut self,
            metric_type: MetricType,
            value: u128,
            proof: ZKProof,
            data_quality_score: u8,
        ) -> Result<(), AnalyticsError> {
            let caller = self.env().caller();
            
            // 验证提交者是否为可信节点
            if !self.is_trusted_node(&caller) {
                return Err(AnalyticsError::UnauthorizedNode);
            }
            
            // 验证数据质量评分
            if data_quality_score < 70 {
                return Err(AnalyticsError::DataQualityTooLow);
            }
            
            // 简化的零知识证明验证（实际应用中需要更复杂的验证逻辑）
            if !self.verify_proof(&proof, &metric_type, value) {
                return Err(AnalyticsError::InvalidProof);
            }
            
            // 获取用户隐私级别设置
            let privacy_level = self.privacy_levels.get(&caller)
                .unwrap_or(PrivacyLevel::High);
            
            // 存储证明
            let proof_id = self.env().block_timestamp();
            self.proofs.insert(proof_id, &proof);
            
            // 创建指标值
            let metric_value = MetricValue {
                value,
                timestamp: self.env().block_timestamp(),
                proof_id,
                privacy_level,
                data_quality_score,
                source_node: caller,
            };
            
            // 存储指标数据
            self.metrics.insert(metric_type.clone(), &metric_value);
            
            // 更新贡献者统计信息
            self.update_contributor_info(caller, data_quality_score);
            
            // 发出事件
            self.env().emit_event(MetricSubmitted {
                metric_type,
                value,
                quality_score: data_quality_score,
                contributor: caller,
                timestamp: self.env().block_timestamp(),
            });
            
            Ok(())
        }
        
        /// 获取网络指标（根据用户隐私级别过滤）
        #[ink(message)]
        pub fn get_metric(
            &self,
            metric_type: MetricType,
        ) -> Option<MetricValue> {
            let caller = self.env().caller();
            let user_privacy = self.privacy_levels.get(&caller)
                .unwrap_or(PrivacyLevel::High);
            
            if let Some(metric) = self.metrics.get(&metric_type) {
                // 根据用户隐私级别返回不同粒度的数据
                Some(self.apply_privacy_filter(metric, user_privacy))
            } else {
                None
            }
        }
        
        /// 获取网络健康度评分
        #[ink(message)]
        pub fn get_network_health_score(&self) -> NetworkHealthScore {
            let block_time = self.metrics.get(&MetricType::AverageBlockTime);
            let tx_volume = self.metrics.get(&MetricType::TransactionVolume);
            let validator_uptime = self.metrics.get(&MetricType::ValidatorUptime);
            let congestion = self.metrics.get(&MetricType::NetworkCongestion);
            
            // 计算各项评分
            let block_time_score = self.calculate_block_time_score(&block_time);
            let transaction_score = self.calculate_transaction_score(&tx_volume);
            let validator_score = self.calculate_validator_score(&validator_uptime);
            let congestion_score = self.calculate_congestion_score(&congestion);
            
            // 计算总体评分
            let overall_score = (block_time_score + transaction_score + validator_score + congestion_score) / 4;
            
            NetworkHealthScore {
                overall_score,
                block_time_score,
                transaction_score,
                validator_score,
                congestion_score,
                last_updated: self.env().block_timestamp(),
                data_freshness: self.calculate_data_freshness(),
            }
        }
        
        /// 设置用户隐私级别
        #[ink(message)]
        pub fn set_privacy_level(
            &mut self,
            level: PrivacyLevel,
        ) {
            let caller = self.env().caller();
            self.privacy_levels.insert(caller, &level);
            
            self.env().emit_event(PrivacyLevelUpdated {
                user: caller,
                new_level: level,
                timestamp: self.env().block_timestamp(),
            });
        }
        
        /// 注册可信数据节点（仅合约所有者）
        #[ink(message)]
        pub fn add_trusted_node(
            &mut self,
            node: AccountId,
        ) -> Result<(), AnalyticsError> {
            let caller = self.env().caller();
            
            // 只有合约所有者可以添加可信节点
            if caller != self.owner {
                return Err(AnalyticsError::InsufficientPermission);
            }
            
            if !self.trusted_nodes.contains(&node) {
                self.trusted_nodes.push(node);
                
                self.env().emit_event(TrustedNodeAdded {
                    node,
                    timestamp: self.env().block_timestamp(),
                });
            }
            
            Ok(())
        }
        
        /// 获取贡献者统计信息
        #[ink(message)]
        pub fn get_contributor_stats(
            &self,
            contributor: AccountId,
        ) -> Option<ContributorInfo> {
            self.contributors.get(&contributor)
        }
        
        /// 验证零知识证明（简化实现）
        #[ink(message)]
        pub fn verify_proof_public(
            &self,
            proof_id: u64,
        ) -> bool {
            if let Some(proof) = self.proofs.get(&proof_id) {
                // 简化验证：检查证明格式是否正确
                !proof.proof_value.is_empty() && !proof.public_inputs.is_empty()
            } else {
                false
            }
        }
        
        /// 获取可信节点列表（仅限可信节点查看）
        #[ink(message)]
        pub fn get_trusted_nodes(&self) -> Result<Vec<AccountId>, AnalyticsError> {
            let caller = self.env().caller();
            
            if !self.is_trusted_node(&caller) && caller != self.owner {
                return Err(AnalyticsError::InsufficientPermission);
            }
            
            Ok(self.trusted_nodes.clone())
        }
        
        // 私有辅助方法
        
        /// 检查是否为可信节点
        fn is_trusted_node(&self, node: &AccountId) -> bool {
            self.trusted_nodes.contains(node)
        }
        
        /// 验证零知识证明（简化实现）
        fn verify_proof(
            &self,
            proof: &ZKProof,
            metric_type: &MetricType,
            value: u128,
        ) -> bool {
            // 基本格式检查
            if proof.proof_value.is_empty() || proof.public_inputs.is_empty() {
                return false;
            }
            
            // 验证公开输入是否匹配
            if proof.public_inputs.len() > 0 && proof.public_inputs[0] != value {
                return false;
            }
            
            // 简化验证：在实际应用中，这里应该进行复杂的零知识证明验证
            // 包括椭圆曲线运算、配对检查等
            true
        }
        
        /// 更新贡献者信息
        fn update_contributor_info(&mut self, contributor: AccountId, quality_score: u8) {
            if let Some(mut info) = self.contributors.get(&contributor) {
                info.total_contributions += 1;
                info.data_quality_average = 
                    ((info.data_quality_average as u32 * (info.total_contributions - 1) as u32) 
                     + quality_score as u32) / info.total_contributions as u32;
                info.last_contribution = self.env().block_timestamp();
                info.reputation_score += quality_score as u32;
                self.contributors.insert(contributor, &info);
            } else {
                let new_info = ContributorInfo {
                    total_contributions: 1,
                    data_quality_average: quality_score,
                    last_contribution: self.env().block_timestamp(),
                    reputation_score: quality_score as u32,
                    verification_count: 0,
                };
                self.contributors.insert(contributor, &new_info);
            }
        }
        
        /// 应用隐私过滤器
        fn apply_privacy_filter(
            &self,
            metric: MetricValue,
            privacy_level: PrivacyLevel,
        ) -> MetricValue {
            match privacy_level {
                PrivacyLevel::Maximum => {
                    // 最大隐私：返回高度模糊化的值
                    MetricValue {
                        value: (metric.value / 1000) * 1000, // 取整到千位
                        data_quality_score: 0, // 不显示质量评分
                        source_node: AccountId::from([0u8; 32]), // 匿名化源节点
                        ..metric
                    }
                },
                PrivacyLevel::High => {
                    // 高隐私：返回模糊化的值
                    MetricValue {
                        value: (metric.value / 100) * 100, // 取整到百位
                        data_quality_score: (metric.data_quality_score / 10) * 10,
                        source_node: AccountId::from([0u8; 32]),
                        ..metric
                    }
                },
                PrivacyLevel::Medium => {
                    // 中隐私：返回较精确的值
                    MetricValue {
                        value: (metric.value / 10) * 10, // 取整到十位
                        source_node: AccountId::from([0u8; 32]),
                        ..metric
                    }
                },
                PrivacyLevel::Low => {
                    // 低隐私：返回精确值但隐藏源节点
                    MetricValue {
                        source_node: AccountId::from([0u8; 32]),
                        ..metric
                    }
                },
                PrivacyLevel::Minimal => metric, // 最小隐私：返回原始值
            }
        }
        
        /// 计算区块时间评分
        fn calculate_block_time_score(&self, block_time: &Option<MetricValue>) -> u32 {
            if let Some(bt) = block_time {
                let target_block_time = 6000u128; // 目标：6秒
                let deviation = if bt.value > target_block_time {
                    bt.value - target_block_time
                } else {
                    target_block_time - bt.value
                };
                let score = ((1000u128.saturating_sub(deviation.min(1000))) * 100 / 1000) as u32;
                score
            } else {
                0
            }
        }
        
        /// 计算交易量评分
        fn calculate_transaction_score(&self, tx_volume: &Option<MetricValue>) -> u32 {
            if let Some(tv) = tx_volume {
                // 交易量越高，网络活跃度越高（最高100分）
                (tv.value / 100).min(100) as u32
            } else {
                0
            }
        }
        
        /// 计算验证者评分
        fn calculate_validator_score(&self, validator_uptime: &Option<MetricValue>) -> u32 {
            if let Some(vu) = validator_uptime {
                // 验证者在线率越高越好（已经是百分比）
                vu.value.min(100) as u32
            } else {
                0
            }
        }
        
        /// 计算拥堵度评分
        fn calculate_congestion_score(&self, congestion: &Option<MetricValue>) -> u32 {
            if let Some(nc) = congestion {
                // 网络拥堵度越低越好
                (100u128.saturating_sub(nc.value.min(100))) as u32
            } else {
                0
            }
        }
        
        /// 计算数据新鲜度
        fn calculate_data_freshness(&self) -> u8 {
            let current_time = self.env().block_timestamp();
            let mut total_freshness = 0u32;
            let mut count = 0u32;
            
            // 检查各个指标的最后更新时间
            let metric_types = [
                MetricType::AverageBlockTime,
                MetricType::TransactionVolume,
                MetricType::ValidatorUptime,
                MetricType::NetworkCongestion,
            ];
            
            for metric_type in metric_types.iter() {
                if let Some(metric) = self.metrics.get(metric_type) {
                    let age = current_time.saturating_sub(metric.timestamp);
                    // 数据越新鲜，评分越高（以秒为单位）
                    let freshness = if age < 300 { // 5分钟内
                        100
                    } else if age < 600 { // 10分钟内
                        80
                    } else if age < 1800 { // 30分钟内
                        60
                    } else if age < 3600 { // 1小时内
                        40
                    } else {
                        20
                    };
                    total_freshness += freshness;
                    count += 1;
                }
            }
            
            if count > 0 {
                (total_freshness / count) as u8
            } else {
                0
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn test_contract_creation() {
            let analytics = Analytics::new();
            // 验证合约初始化
            assert_eq!(analytics.trusted_nodes.len(), 0);
        }

        #[ink::test]
        fn test_privacy_level_setting() {
            let mut analytics = Analytics::new();
            
            // 设置隐私级别
            analytics.set_privacy_level(PrivacyLevel::Medium);
            
            // 由于无法直接访问私有字段，这里只验证函数执行成功
            // 在实际测试中需要添加获取隐私级别的公开方法
        }

        #[ink::test]
        fn test_add_trusted_node() {
            let mut analytics = Analytics::new();
            let test_node = AccountId::from([1u8; 32]);
            
            // 添加可信节点（作为所有者）
            let result = analytics.add_trusted_node(test_node);
            assert!(result.is_ok());
            assert_eq!(analytics.trusted_nodes.len(), 1);
        }

        #[ink::test]
        fn test_proof_verification() {
            let analytics = Analytics::new();
            
            let proof = ZKProof {
                proof_value: vec![1, 2, 3, 4],
                public_inputs: vec![1000],
                verification_key: vec![5, 6, 7, 8],
                circuit_id: 0,
            };
            
            // 测试简化的证明验证
            let is_valid = analytics.verify_proof(&proof, &MetricType::AverageBlockTime, 1000);
            assert!(is_valid);
        }
    }
}