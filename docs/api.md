# PolyVisor API 文档

## 📋 API 概览

PolyVisor提供完整的REST API和GraphQL接口，支持网络数据查询、隐私设置管理和实时数据订阅。

### API版本
- **当前版本**: v1.0
- **基础URL**: `http://localhost:8080/api/v1`
- **GraphQL端点**: `http://localhost:8081/graphql`
- **WebSocket**: `ws://localhost:8081/ws`

## 🔐 身份验证

### JWT Token认证
```http
Authorization: Bearer <jwt_token>
```

### 获取访问令牌
```http
POST /api/v1/auth/login
Content-Type: application/json

{
  "address": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKv3gB",
  "signature": "0x...",
  "message": "Login message"
}
```

## 📊 REST API

### 1. 网络健康 API

#### 获取网络总体健康状况
```http
GET /api/v1/health/network
```

**响应示例:**
```json
{
  "overall_score": 92,
  "status": "healthy",
  "last_updated": "2024-01-15T10:30:00Z",
  "metrics": {
    "connectivity_score": 95,
    "throughput_score": 88,
    "latency_score": 90,
    "consensus_score": 94,
    "availability_score": 96
  },
  "warnings": [
    {
      "level": "warning",
      "message": "部分节点响应时间较慢",
      "component": "network",
      "timestamp": "2024-01-15T10:25:00Z"
    }
  ]
}
```

#### 获取详细网络指标
```http
GET /api/v1/metrics?timeRange=7d&granularity=1h
```

**查询参数:**
- `timeRange`: 时间范围 (1d, 7d, 30d, 90d)
- `granularity`: 数据粒度 (5m, 1h, 1d)

**响应示例:**
```json
{
  "metrics": [
    {
      "timestamp": "2024-01-15T10:00:00Z",
      "total_nodes": 156,
      "active_connections": 1247,
      "block_time_avg": 6.2,
      "transaction_rate": 45.8,
      "network_hash_rate": "125.6 TH/s"
    }
  ]
}
```

### 2. 数据分析 API

#### 获取网络趋势分析
```http
GET /api/v1/analytics/trends?timeRange=30d&type=network
```

**响应示例:**
```json
{
  "networkTrends": [
    {
      "timestamp": "2024-01-01",
      "transactions": 1250,
      "blocks": 45,
      "validators": 120,
      "privacy_score": 85.5
    }
  ],
  "predictions": {
    "network_growth": 0.15,
    "privacy_adoption": 0.25,
    "transaction_volume": 0.18,
    "confidence_score": 0.82
  }
}
```

#### 获取隐私指标
```http
GET /api/v1/analytics/privacy
```

**响应示例:**
```json
{
  "anonymity_set_size": 15000,
  "mixing_effectiveness": 0.92,
  "privacy_level": "high",
  "zk_proofs_generated": 8500,
  "k_anonymity_score": 12.8,
  "differential_privacy_epsilon": 0.1
}
```

### 3. 零知识证明 API

#### 生成ZK证明
```http
POST /api/v1/zkproof/generate
Content-Type: application/json

{
  "proof_type": "transaction",
  "data": {
    "amount": 1000,
    "sender_hash": "0x...",
    "recipient_hash": "0x..."
  },
  "privacy_level": "high"
}
```

**响应示例:**
```json
{
  "proof_id": "zk_proof_123456",
  "proof_hash": "0xabcdef...",
  "verification_key": "0x123456...",
  "status": "generated",
  "created_at": "2024-01-15T10:30:00Z"
}
```

#### 验证ZK证明
```http
POST /api/v1/zkproof/verify
Content-Type: application/json

{
  "proof_hash": "0xabcdef...",
  "verification_key": "0x123456...",
  "public_inputs": ["0x111", "0x222"]
}
```

### 4. 贡献者管理 API

#### 获取贡献者列表
```http
GET /api/v1/contributors?page=1&limit=20&type=validator
```

**响应示例:**
```json
{
  "contributors": [
    {
      "address": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKv3gB",
      "display_name": "Validator Node 001",
      "type": "validator",
      "contributions": 1247,
      "quality_score": 95.5,
      "reputation": 892.0,
      "last_active": "2024-01-15T10:30:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 20,
    "total": 156,
    "total_pages": 8
  }
}
```

#### 更新贡献者信息
```http
PUT /api/v1/contributors/:address
Content-Type: application/json

{
  "display_name": "Updated Validator Name",
  "metadata": {
    "location": "Tokyo",
    "website": "https://validator.example.com"
  }
}
```

### 5. 隐私设置 API

#### 获取用户隐私设置
```http
GET /api/v1/privacy/settings/:address
```

**响应示例:**
```json
{
  "address": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKv3gB",
  "settings": {
    "allow_analytics": true,
    "allow_sharing": false,
    "data_retention": 30,
    "privacy_level": "protected",
    "anonymization": true
  },
  "last_updated": "2024-01-15T10:30:00Z"
}
```

#### 更新隐私设置
```http
PUT /api/v1/privacy/settings/:address
Content-Type: application/json

{
  "allow_analytics": true,
  "allow_sharing": false,
  "data_retention": 60,
  "privacy_level": "high",
  "anonymization": true
}
```

## 🔄 GraphQL API

### Schema定义

```graphql
type Query {
  # 网络健康查询
  networkHealth: NetworkHealth
  networkMetrics(timeRange: String, granularity: String): [NetworkMetric]
  
  # 数据分析查询
  analyticsData(timeRange: String, type: String): AnalyticsData
  privacyMetrics: PrivacyMetrics
  
  # 贡献者查询
  contributors(filter: ContributorFilter, pagination: Pagination): ContributorConnection
  contributor(address: String!): Contributor
  
  # 零知识证明查询
  zkProof(proofHash: String!): ZKProof
  zkProofStats: ZKProofStats
}

type Mutation {
  # 零知识证明操作
  generateZKProof(input: ZKProofInput!): ZKProofResult
  verifyZKProof(input: ZKVerificationInput!): ZKVerificationResult
  
  # 隐私设置操作
  updatePrivacySettings(address: String!, settings: PrivacySettingsInput!): PrivacySettings
  
  # 贡献者操作
  updateContributor(address: String!, input: ContributorInput!): Contributor
}

type Subscription {
  # 实时数据订阅
  networkMetricsUpdated: NetworkMetric
  newZKProofGenerated: ZKProof
  contributorActivity: ContributorActivity
}
```

### 查询示例

#### 获取网络指标
```graphql
query GetNetworkMetrics($timeRange: String) {
  networkMetrics(timeRange: $timeRange) {
    timestamp
    totalNodes
    activeConnections
    blockTimeAvg
    transactionRate
  }
}
```

#### 生成ZK证明
```graphql
mutation GenerateZKProof($input: ZKProofInput!) {
  generateZKProof(input: $input) {
    proofId
    proofHash
    verificationKey
    status
    createdAt
  }
}
```

#### 订阅实时更新
```graphql
subscription NetworkUpdates {
  networkMetricsUpdated {
    timestamp
    totalNodes
    activeConnections
    transactionRate
  }
}
```

## 🔌 WebSocket API

### 连接建立
```javascript
const ws = new WebSocket('ws://localhost:8081/ws');

ws.onopen = function(event) {
  console.log('WebSocket连接已建立');
};

ws.onmessage = function(event) {
  const message = JSON.parse(event.data);
  console.log('收到消息:', message);
};
```

### 消息格式

#### 订阅网络指标
```json
{
  "type": "subscribe",
  "channel": "network_metrics",
  "params": {
    "granularity": "1m"
  }
}
```

#### 实时数据推送
```json
{
  "type": "network_metrics_update",
  "timestamp": "2024-01-15T10:30:00Z",
  "data": {
    "total_nodes": 156,
    "active_connections": 1247,
    "transaction_rate": 45.8
  }
}
```

## 📝 错误处理

### HTTP状态码
- `200 OK` - 请求成功
- `201 Created` - 资源创建成功
- `400 Bad Request` - 请求参数错误
- `401 Unauthorized` - 未授权访问
- `403 Forbidden` - 权限不足
- `404 Not Found` - 资源不存在
- `429 Too Many Requests` - 请求频率超限
- `500 Internal Server Error` - 服务器内部错误

### 错误响应格式
```json
{
  "error": {
    "code": "INVALID_PARAMETER",
    "message": "Invalid time range parameter",
    "details": {
      "parameter": "timeRange",
      "expected": "1d|7d|30d|90d",
      "received": "invalid"
    },
    "timestamp": "2024-01-15T10:30:00Z",
    "request_id": "req_123456"
  }
}
```

## 🔄 API限流

### 限流策略
- **用户级限流**: 每用户每分钟100请求
- **IP级限流**: 每IP每分钟500请求
- **接口级限流**: 计算密集型接口单独限制

### 限流响应头
```http
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 95
X-RateLimit-Reset: 1642248600
```

## 📊 API监控

### 性能指标
- 响应时间 (P50, P95, P99)
- 错误率
- QPS (每秒查询数)
- 并发连接数

### 监控告警
- 响应时间超过2秒
- 错误率超过5%
- QPS异常波动
- 服务不可用

## 🧪 API测试

### 测试环境
- **开发环境**: `http://localhost:8080`
- **测试环境**: `https://test-api.polyvisor.io`
- **生产环境**: `https://api.polyvisor.io`

### 测试工具推荐
- **Postman** - REST API测试
- **GraphQL Playground** - GraphQL测试
- **WebSocket King** - WebSocket测试

---

📚 **API文档更新**: 该API文档会随着功能更新保持同步，建议订阅更新通知。