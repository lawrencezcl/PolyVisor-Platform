use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 电路类型枚举
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum CircuitType {
    /// 网络指标电路
    NetworkMetric,
    /// 数据完整性电路
    DataIntegrity,
    /// 聚合证明电路
    AggregationProof,
    /// 隐私保护电路
    PrivacyPreserving,
}

/// 简化的网络指标电路
/// 在实际应用中，这里应该使用bellman库实现复杂的零知识证明电路
#[derive(Debug, Clone)]
pub struct NetworkMetricCircuit {
    /// 电路ID
    pub circuit_id: u32,
    /// 电路类型
    pub circuit_type: CircuitType,
    /// 支持的最大数据点数量
    pub max_data_points: usize,
    /// 支持的最大数据源数量
    pub max_data_sources: usize,
    /// 电路描述
    pub description: String,
}

impl NetworkMetricCircuit {
    /// 创建新的网络指标电路
    pub fn new(
        circuit_id: u32,
        max_data_points: usize,
        max_data_sources: usize,
        description: String,
    ) -> Self {
        Self {
            circuit_id,
            circuit_type: CircuitType::NetworkMetric,
            max_data_points,
            max_data_sources,
            description,
        }
    }
    
    /// 验证电路约束
    pub fn verify_constraints(
        &self,
        private_data: &[u128],
        data_sources: &[u32],
        public_metric: u128,
        quality_score: u8,
    ) -> bool {
        // 检查数据点数量限制
        if private_data.len() > self.max_data_points {
            return false;
        }
        
        // 检查数据源数量限制
        if data_sources.len() > self.max_data_sources {
            return false;
        }
        
        // 检查数据源数量至少为2
        if data_sources.len() < 2 {
            return false;
        }
        
        // 验证聚合正确性（简化实现）
        if !private_data.is_empty() {
            let calculated_avg = private_data.iter().sum::<u128>() / private_data.len() as u128;
            // 允许5%的误差范围
            let tolerance = public_metric / 20; // 5%
            if calculated_avg.abs_diff(public_metric) > tolerance {
                return false;
            }
        }
        
        // 验证质量评分合理性
        if quality_score > 100 {
            return false;
        }
        
        // 验证数据源可靠性影响质量评分
        let avg_reliability = data_sources.iter().sum::<u32>() / data_sources.len() as u32;
        if quality_score as u32 > avg_reliability + 10 { // 质量评分不应显著超过数据源可靠性
            return false;
        }
        
        true
    }
    
    /// 估算证明生成的计算复杂度
    pub fn estimate_complexity(&self, data_points: usize, sources: usize) -> CircuitComplexity {
        let constraint_count = self.estimate_constraint_count(data_points, sources);
        let witness_count = self.estimate_witness_count(data_points, sources);
        
        CircuitComplexity {
            constraint_count,
            witness_count,
            estimated_generation_time_ms: constraint_count / 1000 + witness_count / 500,
            estimated_verification_time_ms: constraint_count / 5000 + 10,
            memory_usage_mb: (constraint_count + witness_count) / 10000,
        }
    }
    
    /// 获取电路的公开输入规范
    pub fn get_public_input_spec(&self) -> PublicInputSpec {
        PublicInputSpec {
            inputs: vec![
                InputSpec {
                    name: "aggregated_metric".to_string(),
                    description: "聚合后的网络指标值".to_string(),
                    data_type: InputDataType::U128,
                    constraints: vec![
                        "value > 0".to_string(),
                        "value < 2^64".to_string(),
                    ],
                },
                InputSpec {
                    name: "quality_score".to_string(),
                    description: "数据质量评分 (0-100)".to_string(),
                    data_type: InputDataType::U8,
                    constraints: vec![
                        "value <= 100".to_string(),
                    ],
                },
                InputSpec {
                    name: "time_window".to_string(),
                    description: "时间窗口（小时）".to_string(),
                    data_type: InputDataType::U8,
                    constraints: vec![
                        "value > 0".to_string(),
                        "value <= 24".to_string(),
                    ],
                },
            ],
        }
    }
    
    // 私有辅助方法
    
    /// 估算约束数量
    fn estimate_constraint_count(&self, data_points: usize, sources: usize) -> usize {
        // 基础约束：数据范围检查、聚合计算、质量评分验证
        let base_constraints = 50;
        
        // 每个数据点的约束：范围检查 + 聚合计算参与
        let data_constraints = data_points * 5;
        
        // 每个数据源的约束：可靠性检查 + 多样性验证
        let source_constraints = sources * 3;
        
        // 额外约束：时间一致性、数据完整性等
        let additional_constraints = 20;
        
        base_constraints + data_constraints + source_constraints + additional_constraints
    }
    
    /// 估算见证变量数量
    fn estimate_witness_count(&self, data_points: usize, sources: usize) -> usize {
        // 私有数据见证
        let private_witnesses = data_points;
        
        // 数据源信息见证
        let source_witnesses = sources * 2;
        
        // 中间计算见证：聚合过程、质量计算等
        let intermediate_witnesses = data_points + sources + 10;
        
        private_witnesses + source_witnesses + intermediate_witnesses
    }
}

/// 电路复杂度信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitComplexity {
    /// 约束数量
    pub constraint_count: usize,
    /// 见证变量数量
    pub witness_count: usize,
    /// 估算证明生成时间（毫秒）
    pub estimated_generation_time_ms: usize,
    /// 估算验证时间（毫秒）
    pub estimated_verification_time_ms: usize,
    /// 估算内存使用（MB）
    pub memory_usage_mb: usize,
}

/// 公开输入规范
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicInputSpec {
    /// 输入列表
    pub inputs: Vec<InputSpec>,
}

/// 输入规范
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputSpec {
    /// 输入名称
    pub name: String,
    /// 输入描述
    pub description: String,
    /// 数据类型
    pub data_type: InputDataType,
    /// 约束条件
    pub constraints: Vec<String>,
}

/// 输入数据类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputDataType {
    U8,
    U32,
    U64,
    U128,
    Bytes,
}

/// 电路管理器
pub struct CircuitManager {
    /// 已注册的电路
    circuits: HashMap<u32, NetworkMetricCircuit>,
    /// 电路类型映射
    type_mapping: HashMap<CircuitType, Vec<u32>>,
}

impl CircuitManager {
    /// 创建新的电路管理器
    pub fn new() -> Self {
        let mut manager = Self {
            circuits: HashMap::new(),
            type_mapping: HashMap::new(),
        };
        
        // 注册默认电路
        manager.register_default_circuits();
        
        manager
    }
    
    /// 注册电路
    pub fn register_circuit(&mut self, circuit: NetworkMetricCircuit) {
        let circuit_id = circuit.circuit_id;
        let circuit_type = circuit.circuit_type.clone();
        
        self.circuits.insert(circuit_id, circuit);
        
        self.type_mapping
            .entry(circuit_type)
            .or_insert_with(Vec::new)
            .push(circuit_id);
    }
    
    /// 获取电路
    pub fn get_circuit(&self, circuit_id: u32) -> Option<&NetworkMetricCircuit> {
        self.circuits.get(&circuit_id)
    }
    
    /// 根据类型获取电路
    pub fn get_circuits_by_type(&self, circuit_type: &CircuitType) -> Vec<&NetworkMetricCircuit> {
        if let Some(circuit_ids) = self.type_mapping.get(circuit_type) {
            circuit_ids
                .iter()
                .filter_map(|id| self.circuits.get(id))
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// 选择最适合的电路
    pub fn select_optimal_circuit(
        &self,
        circuit_type: &CircuitType,
        data_points: usize,
        sources: usize,
    ) -> Option<&NetworkMetricCircuit> {
        let candidates = self.get_circuits_by_type(circuit_type);
        
        // 选择能处理数据且复杂度最低的电路
        candidates
            .into_iter()
            .filter(|circuit| {
                circuit.max_data_points >= data_points && circuit.max_data_sources >= sources
            })
            .min_by_key(|circuit| {
                let complexity = circuit.estimate_complexity(data_points, sources);
                complexity.estimated_generation_time_ms
            })
    }
    
    /// 获取所有已注册的电路类型
    pub fn get_supported_types(&self) -> Vec<CircuitType> {
        self.type_mapping.keys().cloned().collect()
    }
    
    /// 获取电路统计信息
    pub fn get_statistics(&self) -> CircuitStatistics {
        CircuitStatistics {
            total_circuits: self.circuits.len(),
            circuits_by_type: self.type_mapping
                .iter()
                .map(|(t, ids)| (t.clone(), ids.len()))
                .collect(),
            average_max_data_points: self.circuits
                .values()
                .map(|c| c.max_data_points)
                .sum::<usize>() / self.circuits.len().max(1),
            average_max_sources: self.circuits
                .values()
                .map(|c| c.max_data_sources)
                .sum::<usize>() / self.circuits.len().max(1),
        }
    }
    
    // 私有辅助方法
    
    /// 注册默认电路
    fn register_default_circuits(&mut self) {
        // 小型网络指标电路
        let small_circuit = NetworkMetricCircuit::new(
            1,
            10,
            5,
            "小型网络指标电路，适用于少量数据点和数据源".to_string(),
        );
        self.register_circuit(small_circuit);
        
        // 中型网络指标电路
        let medium_circuit = NetworkMetricCircuit::new(
            2,
            50,
            20,
            "中型网络指标电路，适用于中等规模数据".to_string(),
        );
        self.register_circuit(medium_circuit);
        
        // 大型网络指标电路
        let large_circuit = NetworkMetricCircuit::new(
            3,
            200,
            100,
            "大型网络指标电路，适用于大规模数据聚合".to_string(),
        );
        self.register_circuit(large_circuit);
    }
}

/// 电路统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitStatistics {
    /// 总电路数量
    pub total_circuits: usize,
    /// 按类型分组的电路数量
    pub circuits_by_type: HashMap<CircuitType, usize>,
    /// 平均最大数据点数
    pub average_max_data_points: usize,
    /// 平均最大数据源数
    pub average_max_sources: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_network_metric_circuit_creation() {
        let circuit = NetworkMetricCircuit::new(
            1,
            10,
            5,
            "Test circuit".to_string(),
        );
        
        assert_eq!(circuit.circuit_id, 1);
        assert_eq!(circuit.circuit_type, CircuitType::NetworkMetric);
        assert_eq!(circuit.max_data_points, 10);
        assert_eq!(circuit.max_data_sources, 5);
    }
    
    #[test]
    fn test_constraint_verification() {
        let circuit = NetworkMetricCircuit::new(1, 10, 5, "Test".to_string());
        
        // 测试有效数据
        let private_data = vec![100, 200, 300];
        let data_sources = vec![90, 85, 95]; // 可靠性评分
        let public_metric = 200; // 平均值
        let quality_score = 88;
        
        let result = circuit.verify_constraints(
            &private_data,
            &data_sources,
            public_metric,
            quality_score,
        );
        
        assert!(result);
    }
    
    #[test]
    fn test_constraint_verification_failure() {
        let circuit = NetworkMetricCircuit::new(1, 10, 5, "Test".to_string());
        
        // 测试无效数据：质量评分超出范围
        let private_data = vec![100, 200, 300];
        let data_sources = vec![90, 85];
        let public_metric = 200;
        let quality_score = 150; // 无效：超过100
        
        let result = circuit.verify_constraints(
            &private_data,
            &data_sources,
            public_metric,
            quality_score,
        );
        
        assert!(!result);
    }
    
    #[test]
    fn test_complexity_estimation() {
        let circuit = NetworkMetricCircuit::new(1, 100, 50, "Test".to_string());
        
        let complexity = circuit.estimate_complexity(10, 5);
        
        assert!(complexity.constraint_count > 0);
        assert!(complexity.witness_count > 0);
        assert!(complexity.estimated_generation_time_ms > 0);
        assert!(complexity.estimated_verification_time_ms > 0);
    }
    
    #[test]
    fn test_circuit_manager() {
        let mut manager = CircuitManager::new();
        
        // 检查默认电路是否已注册
        assert!(manager.get_circuit(1).is_some());
        assert!(manager.get_circuit(2).is_some());
        assert!(manager.get_circuit(3).is_some());
        
        // 检查类型映射
        let network_circuits = manager.get_circuits_by_type(&CircuitType::NetworkMetric);
        assert_eq!(network_circuits.len(), 3);
        
        // 测试最优电路选择
        let optimal = manager.select_optimal_circuit(&CircuitType::NetworkMetric, 5, 3);
        assert!(optimal.is_some());
        assert_eq!(optimal.unwrap().circuit_id, 1); // 应该选择最小的电路
    }
    
    #[test]
    fn test_public_input_spec() {
        let circuit = NetworkMetricCircuit::new(1, 10, 5, "Test".to_string());
        let spec = circuit.get_public_input_spec();
        
        assert_eq!(spec.inputs.len(), 3);
        assert_eq!(spec.inputs[0].name, "aggregated_metric");
        assert_eq!(spec.inputs[1].name, "quality_score");
        assert_eq!(spec.inputs[2].name, "time_window");
    }
}