# PolyVisor 开发指南

## 🛠️ 开发环境搭建

### 系统要求
- **操作系统**: macOS 11+, Ubuntu 20.04+, Windows 10+ (WSL2)
- **内存**: 最少 8GB，推荐 16GB+
- **存储**: 至少 50GB 可用空间
- **网络**: 稳定的互联网连接

### 开发工具安装

#### 1. Rust开发环境
```bash
# 安装Rust和Cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 安装Substrate依赖
rustup default stable
rustup update
rustup target add wasm32-unknown-unknown

# 安装开发工具
cargo install --git https://github.com/paritytech/substrate subkey
cargo install cargo-contract --vers 3.2.0
```

#### 2. Node.js环境
```bash
# 使用nvm安装Node.js
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 18
nvm use 18

# 安装pnpm包管理器
npm install -g pnpm
```

#### 3. 数据库环境
```bash
# 安装PostgreSQL
brew install postgresql@15  # macOS
sudo apt install postgresql-15 postgresql-contrib  # Ubuntu

# 安装Redis
brew install redis  # macOS
sudo apt install redis-server  # Ubuntu

# 启动服务
brew services start postgresql@15
brew services start redis
```

#### 4. 容器环境
```bash
# 安装Docker
# macOS: 下载Docker Desktop
# Ubuntu: 
sudo apt update
sudo apt install docker.io docker-compose
sudo usermod -aG docker $USER
```

### IDE配置

#### VS Code推荐插件
```json
{
  "recommendations": [
    "rust-lang.rust-analyzer",
    "ms-vscode.vscode-typescript-next",
    "bradlc.vscode-tailwindcss",
    "esbenp.prettier-vscode",
    "ms-vscode.vscode-json",
    "redhat.vscode-yaml",
    "ms-vscode.hexeditor"
  ]
}
```

#### Rust Analyzer配置
```json
{
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.procMacro.enable": true
}
```

## 📁 项目结构详解

```
PolyVisor/
├── blockchain/                 # Substrate区块链节点
│   ├── node/                  # 节点实现
│   ├── runtime/               # 运行时代码
│   └── pallets/               # 自定义Pallets
├── contracts/                 # 智能合约
│   └── analytics/             # Analytics合约
├── zkproof/                   # 零知识证明
│   ├── circuits/              # 证明电路
│   └── prover/                # 证明器实现
├── backend/                   # 后端服务
│   ├── api-gateway/           # API网关
│   ├── data-collector/        # 数据收集
│   ├── privacy-service/       # 隐私保护
│   └── analytics-service/     # 分析服务
├── frontend/                  # React前端
│   ├── src/                   # 源代码
│   ├── public/                # 静态资源
│   └── dist/                  # 构建输出
├── docs/                      # 项目文档
├── scripts/                   # 构建脚本
└── docker-compose.yml         # 容器编排
```

## 🚀 快速启动

### 1. 克隆项目
```bash
git clone https://github.com/your-org/polyvisor.git
cd polyvisor
```

### 2. 环境配置
```bash
# 复制环境变量文件
cp .env.example .env

# 编辑配置文件
nano .env
```

### 3. 启动开发环境
```bash
# 使用Docker Compose一键启动
docker-compose up -d

# 或者分别启动各个服务
./scripts/dev.sh
```

### 4. 验证安装
```bash
# 检查区块链节点
curl http://localhost:9933/health

# 检查API服务
curl http://localhost:8080/api/v1/health

# 检查前端应用
curl http://localhost:3000
```

## 🔧 开发工作流

### Git工作流
```bash
# 创建功能分支
git checkout -b feature/new-feature

# 开发完成后提交
git add .
git commit -m "feat: 添加新功能"

# 推送并创建PR
git push origin feature/new-feature
```

### 代码提交规范
```
类型(范围): 简短描述

详细描述（可选）

- 列出具体变更
- 影响范围说明

Closes #123
```

**提交类型:**
- `feat`: 新功能
- `fix`: Bug修复
- `docs`: 文档更新
- `style`: 代码格式调整
- `refactor`: 代码重构
- `test`: 测试相关
- `chore`: 构建工具、辅助工具变动

### 分支策略
- `main`: 主分支，稳定版本
- `develop`: 开发分支，集成测试
- `feature/*`: 功能分支
- `hotfix/*`: 紧急修复分支
- `release/*`: 发布分支

## 🧪 测试指南

### 单元测试
```bash
# Rust测试
cd blockchain
cargo test

# Node.js测试
cd frontend
npm test

# 运行特定测试
cargo test test_analytics_pallet
```

### 集成测试
```bash
# 启动测试环境
docker-compose -f docker-compose.test.yml up -d

# 运行集成测试
./scripts/test.sh
```

### 端到端测试
```bash
# 安装Playwright
cd frontend
npx playwright install

# 运行E2E测试
npm run test:e2e
```

### 测试覆盖率
```bash
# Rust覆盖率
cargo install cargo-tarpaulin
cargo tarpaulin --out Html

# Frontend覆盖率
npm run test:coverage
```

## 📝 编码规范

### Rust编码规范

#### 命名约定
```rust
// 使用snake_case命名变量和函数
let user_address = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKv3gB";

// 使用PascalCase命名类型和特征
struct NetworkMetrics {
    block_number: u64,
    transaction_count: u32,
}

// 使用SCREAMING_SNAKE_CASE命名常量
const MAX_BLOCK_SIZE: u32 = 1024 * 1024;
```

#### 错误处理
```rust
// 使用Result类型处理错误
fn get_network_health() -> Result<HealthScore, NetworkError> {
    // 实现逻辑
}

// 使用?操作符传播错误
fn process_block(block: Block) -> Result<(), ProcessError> {
    let metrics = calculate_metrics(&block)?;
    store_metrics(metrics)?;
    Ok(())
}
```

#### 文档注释
```rust
/// 计算网络健康评分
/// 
/// # Arguments
/// 
/// * `metrics` - 网络指标数据
/// * `weights` - 各指标权重
/// 
/// # Returns
/// 
/// 返回0-100的健康评分
/// 
/// # Examples
/// 
/// ```
/// let score = calculate_health_score(&metrics, &weights);
/// assert!(score >= 0.0 && score <= 100.0);
/// ```
pub fn calculate_health_score(
    metrics: &NetworkMetrics,
    weights: &Weights,
) -> f64 {
    // 实现逻辑
}
```

### TypeScript编码规范

#### 接口定义
```typescript
// 使用PascalCase命名接口
interface NetworkMetrics {
  blockNumber: number;
  transactionCount: number;
  validatorCount: number;
  timestamp: string;
}

// 使用readonly修饰只读属性
interface ReadonlyConfig {
  readonly apiUrl: string;
  readonly wsUrl: string;
}
```

#### 函数定义
```typescript
// 使用camelCase命名函数
const fetchNetworkMetrics = async (
  timeRange: string
): Promise<NetworkMetrics[]> => {
  // 实现逻辑
};

// 使用箭头函数或function声明
function calculateHealthScore(metrics: NetworkMetrics): number {
  // 实现逻辑
}
```

#### 组件规范
```typescript
// React组件使用PascalCase
interface DashboardProps {
  metrics: NetworkMetrics[];
  loading: boolean;
}

const Dashboard: React.FC<DashboardProps> = ({ metrics, loading }) => {
  // 使用Hook管理状态
  const [selectedMetric, setSelectedMetric] = useState<string>('');
  
  // 事件处理函数使用handle前缀
  const handleMetricSelect = (metric: string) => {
    setSelectedMetric(metric);
  };

  return (
    // JSX实现
  );
};
```

### 代码格式化

#### Rust格式化
```bash
# 自动格式化
cargo fmt

# 检查格式
cargo fmt --check

# Clippy代码检查
cargo clippy -- -D warnings
```

#### TypeScript格式化
```bash
# 使用Prettier格式化
npx prettier --write "src/**/*.{ts,tsx}"

# ESLint检查
npx eslint "src/**/*.{ts,tsx}"
```

### 配置文件

#### .rustfmt.toml
```toml
max_width = 100
hard_tabs = false
tab_spaces = 4
newline_style = "Unix"
use_small_heuristics = "Default"
reorder_imports = true
reorder_modules = true
remove_nested_parens = true
```

#### .eslintrc.js
```javascript
module.exports = {
  extends: [
    'eslint:recommended',
    '@typescript-eslint/recommended',
    'react-hooks/recommended',
  ],
  rules: {
    '@typescript-eslint/no-unused-vars': 'error',
    '@typescript-eslint/explicit-function-return-type': 'warn',
    'react-hooks/exhaustive-deps': 'warn',
  },
};
```

## 🔍 调试指南

### Rust调试
```bash
# 使用环境变量启用调试日志
RUST_LOG=debug cargo run

# 使用gdb调试
rust-gdb target/debug/node-template

# 使用lldb调试（macOS）
rust-lldb target/debug/node-template
```

### Node.js调试
```bash
# 使用Chrome DevTools
node --inspect-brk server.js

# VS Code调试配置
{
  "type": "node",
  "request": "launch",
  "name": "Debug Server",
  "program": "${workspaceFolder}/server.js",
  "env": {
    "NODE_ENV": "development"
  }
}
```

### 前端调试
```bash
# React DevTools
npm install -g react-devtools

# 使用浏览器调试工具
# 在Chrome中按F12打开开发者工具
```

## 📊 性能优化

### Rust性能优化
```rust
// 使用&str而不是String减少内存分配
fn process_address(address: &str) -> Result<(), Error> {
    // 处理逻辑
}

// 使用Vec::with_capacity预分配内存
let mut metrics = Vec::with_capacity(1000);

// 使用迭代器而不是索引访问
let total: u64 = metrics.iter().map(|m| m.value).sum();
```

### 前端性能优化
```typescript
// 使用React.memo优化组件渲染
const MetricsChart = React.memo<MetricsChartProps>(({ data }) => {
  // 组件实现
});

// 使用useMemo缓存计算结果
const processedData = useMemo(() => {
  return data.map(item => processItem(item));
}, [data]);

// 使用useCallback优化事件处理
const handleClick = useCallback((id: string) => {
  // 处理逻辑
}, [dependency]);
```

## 🔒 安全最佳实践

### 代码安全
```rust
// 避免panic，使用Result类型
fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("Division by zero")
    } else {
        Ok(a / b)
    }
}

// 使用安全的字符串操作
let sanitized = input.chars()
    .filter(|c| c.is_alphanumeric())
    .collect::<String>();
```

### 依赖管理
```bash
# 检查依赖漏洞
cargo audit

# 更新依赖
cargo update

# NPM安全检查
npm audit
npm audit fix
```

## 📚 学习资源

### 官方文档
- [Substrate Developer Hub](https://docs.substrate.io/)
- [Polkadot Wiki](https://wiki.polkadot.network/)
- [React Documentation](https://reactjs.org/docs/)
- [TypeScript Handbook](https://www.typescriptlang.org/docs/)

### 教程和示例
- [Substrate Tutorials](https://docs.substrate.io/tutorials/v3/)
- [Zero-Knowledge Proofs](https://zkp.science/)
- [Web3 Development](https://ethereum.org/developers/)

### 社区资源
- [Substrate Stack Exchange](https://substrate.stackexchange.com/)
- [Polkadot Discord](https://discord.gg/polkadot)
- [Rust Community](https://www.rust-lang.org/community)

---

💡 **提示**: 开发过程中遇到问题，优先查阅官方文档和社区资源，善用搜索引擎和AI助手。