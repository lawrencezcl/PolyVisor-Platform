#!/bin/bash
set -e

# PolyVisor 服务器启动脚本（支持外部IP访问）

echo "🚀 启动 PolyVisor 完整服务..."

# 获取服务器IP地址
if [ -z "$SERVER_IP" ]; then
    # 尝试自动获取外部IP
    SERVER_IP=$(curl -s ifconfig.me 2>/dev/null || curl -s ipinfo.io/ip 2>/dev/null || echo "localhost")
    echo "⚙️ 自动检测到服务器IP: $SERVER_IP"
else
    echo "⚙️ 使用指定的服务器IP: $SERVER_IP"
fi

# 导出环境变量供Docker Compose使用
export SERVER_IP

# 创建必要的目录
mkdir -p volumes/postgres
mkdir -p volumes/redis
mkdir -p volumes/blockchain

# 停止现有服务（如果存在）
echo "🛑 停止现有服务..."
docker-compose down -v 2>/dev/null || true

# 清理旧的容器和网络
echo "🧹 清理旧资源..."
docker system prune -f

# 启动基础服务
echo "🗄️ 启动基础服务 (PostgreSQL, Redis)..."
docker-compose up -d postgres redis

# 等待数据库启动
echo "⏳ 等待数据库启动..."
sleep 15

# 检查数据库连接
echo "🔍 检查数据库连接..."
for i in {1..30}; do
    if docker exec polyvisor-postgres-1 pg_isready -U polyvisor 2>/dev/null; then
        echo "✅ 数据库连接成功"
        break
    fi
    echo "⏳ 等待数据库连接... ($i/30)"
    sleep 2
done

# 构建并启动区块链节点
echo "⛓️ 构建并启动区块链节点..."
docker-compose build blockchain
docker-compose up -d blockchain

# 等待区块链节点启动
echo "⏳ 等待区块链节点启动..."
sleep 20

# 检查区块链节点健康状态
echo "🔍 检查区块链节点状态..."
for i in {1..20}; do
    if curl -s http://localhost:9933/health 2>/dev/null; then
        echo "✅ 区块链节点启动成功"
        break
    fi
    echo "⏳ 等待区块链节点启动... ($i/20)"
    sleep 3
done

# 构建并启动后端服务
echo "🔧 构建并启动后端API服务..."
docker-compose build backend
docker-compose up -d backend

# 等待后端服务启动
echo "⏳ 等待后端服务启动..."
sleep 15

# 检查后端服务健康状态
echo "🔍 检查后端服务状态..."
for i in {1..15}; do
    if curl -s http://localhost:8080/health 2>/dev/null; then
        echo "✅ 后端服务启动成功"
        break
    fi
    echo "⏳ 等待后端服务启动... ($i/15)"
    sleep 2
done

# 启动数据收集服务
echo "📊 启动数据收集服务..."
docker-compose up -d data-collector

# 构建并启动前端服务
echo "🌐 构建并启动前端服务..."
docker-compose build frontend
docker-compose up -d frontend

# 等待前端服务启动
echo "⏳ 等待前端服务启动..."
sleep 20

# 显示服务状态
echo ""
echo "=============================================="
echo "🎉 PolyVisor 服务启动完成!"
echo "=============================================="
echo ""
echo "📊 服务访问地址:"
echo "  🌐 前端应用:        http://$SERVER_IP:3000"
echo "  🔧 后端API:         http://$SERVER_IP:8080"
echo "  📖 API文档:         http://$SERVER_IP:8080/docs"
echo "  🔍 GraphQL:         http://$SERVER_IP:8080/graphql"
echo "  ⛓️ 区块链节点 (WS): ws://$SERVER_IP:9944"
echo "  ⛓️ 区块链节点 (RPC): http://$SERVER_IP:9933"
echo ""
echo "🔍 健康检查:"
echo "  后端健康状态: curl http://$SERVER_IP:8080/health"
echo "  区块链状态:   curl http://$SERVER_IP:9933/health"
echo ""
echo "📋 服务状态:"
docker-compose ps
echo ""
echo "📝 实时日志查看:"
echo "  所有服务: docker-compose logs -f"
echo "  后端服务: docker-compose logs -f backend"
echo "  前端服务: docker-compose logs -f frontend"
echo "  区块链:   docker-compose logs -f blockchain"
echo ""
echo "🛑 停止服务: ./scripts/stop.sh"
echo "=============================================="

# 等待用户中断
echo "按 Ctrl+C 查看实时日志，或运行 './scripts/stop.sh' 停止服务"
sleep 3

# 显示实时日志
docker-compose logs -f