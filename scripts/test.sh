#!/bin/bash

# PolyVisor 测试脚本

echo "🧪 运行 PolyVisor 测试套件..."

# 设置测试环境变量
export RUST_LOG=debug
export DATABASE_URL="postgresql://test:test@localhost:5433/polyvisor_test"

# 启动测试数据库
echo "🗄️ 启动测试数据库..."
docker run -d --name polyvisor-test-db \
  -e POSTGRES_DB=polyvisor_test \
  -e POSTGRES_USER=test \
  -e POSTGRES_PASSWORD=test \
  -p 5433:5432 \
  postgres:15-alpine

# 等待数据库启动
sleep 5

# 运行 Rust 测试
echo "🦀 运行 Rust 单元测试..."
cargo test --workspace

# 运行智能合约测试
echo "📄 运行智能合约测试..."
cd contracts/analytics && cargo test
cd ../zkproof && cargo test  
cd ../privacy && cargo test
cd ../..

# 运行前端测试
echo "⚛️ 运行前端测试..."
cd frontend
npm test -- --coverage --watchAll=false
cd ..

# 运行集成测试
echo "🔗 运行集成测试..."
cd tests
cargo test --test integration_tests
cd ..

# 清理测试环境
echo "🧹 清理测试环境..."
docker stop polyvisor-test-db
docker rm polyvisor-test-db

echo "✅ 所有测试完成！"