# PolyVisor 项目文档

## 📚 文档目录

### 核心文档
- [项目概述](../README.md) - 项目介绍和快速开始
- [产品需求文档 (PRD)](../prd.md) - 产品功能需求
- [技术设计文档](../design.md) - 系统架构和技术方案
- [开发完成报告](../DEVELOPMENT_COMPLETE.md) - 项目开发总结

### 架构文档
- [系统架构](./architecture.md) - 整体系统架构设计
- [API文档](./api.md) - REST API和GraphQL接口
- [数据库设计](./database.md) - 数据模型和表结构
- [安全设计](./security.md) - 隐私保护和安全机制

### 开发文档
- [开发指南](./development.md) - 开发环境配置和编码规范
- [部署指南](./deployment.md) - 生产环境部署说明
- [运维指南](./operations.md) - 监控、日志和故障排查
- [测试指南](./testing.md) - 测试策略和测试用例

### 用户文档
- [用户手册](./user-guide.md) - 平台使用指南
- [FAQ](./faq.md) - 常见问题解答
- [更新日志](./changelog.md) - 版本更新记录

### 技术文档
- [区块链模块](./blockchain.md) - Substrate节点和Pallets
- [零知识证明](./zkproof.md) - ZK证明系统设计
- [前端开发](./frontend.md) - React应用开发指南
- [隐私保护](./privacy.md) - 隐私技术实现

## 📖 文档规范

### 文档结构
```
docs/
├── README.md           # 文档导航
├── architecture.md     # 系统架构
├── api.md             # API文档
├── database.md        # 数据库设计
├── security.md        # 安全设计
├── development.md     # 开发指南
├── deployment.md      # 部署指南
├── operations.md      # 运维指南
├── testing.md         # 测试指南
├── user-guide.md      # 用户手册
├── faq.md            # 常见问题
├── changelog.md       # 更新日志
├── blockchain.md      # 区块链模块
├── zkproof.md        # 零知识证明
├── frontend.md       # 前端开发
├── privacy.md        # 隐私保护
└── images/           # 文档图片
```

### 编写规范
1. **标题层级**: 使用标准的Markdown标题层级
2. **代码块**: 明确指定语言类型
3. **图片**: 统一存放在images目录
4. **链接**: 使用相对路径链接
5. **格式**: 保持一致的格式和风格

### 维护要求
- 每次重大功能更新需要同步更新相关文档
- 定期检查文档的准确性和完整性
- 保持文档的时效性和可读性

## 🔧 文档工具

### 预览工具
```bash
# 安装文档预览工具
npm install -g docsify-cli

# 启动文档服务
docsify serve docs
```

### 图表工具
- Mermaid - 流程图和架构图
- PlantUML - UML图表
- Draw.io - 复杂架构图

### 文档生成
- 自动从代码注释生成API文档
- 从测试用例生成测试文档
- 自动化更新版本信息

---

📝 **文档维护**: 请确保在修改代码时同步更新相关文档