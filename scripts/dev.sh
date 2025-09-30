#!/bin/bash

# PolyVisor 开发环境启动脚本

echo "🚀 启动 PolyVisor 开发环境..."

# 检查 Docker 是否运行
if ! docker info &> /dev/null; then
    echo "❌ Docker 未运行，请先启动 Docker"
    exit 1
fi

# 启动基础服务 (Redis, PostgreSQL)
echo "🗄️ 启动基础服务..."
docker-compose up -d redis postgres

# 等待数据库启动
echo "⏳ 等待数据库启动..."
sleep 10

# 检查区块链是否已构建
if [ ! -f "blockchain/target/release/polyvisor-node" ]; then
    echo "🔨 构建区块链节点..."
    cd blockchain
    cargo build --release
    cd ..
fi

# 启动区块链节点
echo "⛓️ 启动区块链节点..."
docker-compose up -d blockchain

# 等待区块链启动
echo "⏳ 等待区块链节点启动..."
sleep 15

# 部署智能合约
echo "📄 部署智能合约..."
# TODO: 添加合约部署逻辑

# 启动后端服务
echo "🔧 启动后端服务..."
# 在开发模式下直接运行，不使用容器
cd backend
cargo run &
BACKEND_PID=$!
cd ..

# 启动数据收集服务
echo "📊 启动数据收集服务..."
cd data-collection
cargo run &
DATA_COLLECTOR_PID=$!
cd ..

# 启动前端应用
echo "🌐 启动前端应用..."
cd frontend
npm start &
FRONTEND_PID=$!
cd ..

echo "✅ 开发环境启动完成！"
echo ""
echo "🌐 访问地址:"
echo "  前端应用: http://localhost:3000"
echo "  后端API:  http://localhost:8080"
echo "  区块链:   ws://localhost:9944"
echo ""
echo "🛑 停止服务:"
echo "  Ctrl+C 或运行 ./scripts/stop.sh"

# 等待用户中断
trap "echo '🛑 停止服务...'; kill $BACKEND_PID $DATA_COLLECTOR_PID $FRONTEND_PID 2>/dev/null; docker-compose down; exit" INT TERM

# 保持脚本运行
wait