#!/bin/bash

# PolyVisor 安装脚本

echo "🚀 开始安装 PolyVisor 开发环境..."

# 检查必要的工具
echo "📋 检查环境依赖..."

# 检查 Rust
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust 未安装。请先安装 Rust: https://rustup.rs/"
    exit 1
fi

# 检查 Node.js
if ! command -v node &> /dev/null; then
    echo "❌ Node.js 未安装。请先安装 Node.js 18+: https://nodejs.org/"
    exit 1
fi

# 检查 Docker
if ! command -v docker &> /dev/null; then
    echo "❌ Docker 未安装。请先安装 Docker: https://docker.com/"
    exit 1
fi

echo "✅ 环境检查通过"

# 安装 Substrate 工具
echo "🔧 安装 Substrate 工具..."
if ! command -v substrate &> /dev/null; then
    cargo install --git https://github.com/paritytech/substrate subkey --force --locked
    cargo install --git https://github.com/paritytech/substrate substrate --force --locked
fi

# 安装 ink! CLI
echo "🔧 安装 ink! CLI..."
if ! command -v cargo-contract &> /dev/null; then
    cargo install cargo-contract --force
fi

# 创建目录结构
echo "📁 创建项目目录结构..."
mkdir -p blockchain/{node,runtime,pallets/{analytics,privacy}}
mkdir -p contracts/{analytics,zkproof,privacy}
mkdir -p backend/{src,api,services}
mkdir -p frontend/{src,components,pages}
mkdir -p zkproof/src
mkdir -p privacy/src
mkdir -p data-collection/src
mkdir -p docs
mkdir -p tests
mkdir -p docker
mkdir -p scripts

# 安装 Rust 依赖
echo "📦 安装 Rust 依赖..."
cargo check

# 安装前端依赖
echo "📦 安装前端依赖..."
cd frontend
npm install
cd ..

# 设置权限
chmod +x scripts/*.sh

echo "✅ PolyVisor 安装完成！"
echo ""
echo "🚀 快速开始:"
echo "  ./scripts/dev.sh     # 启动开发环境"
echo "  ./scripts/test.sh    # 运行测试"
echo "  ./scripts/build.sh   # 构建项目"
echo ""
echo "📖 更多信息请查看 README.md"