# PolyVisor - Privacy-Preserving Network Analytics for Polkadot

PolyVisor是一个创新的网络分析平台，使用零知识证明技术为Polkadot生态系统提供隐私保护的实时网络指标和分析。

## 🌟 核心特性

- **隐私优先设计**: 使用零知识证明确保个人交易数据不被暴露
- **链上验证**: 所有指标都可在链上验证真实性
- **实时分析**: 提供实时网络健康度和性能指标
- **用户友好界面**: 直观的数据可视化界面
- **可扩展架构**: 模块化架构支持功能扩展

## 🏗️ 项目结构

```
PolyVisor/
├── blockchain/          # Substrate区块链
│   ├── runtime/         # 运行时模块
│   ├── pallets/         # 自定义模块
│   └── node/           # 节点实现
├── contracts/          # ink!智能合约
│   ├── analytics/      # 分析合约
│   ├── zkproof/       # 零知识证明合约
│   └── privacy/       # 隐私保护合约
├── backend/           # 后端服务
│   ├── src/           # 源代码
│   ├── api/           # API接口
│   └── services/      # 业务服务
├── frontend/          # React前端应用
│   ├── src/           # 源代码
│   ├── components/    # React组件
│   └── pages/         # 页面组件
├── zkproof/           # 零知识证明库
├── privacy/           # 隐私保护库
├── data-collection/   # 数据收集服务
├── docs/             # 文档
├── tests/            # 测试文件
├── docker/           # Docker配置
└── scripts/          # 构建脚本
```

## 🚀 快速开始

### 环境要求

- Rust 1.70+
- Node.js 18+
- Docker & Docker Compose
- Substrate CLI

### 安装与运行

```bash
# 克隆项目
git clone <repository-url>
cd PolyVisor

# 安装依赖
./scripts/install.sh

# 启动本地开发环境
./scripts/dev.sh

# 访问应用
# 前端: http://localhost:3000
# 后端API: http://localhost:8080
# 区块链节点: ws://localhost:9944
```

## 📖 技术架构

### 区块链层
- **Substrate框架**: 自定义区块链运行时
- **ink!智能合约**: Rust编写的智能合约
- **Polkadot.js**: 区块链交互库

### 后端服务
- **Rust**: 高性能后端服务
- **GraphQL + REST**: 灵活的API接口
- **Redis**: 数据缓存
- **PostgreSQL**: 元数据存储

### 前端应用
- **React 18**: 现代化用户界面
- **TypeScript**: 类型安全
- **D3.js**: 数据可视化
- **Tailwind CSS**: 样式框架

### 存储层
- **IPFS**: 分布式文件存储
- **链上存储**: 关键数据和证明哈希

## 🔒 隐私保护

PolyVisor采用多层隐私保护机制：

1. **零知识证明**: 证明数据真实性而不暴露原始数据
2. **差分隐私**: 在数据中添加校准过的噪声
3. **K-匿名性**: 确保数据无法追溯到特定用户
4. **数据泛化**: 降低数据精度保护隐私
5. **用户控制**: 多级隐私设置供用户选择

## 🧪 测试

```bash
# 运行所有测试
cargo test

# 运行智能合约测试
cd contracts && cargo test

# 运行前端测试
cd frontend && npm test

# 运行集成测试
./scripts/test-integration.sh
```

## 📊 数据指标

### 支持的网络指标
- 平均区块时间
- 交易吞吐量
- 验证者在线率
- 网络拥堵度
- 链活跃度
- Gas使用情况
- 网络延迟

### 隐私保护级别
- **最大隐私**: 高度聚合的数据
- **高隐私**: 区间数据
- **中等隐私**: 近似值
- **低隐私**: 较详细数据
- **最小隐私**: 详细数据

## 🤝 贡献指南

1. Fork本项目
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建Pull Request

## 📝 开发计划

- [x] 项目结构搭建
- [ ] Substrate区块链开发
- [ ] 智能合约实现
- [ ] 零知识证明集成
- [ ] 后端API开发
- [ ] 前端界面开发
- [ ] 数据可视化
- [ ] 集成测试
- [ ] 部署上线

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情

## 🙏 致谢

感谢Polkadot生态系统和所有开源贡献者的支持。

---

**注意**: 这是一个黑客松项目，仍在积极开发中。