# PolyVisor 数据库设计文档

## 📊 数据库概览

PolyVisor采用PostgreSQL作为主数据库，Redis作为缓存层，设计了完整的数据模型来支持隐私保护的网络分析功能。

## 🗄️ 数据库架构

### 数据库选择
- **主数据库**: PostgreSQL 15
- **缓存数据库**: Redis 7
- **时序数据**: TimescaleDB (PostgreSQL扩展)
- **搜索引擎**: Elasticsearch (可选)

### 数据库连接配置
```yaml
database:
  host: localhost
  port: 5432
  name: polyvisor
  user: polyvisor
  password: ${DATABASE_PASSWORD}
  pool_size: 20
  max_connections: 100
  ssl_mode: require
```

## 📋 数据表设计

### 1. 用户相关表

#### users - 用户基础信息表
```sql
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    address VARCHAR(255) UNIQUE NOT NULL,
    display_name VARCHAR(255),
    email VARCHAR(255),
    avatar_url TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE,
    
    CONSTRAINT users_address_check CHECK (LENGTH(address) >= 40)
);

-- 索引
CREATE INDEX idx_users_address ON users(address);
CREATE INDEX idx_users_created_at ON users(created_at);
CREATE INDEX idx_users_deleted_at ON users(deleted_at) WHERE deleted_at IS NULL;
```

#### user_sessions - 用户会话表
```sql
CREATE TABLE user_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id BIGINT NOT NULL REFERENCES users(id),
    session_token VARCHAR(255) UNIQUE NOT NULL,
    ip_address INET,
    user_agent TEXT,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    
    CONSTRAINT valid_expiry CHECK (expires_at > created_at)
);

-- 索引
CREATE INDEX idx_user_sessions_token ON user_sessions(session_token);
CREATE INDEX idx_user_sessions_user_id ON user_sessions(user_id);
CREATE INDEX idx_user_sessions_expires_at ON user_sessions(expires_at);
```

### 2. 网络数据表

#### network_metrics - 网络指标表
```sql
CREATE TABLE network_metrics (
    id BIGSERIAL PRIMARY KEY,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    block_number BIGINT NOT NULL,
    block_hash VARCHAR(66) NOT NULL,
    transaction_count INTEGER NOT NULL DEFAULT 0,
    validator_count INTEGER NOT NULL DEFAULT 0,
    total_nodes INTEGER NOT NULL DEFAULT 0,
    active_connections INTEGER NOT NULL DEFAULT 0,
    network_health_score DECIMAL(5,2) CHECK (network_health_score >= 0 AND network_health_score <= 100),
    privacy_score DECIMAL(5,2) CHECK (privacy_score >= 0 AND privacy_score <= 100),
    throughput_tps DECIMAL(10,2),
    latency_ms DECIMAL(10,2),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 转换为时序表
SELECT create_hypertable('network_metrics', 'timestamp');

-- 索引
CREATE INDEX idx_network_metrics_timestamp ON network_metrics(timestamp DESC);
CREATE INDEX idx_network_metrics_block_number ON network_metrics(block_number);
CREATE INDEX idx_network_metrics_block_hash ON network_metrics(block_hash);
```

#### blocks - 区块信息表
```sql
CREATE TABLE blocks (
    id BIGSERIAL PRIMARY KEY,
    block_number BIGINT UNIQUE NOT NULL,
    block_hash VARCHAR(66) UNIQUE NOT NULL,
    parent_hash VARCHAR(66) NOT NULL,
    state_root VARCHAR(66) NOT NULL,
    extrinsics_root VARCHAR(66) NOT NULL,
    block_time TIMESTAMP WITH TIME ZONE NOT NULL,
    validator_address VARCHAR(255),
    extrinsics_count INTEGER NOT NULL DEFAULT 0,
    block_size_bytes INTEGER NOT NULL DEFAULT 0,
    block_weight BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 索引
CREATE INDEX idx_blocks_number ON blocks(block_number DESC);
CREATE INDEX idx_blocks_hash ON blocks(block_hash);
CREATE INDEX idx_blocks_time ON blocks(block_time DESC);
CREATE INDEX idx_blocks_validator ON blocks(validator_address);
```

#### transactions - 交易信息表
```sql
CREATE TABLE transactions (
    id BIGSERIAL PRIMARY KEY,
    tx_hash VARCHAR(66) UNIQUE NOT NULL,
    block_id BIGINT NOT NULL REFERENCES blocks(id),
    extrinsic_index INTEGER NOT NULL,
    pallet_name VARCHAR(100) NOT NULL,
    call_name VARCHAR(100) NOT NULL,
    signer_address VARCHAR(255),
    nonce BIGINT,
    success BOOLEAN NOT NULL DEFAULT false,
    fee_paid DECIMAL(30,0),
    tip DECIMAL(30,0),
    weight_used BIGINT,
    events_count INTEGER DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    
    UNIQUE(block_id, extrinsic_index)
);

-- 索引
CREATE INDEX idx_transactions_hash ON transactions(tx_hash);
CREATE INDEX idx_transactions_block_id ON transactions(block_id);
CREATE INDEX idx_transactions_signer ON transactions(signer_address);
CREATE INDEX idx_transactions_pallet_call ON transactions(pallet_name, call_name);
CREATE INDEX idx_transactions_success ON transactions(success);
```

### 3. 隐私保护相关表

#### privacy_data - 隐私数据表
```sql
CREATE TABLE privacy_data (
    id BIGSERIAL PRIMARY KEY,
    user_address VARCHAR(255) NOT NULL,
    data_hash VARCHAR(255) UNIQUE NOT NULL,
    data_type VARCHAR(50) NOT NULL,
    anonymity_level VARCHAR(50) NOT NULL,
    k_anonymity_value INTEGER,
    privacy_budget_used DECIMAL(10,6),
    differential_privacy_epsilon DECIMAL(10,6),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP WITH TIME ZONE,
    
    CONSTRAINT privacy_data_anonymity_level_check 
    CHECK (anonymity_level IN ('public', 'protected', 'private', 'sensitive'))
);

-- 索引
CREATE INDEX idx_privacy_data_user ON privacy_data(user_address);
CREATE INDEX idx_privacy_data_hash ON privacy_data(data_hash);
CREATE INDEX idx_privacy_data_type ON privacy_data(data_type);
CREATE INDEX idx_privacy_data_expires ON privacy_data(expires_at);
```

#### privacy_settings - 隐私设置表
```sql
CREATE TABLE privacy_settings (
    id BIGSERIAL PRIMARY KEY,
    user_address VARCHAR(255) UNIQUE NOT NULL,
    allow_analytics BOOLEAN NOT NULL DEFAULT true,
    allow_sharing BOOLEAN NOT NULL DEFAULT false,
    data_retention_days INTEGER NOT NULL DEFAULT 30,
    privacy_level VARCHAR(50) NOT NULL DEFAULT 'protected',
    anonymization_enabled BOOLEAN NOT NULL DEFAULT true,
    differential_privacy_enabled BOOLEAN NOT NULL DEFAULT true,
    k_anonymity_threshold INTEGER NOT NULL DEFAULT 5,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    
    CONSTRAINT privacy_settings_level_check 
    CHECK (privacy_level IN ('public', 'protected', 'private', 'sensitive')),
    CONSTRAINT privacy_settings_retention_check 
    CHECK (data_retention_days >= 1 AND data_retention_days <= 365),
    CONSTRAINT privacy_settings_k_anonymity_check 
    CHECK (k_anonymity_threshold >= 2)
);

-- 索引
CREATE INDEX idx_privacy_settings_user ON privacy_settings(user_address);
```

### 4. 零知识证明相关表

#### zk_proofs - 零知识证明表
```sql
CREATE TABLE zk_proofs (
    id BIGSERIAL PRIMARY KEY,
    proof_hash VARCHAR(255) UNIQUE NOT NULL,
    proof_type VARCHAR(100) NOT NULL,
    prover_address VARCHAR(255),
    verifier_address VARCHAR(255),
    public_inputs JSONB,
    proof_data BYTEA,
    verification_key BYTEA,
    verification_status BOOLEAN,
    verification_attempts INTEGER DEFAULT 0,
    gas_used BIGINT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    verified_at TIMESTAMP WITH TIME ZONE,
    
    CONSTRAINT zk_proofs_type_check 
    CHECK (proof_type IN ('transaction', 'identity', 'membership', 'range', 'knowledge'))
);

-- 索引
CREATE INDEX idx_zk_proofs_hash ON zk_proofs(proof_hash);
CREATE INDEX idx_zk_proofs_type ON zk_proofs(proof_type);
CREATE INDEX idx_zk_proofs_prover ON zk_proofs(prover_address);
CREATE INDEX idx_zk_proofs_status ON zk_proofs(verification_status);
CREATE INDEX idx_zk_proofs_created ON zk_proofs(created_at DESC);
```

#### zk_proof_circuits - 证明电路表
```sql
CREATE TABLE zk_proof_circuits (
    id BIGSERIAL PRIMARY KEY,
    circuit_name VARCHAR(100) UNIQUE NOT NULL,
    circuit_version VARCHAR(20) NOT NULL,
    setup_params BYTEA,
    proving_key BYTEA,
    verification_key BYTEA,
    circuit_size INTEGER,
    constraint_count INTEGER,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    
    UNIQUE(circuit_name, circuit_version)
);

-- 索引
CREATE INDEX idx_zk_circuits_name ON zk_proof_circuits(circuit_name);
CREATE INDEX idx_zk_circuits_active ON zk_proof_circuits(is_active);
```

### 5. 贡献者相关表

#### contributors - 贡献者表
```sql
CREATE TABLE contributors (
    id BIGSERIAL PRIMARY KEY,
    address VARCHAR(255) UNIQUE NOT NULL,
    display_name VARCHAR(255),
    contributor_type VARCHAR(50) NOT NULL,
    contribution_count INTEGER DEFAULT 0,
    quality_score DECIMAL(5,2) DEFAULT 0,
    reputation DECIMAL(10,2) DEFAULT 0,
    stake_amount DECIMAL(30,0) DEFAULT 0,
    commission_rate DECIMAL(5,4),
    location_country VARCHAR(2),
    location_city VARCHAR(100),
    website_url TEXT,
    contact_email VARCHAR(255),
    is_active BOOLEAN DEFAULT true,
    last_active_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    
    CONSTRAINT contributors_type_check 
    CHECK (contributor_type IN ('validator', 'data_provider', 'researcher', 'developer', 'individual')),
    CONSTRAINT contributors_quality_check 
    CHECK (quality_score >= 0 AND quality_score <= 100),
    CONSTRAINT contributors_commission_check 
    CHECK (commission_rate >= 0 AND commission_rate <= 1)
);

-- 索引
CREATE INDEX idx_contributors_address ON contributors(address);
CREATE INDEX idx_contributors_type ON contributors(contributor_type);
CREATE INDEX idx_contributors_active ON contributors(is_active);
CREATE INDEX idx_contributors_reputation ON contributors(reputation DESC);
```

#### contributor_activities - 贡献者活动表
```sql
CREATE TABLE contributor_activities (
    id BIGSERIAL PRIMARY KEY,
    contributor_id BIGINT NOT NULL REFERENCES contributors(id),
    activity_type VARCHAR(50) NOT NULL,
    activity_data JSONB,
    points_earned INTEGER DEFAULT 0,
    quality_impact DECIMAL(5,2),
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    
    CONSTRAINT activity_type_check 
    CHECK (activity_type IN ('validation', 'data_submission', 'research', 'development', 'governance'))
);

-- 转换为时序表
SELECT create_hypertable('contributor_activities', 'timestamp');

-- 索引
CREATE INDEX idx_contributor_activities_contributor ON contributor_activities(contributor_id);
CREATE INDEX idx_contributor_activities_type ON contributor_activities(activity_type);
CREATE INDEX idx_contributor_activities_timestamp ON contributor_activities(timestamp DESC);
```

### 6. 分析和统计表

#### analytics_cache - 分析缓存表
```sql
CREATE TABLE analytics_cache (
    id BIGSERIAL PRIMARY KEY,
    cache_key VARCHAR(255) UNIQUE NOT NULL,
    cache_data JSONB NOT NULL,
    cache_type VARCHAR(50) NOT NULL,
    time_range VARCHAR(20),
    granularity VARCHAR(10),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    access_count INTEGER DEFAULT 0,
    last_accessed TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 索引
CREATE INDEX idx_analytics_cache_key ON analytics_cache(cache_key);
CREATE INDEX idx_analytics_cache_type ON analytics_cache(cache_type);
CREATE INDEX idx_analytics_cache_expires ON analytics_cache(expires_at);
```

#### system_events - 系统事件表
```sql
CREATE TABLE system_events (
    id BIGSERIAL PRIMARY KEY,
    event_type VARCHAR(50) NOT NULL,
    event_source VARCHAR(100) NOT NULL,
    event_data JSONB,
    severity VARCHAR(20) NOT NULL DEFAULT 'info',
    user_address VARCHAR(255),
    ip_address INET,
    user_agent TEXT,
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    
    CONSTRAINT severity_check 
    CHECK (severity IN ('debug', 'info', 'warning', 'error', 'critical'))
);

-- 转换为时序表
SELECT create_hypertable('system_events', 'timestamp');

-- 索引
CREATE INDEX idx_system_events_type ON system_events(event_type);
CREATE INDEX idx_system_events_source ON system_events(event_source);
CREATE INDEX idx_system_events_severity ON system_events(severity);
CREATE INDEX idx_system_events_timestamp ON system_events(timestamp DESC);
CREATE INDEX idx_system_events_user ON system_events(user_address);
```

## 🔍 视图和函数

### 1. 常用查询视图

#### 网络健康综合视图
```sql
CREATE VIEW network_health_summary AS
SELECT 
    DATE_TRUNC('hour', timestamp) as hour,
    AVG(network_health_score) as avg_health_score,
    AVG(privacy_score) as avg_privacy_score,
    AVG(total_nodes) as avg_nodes,
    AVG(transaction_count) as avg_transactions,
    MIN(network_health_score) as min_health_score,
    MAX(network_health_score) as max_health_score
FROM network_metrics 
WHERE timestamp >= NOW() - INTERVAL '7 days'
GROUP BY DATE_TRUNC('hour', timestamp)
ORDER BY hour DESC;
```

#### 贡献者统计视图
```sql
CREATE VIEW contributor_stats AS
SELECT 
    c.contributor_type,
    COUNT(*) as total_count,
    AVG(c.quality_score) as avg_quality_score,
    SUM(c.contribution_count) as total_contributions,
    AVG(c.reputation) as avg_reputation
FROM contributors c
WHERE c.is_active = true
GROUP BY c.contributor_type;
```

### 2. 存储过程和函数

#### 计算隐私评分函数
```sql
CREATE OR REPLACE FUNCTION calculate_privacy_score(
    anonymity_set_size INTEGER,
    k_anonymity_value INTEGER,
    differential_privacy_epsilon DECIMAL
) RETURNS DECIMAL AS $$
DECLARE
    base_score DECIMAL := 0;
    anonymity_factor DECIMAL := 0;
    k_factor DECIMAL := 0;
    dp_factor DECIMAL := 0;
BEGIN
    -- 基于匿名集大小计算分数
    anonymity_factor := LEAST(anonymity_set_size / 10000.0 * 40, 40);
    
    -- 基于K-匿名性计算分数
    k_factor := LEAST(k_anonymity_value / 20.0 * 30, 30);
    
    -- 基于差分隐私参数计算分数
    dp_factor := CASE 
        WHEN differential_privacy_epsilon <= 0.1 THEN 30
        WHEN differential_privacy_epsilon <= 0.5 THEN 20
        WHEN differential_privacy_epsilon <= 1.0 THEN 10
        ELSE 0
    END;
    
    base_score := anonymity_factor + k_factor + dp_factor;
    
    RETURN LEAST(base_score, 100);
END;
$$ LANGUAGE plpgsql;
```

#### 清理过期数据存储过程
```sql
CREATE OR REPLACE FUNCTION cleanup_expired_data()
RETURNS INTEGER AS $$
DECLARE
    deleted_count INTEGER := 0;
BEGIN
    -- 清理过期的隐私数据
    DELETE FROM privacy_data 
    WHERE expires_at IS NOT NULL AND expires_at < NOW();
    
    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    
    -- 清理过期的分析缓存
    DELETE FROM analytics_cache 
    WHERE expires_at < NOW();
    
    -- 清理过期的用户会话
    DELETE FROM user_sessions 
    WHERE expires_at < NOW();
    
    -- 清理旧的系统事件（保留30天）
    DELETE FROM system_events 
    WHERE timestamp < NOW() - INTERVAL '30 days';
    
    RETURN deleted_count;
END;
$$ LANGUAGE plpgsql;
```

## 📊 数据分区策略

### 时间分区
```sql
-- 按月分区网络指标表
CREATE TABLE network_metrics_y2024m01 PARTITION OF network_metrics
FOR VALUES FROM ('2024-01-01') TO ('2024-02-01');

CREATE TABLE network_metrics_y2024m02 PARTITION OF network_metrics
FOR VALUES FROM ('2024-02-01') TO ('2024-03-01');
```

### 自动分区管理
```sql
-- 创建自动分区函数
CREATE OR REPLACE FUNCTION create_monthly_partitions()
RETURNS VOID AS $$
DECLARE
    start_date DATE;
    end_date DATE;
    table_name TEXT;
BEGIN
    start_date := DATE_TRUNC('month', CURRENT_DATE + INTERVAL '1 month');
    end_date := start_date + INTERVAL '1 month';
    table_name := 'network_metrics_y' || TO_CHAR(start_date, 'YYYY') || 'm' || TO_CHAR(start_date, 'MM');
    
    EXECUTE format('CREATE TABLE IF NOT EXISTS %I PARTITION OF network_metrics FOR VALUES FROM (%L) TO (%L)',
                  table_name, start_date, end_date);
END;
$$ LANGUAGE plpgsql;
```

## 🔐 数据安全

### 数据加密
```sql
-- 启用透明数据加密
ALTER TABLE privacy_data SET (encryption_key_id = 1);
ALTER TABLE zk_proofs SET (encryption_key_id = 1);
```

### 行级安全
```sql
-- 启用行级安全
ALTER TABLE privacy_data ENABLE ROW LEVEL SECURITY;

-- 创建安全策略
CREATE POLICY privacy_data_access_policy ON privacy_data
    FOR ALL TO application_role
    USING (user_address = current_setting('app.user_address'));
```

### 数据脱敏
```sql
-- 创建脱敏视图
CREATE VIEW users_masked AS
SELECT 
    id,
    LEFT(address, 10) || '...' || RIGHT(address, 4) as address_masked,
    display_name,
    created_at
FROM users;
```

## 🚀 性能优化

### 索引优化策略
1. **主键索引**: 自动创建，用于唯一标识
2. **外键索引**: 提升关联查询性能
3. **复合索引**: 支持多字段查询
4. **部分索引**: 过滤条件索引
5. **表达式索引**: 函数计算索引

### 查询优化
```sql
-- 分析查询计划
EXPLAIN (ANALYZE, BUFFERS) 
SELECT * FROM network_metrics 
WHERE timestamp >= NOW() - INTERVAL '1 day';

-- 创建复合索引优化查询
CREATE INDEX idx_network_metrics_time_type 
ON network_metrics(timestamp DESC, block_number) 
WHERE network_health_score > 80;
```

### 连接池配置
```yaml
connection_pool:
  max_size: 20
  min_idle: 5
  connection_timeout: 30s
  idle_timeout: 600s
  max_lifetime: 1800s
```

## 📈 监控和维护

### 数据库监控指标
- 连接数使用率
- 查询响应时间
- 索引使用效率
- 表空间使用率
- 锁等待情况

### 定期维护任务
```sql
-- 分析表统计信息
ANALYZE;

-- 重建索引
REINDEX INDEX CONCURRENTLY idx_network_metrics_timestamp;

-- 清理死行
VACUUM ANALYZE network_metrics;
```

---

📊 **数据库版本**: PostgreSQL 15.x，定期评估升级到最新稳定版本。