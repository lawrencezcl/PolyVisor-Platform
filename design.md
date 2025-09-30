# PolyVisor 系统设计文档

## 1. 系统架构概述

### 1.1 整体架构

```
┌─────────────────────────────────────────────────────────────────┐
│                      前端层 (Frontend)                        │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │   React Web     │  │   Mobile App    │  │   Dashboard     │ │
│  │   Application   │  │   (Optional)    │  │   Admin         │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                      API 网关层                              │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │   GraphQL API   │  │   REST API      │  │   WebSocket     │ │
│  │   (查询优化)     │  │   (标准接口)     │  │   (实时数据)     │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    业务逻辑层 (Services)                      │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │   Analytics     │  │   Privacy       │  │   Data          │ │
│  │   Service       │  │   Service       │  │   Collection    │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │   ZK Proof      │  │   Visualization │  │   Node          │ │
│  │   Service       │  │   Service       │  │   Management    │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                  区块链层 (Blockchain)                       │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │   Substrate     │  │   ink! Smart    │  │   Runtime       │ │
│  │   Runtime       │  │   Contracts     │  │   Pallets       │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    存储层 (Storage)                          │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │   On-Chain      │  │   IPFS          │  │   Cache         │ │
│  │   Storage       │  │   (聚合数据)     │  │   (Redis)       │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

### 1.2 技术栈选择

**区块链层:**
- **Substrate Framework**: 基础区块链框架
- **ink! Smart Contracts**: Rust智能合约
- **Polkadot.js API**: 区块链交互

**后端服务:**
- **Rust**: 高性能后端服务
- **Node.js/TypeScript**: API网关和中间件
- **GraphQL**: 灵活的查询接口
- **WebSocket**: 实时数据推送

**前端:**
- **React 18**: 用户界面框架
- **TypeScript**: 类型安全
- **Tailwind CSS**: 样式框架
- **D3.js**: 数据可视化
- **Polkadot.js Extension**: 钱包集成

**存储:**
- **IPFS**: 分布式文件存储
- **Redis**: 缓存层
- **PostgreSQL**: 元数据存储

## 2. 核心模块设计

### 2.1 Analytics Module (分析模块)

```rust
// analytics/src/lib.rs
#[ink::contract]
mod analytics {
    use ink::storage::Mapping;
    use ink::prelude::{vec::Vec, string::String};
    
    #[ink(storage)]
    pub struct Analytics {
        /// 网络指标存储
        metrics: Mapping<MetricType, MetricValue>,
        /// 零知识证明存储
        proofs: Mapping<u64, ZKProof>,
        /// 隐私级别配置
        privacy_levels: Mapping<AccountId, PrivacyLevel>,
        /// 数据贡献者记录
        contributors: Mapping<AccountId, ContributorInfo>,
        /// 验证者白名单
        trusted_nodes: Vec<AccountId>,
    }
    
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum MetricType {
        AverageBlockTime,
        TransactionVolume,
        ValidatorUptime,
        NetworkCongestion,
        ChainActivity,
        GasUsage,
        NetworkLatency,
    }
    
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct MetricValue {
        value: u128,
        timestamp: u64,
        proof_id: u64,
        privacy_level: PrivacyLevel,
        data_quality_score: u8, // 0-100 数据质量评分
        source_node: AccountId,
    }
    
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum PrivacyLevel {
        Maximum,  // 最大隐私保护，只显示高度聚合的数据
        High,     // 高隐私保护，显示区间数据
        Medium,   // 中等隐私保护，显示近似值
        Low,      // 低隐私保护，显示较详细的数据
        Minimal,  // 最小隐私保护，显示详细数据
    }
    
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct ContributorInfo {
        total_contributions: u32,
        data_quality_average: u8,
        last_contribution: u64,
        reputation_score: u32,
        verification_count: u32,
    }
    
    impl Analytics {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                metrics: Mapping::default(),
                proofs: Mapping::default(),
                privacy_levels: Mapping::default(),
                contributors: Mapping::default(),
                trusted_nodes: Vec::new(),
            }
        }
        
        /// 提交网络指标（需要零知识证明）
        #[ink(message)]
        pub fn submit_metric(
            &mut self,
            metric_type: MetricType,
            value: u128,
            proof: ZKProof,
            data_sources: Vec<DataSource>, // 数据来源证明
        ) -> Result<(), AnalyticsError> {
            let caller = self.env().caller();
            
            // 验证提交者是否为可信节点
            if !self.is_trusted_node(&caller) {
                return Err(AnalyticsError::UnauthorizedNode);
            }
            
            // 验证零知识证明
            if !self.verify_proof(&proof, &metric_type, value, &data_sources) {
                return Err(AnalyticsError::InvalidProof);
            }
            
            // 计算数据质量评分
            let quality_score = self.calculate_data_quality(&data_sources, &proof);
            
            let privacy_level = self.privacy_levels.get(&caller)
                .unwrap_or(PrivacyLevel::High);
            
            let metric_value = MetricValue {
                value,
                timestamp: self.env().block_timestamp(),
                proof_id: self.store_proof(proof),
                privacy_level,
                data_quality_score: quality_score,
                source_node: caller,
            };
            
            // 存储指标数据
            self.metrics.insert(metric_type, &metric_value);
            
            // 更新贡献者信息
            self.update_contributor_info(caller, quality_score);
            
            // 触发数据更新事件
            self.env().emit_event(MetricSubmitted {
                metric_type: metric_type.clone(),
                value,
                quality_score,
                contributor: caller,
                timestamp: self.env().block_timestamp(),
            });
            
            Ok(())
        }
        
        /// 批量提交多个指标
        #[ink(message)]
        pub fn submit_metrics_batch(
            &mut self,
            metrics: Vec<MetricSubmission>,
        ) -> Result<Vec<bool>, AnalyticsError> {
            let caller = self.env().caller();
            
            if !self.is_trusted_node(&caller) {
                return Err(AnalyticsError::UnauthorizedNode);
            }
            
            let mut results = Vec::new();
            
            for metric in metrics {
                let result = self.submit_metric(
                    metric.metric_type,
                    metric.value,
                    metric.proof,
                    metric.data_sources,
                );
                results.push(result.is_ok());
            }
            
            Ok(results)
        }
        
        /// 获取网络指标
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
        
        /// 获取历史指标数据
        #[ink(message)]
        pub fn get_historical_metrics(
            &self,
            metric_type: MetricType,
            from_timestamp: u64,
            to_timestamp: u64,
            privacy_level: PrivacyLevel,
        ) -> Vec<MetricValue> {
            // 这里需要结合链下存储（IPFS）来获取历史数据
            // 简化实现，返回当前指标
            if let Some(metric) = self.metrics.get(&metric_type) {
                if metric.timestamp >= from_timestamp && metric.timestamp <= to_timestamp {
                    vec![self.apply_privacy_filter(metric, privacy_level)]
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            }
        }
        
        /// 获取网络健康度评分
        #[ink(message)]
        pub fn get_network_health_score(&self) -> NetworkHealthScore {
            let block_time = self.metrics.get(&MetricType::AverageBlockTime);
            let tx_volume = self.metrics.get(&MetricType::TransactionVolume);
            let validator_uptime = self.metrics.get(&MetricType::ValidatorUptime);
            let congestion = self.metrics.get(&MetricType::NetworkCongestion);
            
            // 计算综合健康度评分
            let mut total_score = 0u32;
            let mut count = 0u32;
            
            if let Some(bt) = block_time {
                // 区块时间越接近目标值（6秒），评分越高
                let target_block_time = 6000u128; // 6秒，以毫秒为单位
                let deviation = if bt.value > target_block_time {
                    bt.value - target_block_time
                } else {
                    target_block_time - bt.value
                };
                let score = ((1000 - deviation.min(1000)) * 100 / 1000) as u32;
                total_score += score;
                count += 1;
            }
            
            if let Some(tv) = tx_volume {
                // 交易量越高，网络活跃度越高
                let score = (tv.value / 100).min(100) as u32; // 简化评分逻辑
                total_score += score;
                count += 1;
            }
            
            if let Some(vu) = validator_uptime {
                // 验证者在线率越高越好
                let score = vu.value.min(10000) as u32 / 100; // 转换为百分比
                total_score += score;
                count += 1;
            }
            
            if let Some(nc) = congestion {
                // 网络拥堵度越低越好
                let score = (100 - nc.value.min(100)) as u32;
                total_score += score;
                count += 1;
            }
            
            let average_score = if count > 0 { total_score / count } else { 0 };
            
            NetworkHealthScore {
                overall_score: average_score,
                block_time_score: block_time.map(|_| total_score / count.max(1)).unwrap_or(0),
                transaction_score: tx_volume.map(|_| total_score / count.max(1)).unwrap_or(0),
                validator_score: validator_uptime.map(|_| total_score / count.max(1)).unwrap_or(0),
                congestion_score: congestion.map(|_| total_score / count.max(1)).unwrap_or(0),
                last_updated: self.env().block_timestamp(),
                data_freshness: self.calculate_data_freshness(),
            }
        }
        
        /// 注册可信数据节点
        #[ink(message)]
        pub fn register_trusted_node(
            &mut self,
            node: AccountId,
        ) -> Result<(), AnalyticsError> {
            // 只有合约所有者可以注册可信节点
            let caller = self.env().caller();
            // 这里应该检查caller是否为合约管理员
            
            if !self.trusted_nodes.contains(&node) {
                self.trusted_nodes.push(node);
            }
            
            Ok(())
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
        
        /// 获取贡献者统计信息
        #[ink(message)]
        pub fn get_contributor_stats(
            &self,
            contributor: AccountId,
        ) -> Option<ContributorInfo> {
            self.contributors.get(&contributor)
        }
        
        // 私有方法
        
        /// 验证零知识证明
        fn verify_proof(
            &self,
            proof: &ZKProof,
            metric_type: &MetricType,
            value: u128,
            data_sources: &[DataSource],
        ) -> bool {
            // 实现零知识证明验证逻辑
            // 验证数据来源的真实性和完整性
            // 验证聚合计算的正确性
            
            // 基本验证：检查证明格式
            if proof.proof_value.is_empty() || proof.public_inputs.is_empty() {
                return false;
            }
            
            // 验证公开输入是否匹配
            if proof.public_inputs.len() > 0 && proof.public_inputs[0] != value {
                return false;
            }
            
            // 验证数据源
            if data_sources.is_empty() {
                return false;
            }
            
            // 这里应该集成具体的ZK证明库进行验证
            // 暂时返回true作为简化实现
            true
        }
        
        /// 存储证明
        fn store_proof(&mut self, proof: ZKProof) -> u64 {
            let proof_id = self.env().block_timestamp();
            self.proofs.insert(proof_id, &proof);
            proof_id
        }
        
        /// 计算数据质量评分
        fn calculate_data_quality(
            &self,
            data_sources: &[DataSource],
            proof: &ZKProof,
        ) -> u8 {
            let mut score = 100u8;
            
            // 数据源越多，质量评分越高
            if data_sources.len() < 3 {
                score = score.saturating_sub(20);
            }
            
            // 检查数据源的多样性
            let unique_types: std::collections::HashSet<_> = 
                data_sources.iter().map(|ds| &ds.source_type).collect();
            if unique_types.len() < data_sources.len() / 2 {
                score = score.saturating_sub(15);
            }
            
            // 证明复杂度评分
            if proof.verification_key.len() < 100 {
                score = score.saturating_sub(10);
            }
            
            score
        }
        
        /// 检查是否为可信节点
        fn is_trusted_node(&self, node: &AccountId) -> bool {
            self.trusted_nodes.contains(node)
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
                        data_quality_score: metric.data_quality_score,
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
                    // 数据越新鲜，评分越高（以小时为单位）
                    let freshness = if age < 3600000 { // 1小时内
                        100
                    } else if age < 7200000 { // 2小时内
                        80
                    } else if age < 21600000 { // 6小时内
                        60
                    } else if age < 43200000 { // 12小时内
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
    
    // 数据结构定义
    
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct ZKProof {
        pub proof_value: Vec<u8>,
        pub public_inputs: Vec<u128>,
        pub verification_key: Vec<u8>,
        pub circuit_id: u32, // 电路标识符
    }
    
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct DataSource {
        pub source_type: DataSourceType,
        pub source_id: Vec<u8>,
        pub timestamp: u64,
        pub reliability_score: u8,
    }
    
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum DataSourceType {
        ValidatorNode,
        FullNode,
        LightNode,
        Parachain,
        RelayChain,
        ExternalOracle,
    }
    
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct MetricSubmission {
        pub metric_type: MetricType,
        pub value: u128,
        pub proof: ZKProof,
        pub data_sources: Vec<DataSource>,
    }
    
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct NetworkHealthScore {
        pub overall_score: u32,
        pub block_time_score: u32,
        pub transaction_score: u32,
        pub validator_score: u32,
        pub congestion_score: u32,
        pub last_updated: u64,
        pub data_freshness: u8,
    }
    
    // 事件定义
    
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
    
    // 错误定义
    
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum AnalyticsError {
        InvalidProof,
        UnauthorizedNode,
        InvalidMetricType,
        InsufficientDataSources,
        DataQualityTooLow,
        NodeNotRegistered,
    }
}
```

### 2.2 ZK Proof Module (零知识证明模块)

```rust
// zkproof/src/lib.rs  
/// 简化的零知识证明服务
pub struct ZKProofService;

impl ZKProofService {
    pub fn new() -> Self { Self }
    
    /// 生成网络指标的零知识证明
    pub fn generate_proof(&self, private_data: Vec<u128>, public_metric: u128) -> Result<ZKProof, String> {
        // 简化实现：生成模拟证明
        Ok(ZKProof {
            proof_value: vec![1, 2, 3, 4],
            public_inputs: vec![public_metric],
            verification_key: vec![5, 6, 7, 8],
            circuit_id: 0,
        })
    }
    
    /// 验证零知识证明
    pub fn verify_proof(&self, proof: &ZKProof) -> Result<bool, String> {
        Ok(!proof.proof_value.is_empty() && !proof.public_inputs.is_empty())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZKProof {
    pub proof_value: Vec<u8>,
    pub public_inputs: Vec<u128>,
    pub verification_key: Vec<u8>,
    pub circuit_id: u32,
}
```

## 3. API接口设计

### 3.1 RESTful API 核心接口

```
// 主要API端点

// 网络指标相关
GET    /api/v1/metrics                     // 获取网络指标列表
POST   /api/v1/metrics                     // 提交网络指标
GET    /api/v1/metrics/{type}              // 获取特定类型指标
GET    /api/v1/metrics/{type}/history      // 获取历史数据

// 网络健康度
GET    /api/v1/network/health              // 获取网络健康度评分
GET    /api/v1/network/status              // 获取网络状态概览

// 零知识证明
GET    /api/v1/proofs/{id}                 // 获取证明详情
POST   /api/v1/proofs/verify               // 验证证明
POST   /api/v1/proofs/batch-verify         // 批量验证证明

// 隐私设置
GET    /api/v1/privacy/settings            // 获取用户隐私设置
PUT    /api/v1/privacy/settings            // 更新隐私设置

// 数据贡献
POST   /api/v1/contributions/submit        // 提交数据贡献
GET    /api/v1/contributions/stats         // 获取贡献统计
```

### 3.2 GraphQL Schema

```
type Metric {
  id: ID!
  type: MetricType!
  value: Float!
  timestamp: DateTime!
  privacyLevel: PrivacyLevel!
  proofVerified: Boolean!
  qualityScore: Int!
}

type NetworkHealth {
  overallScore: Float!
  blockTimeScore: Float!
  transactionScore: Float!
  validatorScore: Float!
  congestionScore: Float!
  lastUpdated: DateTime!
}

enum MetricType {
  BLOCK_TIME
  TRANSACTION_VOLUME
  VALIDATOR_UPTIME
  NETWORK_CONGESTION
  CHAIN_ACTIVITY
}

enum PrivacyLevel {
  MAXIMUM
  HIGH
  MEDIUM
  LOW
  MINIMAL
}

type Query {
  metrics(filter: MetricFilter, limit: Int = 50): [Metric!]!
  networkHealth: NetworkHealth!
  proof(id: ID!): ZKProof
}

type Mutation {
  submitMetric(input: MetricInput!): Metric!
  verifyProof(proofId: ID!): VerificationResult!
  updatePrivacySettings(settings: PrivacySettingsInput!): PrivacySettings!
}

type Subscription {
  metricUpdated(types: [MetricType!]): Metric!
  networkHealthUpdated: NetworkHealth!
}
```

## 4. 数据存储设计

### 4.1 链上存储结构

```
// 链上存储的核心数据结构
struct OnChainStorage {
    // 网络指标聚合数据
    aggregated_metrics: StorageMap<MetricType, AggregatedMetric>,
    
    // 零知识证明哈希（实际证明存储在IPFS）
    proof_hashes: StorageMap<ProofId, ProofHash>,
    
    // 用户隐私设置
    user_privacy_settings: StorageMap<AccountId, PrivacyConfig>,
    
    // 可信数据节点列表
    trusted_nodes: StorageMap<AccountId, NodeInfo>,
    
    // 数据贡献者信誉记录
    contributor_reputation: StorageMap<AccountId, ReputationInfo>,
}
```

### 4.2 IPFS存储结构

```
{
  "version": "1.0",
  "data_type": "network_metrics",
  "aggregation_period": "1h",
  "privacy_level": "high",
  "metrics": {
    "block_time": {
      "aggregated_value": 6125,
      "quality_score": 92,
      "source_count": 4
    }
  },
  "proof_data": {
    "circuit_id": 1,
    "proof_bytes": "0x...",
    "public_inputs": [6125, 92, 1]
  },
  "metadata": {
    "timestamp": "2024-01-01T12:00:00Z",
    "contributor_nodes": ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]
  }
}
```

## 5. 前端架构设计

### 5.1 React组件层次结构

```
App
├── Layout
│   ├── Header (钱包连接、用户设置)
│   ├── Sidebar (导航菜单)
│   └── Footer
├── Pages
│   ├── Dashboard (主仪表板)
│   ├── Analytics (详细分析)
│   ├── Privacy (隐私设置)
│   ├── Verification (证明验证)
│   └── DataContribution (数据贡献)
└── Components
    ├── Charts (图表组件)
    ├── Metrics (指标显示)
    ├── Privacy (隐私控制)
    └── Common (通用组件)
```

### 5.2 状态管理设计

```typescript
// Redux Store 结构
interface AppState {
  auth: {
    isConnected: boolean;
    account: Account | null;
    networkId: string;
  };
  
  analytics: {
    metrics: MetricData[];
    networkHealth: NetworkHealth;
    loading: boolean;
    error: string | null;
  };
  
  privacy: {
    settings: PrivacySettings;
    level: PrivacyLevel;
  };
  
  ui: {
    theme: 'light' | 'dark';
    sidebarOpen: boolean;
    notifications: Notification[];
  };
}
```

## 6. 部署架构

### 6.1 容器化部署

```
# docker-compose.yml
version: '3.8'
services:
  blockchain:
    build: ./blockchain
    ports:
      - "9944:9944"
    volumes:
      - blockchain_data:/data
  
  backend:
    build: ./backend
    ports:
      - "8080:8080"
    depends_on:
      - redis
      - postgres
    environment:
      - DATABASE_URL=postgresql://user:password@postgres:5432/polyvisor
      - REDIS_URL=redis://redis:6379
  
  frontend:
    build: ./frontend
    ports:
      - "3000:3000"
    depends_on:
      - backend
  
  redis:
    image: redis:alpine
    ports:
      - "6379:6379"
  
  postgres:
    image: postgres:13
    environment:
      - POSTGRES_DB=polyvisor
      - POSTGRES_USER=user
      - POSTGRES_PASSWORD=password
    volumes:
      - postgres_data:/var/lib/postgresql/data

volumes:
  blockchain_data:
  postgres_data:
```

## 7. 开发计划

### 阶段一 (2周): 基础架构
- Substrate区块链搭建
- 核心智能合约开发  
- 基础前端界面

### 阶段二 (2周): 核心功能
- 零知识证明集成
- 隐私保护功能
- 数据可视化

### 阶段三 (1周): 完善优化
- 系统集成测试
- 性能优化
- 部署和文档

## 8. 技术难点和解决方案

### 8.1 零知识证明性能优化
- 预编译电路减少证明生成时间
- 批量验证多个证明提高效率
- 缓存常用证明参数

### 8.2 实时数据处理
- WebSocket保持长连接
- Redis缓存热点数据
- 数据库读写分离

### 8.3 隐私与透明度平衡
- 多级隐私设置
- 用户自主选择数据共享程度
- 透明的算法和证明过程

## 9. 总结

PolyVisor通过创新的零知识证明技术，在保护用户隐私的前提下提供丰富的网络分析功能。该设计文档详细规划了系统的各个层面，从底层区块链到用户界面，确保项目能够成功实现PRD中的所有目标。

### 核心创新点：
1. **隐私优先设计**: 零知识证明确保数据隐私
2. **链上验证机制**: 所有指标都可在链上验证真实性
3. **实时数据处理**: 提供实时网络健康度和性能指标
4. **用户友好界面**: 直观的数据可视化界面
5. **可扩展架构**: 模块化架构支持功能扩展

这个设计为Polkadot生态系统提供了一个隐私友好的网络分析解决方案，为用户和开发者提供有价值的网络洞察，同时保护他们的隐私。