# PolyVisor 部署指南

## 🚀 部署概览

PolyVisor支持多种部署方式，从本地开发到生产环境的完整部署方案。本指南将详细介绍各种部署选项和最佳实践。

## 📋 部署前准备

### 系统要求

#### 最低配置
- **CPU**: 4核心
- **内存**: 8GB RAM
- **存储**: 100GB SSD
- **网络**: 100Mbps带宽
- **操作系统**: Ubuntu 20.04+, CentOS 8+, 或 macOS 11+

#### 推荐配置
- **CPU**: 8核心
- **内存**: 16GB RAM
- **存储**: 500GB SSD
- **网络**: 1Gbps带宽
- **操作系统**: Ubuntu 22.04 LTS

#### 生产环境配置
- **CPU**: 16核心+
- **内存**: 32GB RAM+
- **存储**: 1TB NVMe SSD
- **网络**: 10Gbps带宽
- **操作系统**: Ubuntu 22.04 LTS

### 依赖安装

#### Docker和Docker Compose
```bash
# Ubuntu/Debian
sudo apt update
sudo apt install docker.io docker-compose
sudo usermod -aG docker $USER

# CentOS/RHEL
sudo yum install docker docker-compose
sudo systemctl enable docker
sudo systemctl start docker

# macOS
brew install docker docker-compose
```

#### Kubernetes (可选)
```bash
# 安装kubectl
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl"
sudo install -o root -g root -m 0755 kubectl /usr/local/bin/kubectl

# 安装Helm
curl https://get.helm.sh/helm-v3.12.0-linux-amd64.tar.gz | tar xz
sudo mv linux-amd64/helm /usr/local/bin/
```

## 🐳 Docker部署

### 1. 快速部署

#### 获取代码
```bash
git clone https://github.com/your-org/polyvisor.git
cd polyvisor
```

#### 环境配置
```bash
# 复制环境变量模板
cp .env.example .env

# 编辑环境变量
nano .env
```

#### 一键启动
```bash
# 构建并启动所有服务
docker-compose up -d

# 查看服务状态
docker-compose ps

# 查看日志
docker-compose logs -f
```

### 2. 生产环境部署

#### 环境变量配置
```bash
# .env文件示例
NODE_ENV=production
DATABASE_URL=postgresql://polyvisor:${DB_PASSWORD}@postgres:5432/polyvisor
REDIS_URL=redis://redis:6379
JWT_SECRET=${JWT_SECRET}
SUBSTRATE_RPC_URL=ws://substrate-node:9944

# 安全配置
POSTGRES_PASSWORD=${DB_PASSWORD}
REDIS_PASSWORD=${REDIS_PASSWORD}

# SSL配置
SSL_CERT_PATH=/etc/ssl/certs/polyvisor.crt
SSL_KEY_PATH=/etc/ssl/private/polyvisor.key

# 监控配置
PROMETHEUS_ENABLED=true
GRAFANA_ADMIN_PASSWORD=${GRAFANA_PASSWORD}
```

#### SSL证书配置
```bash
# 创建SSL目录
mkdir -p nginx/ssl

# 使用Let's Encrypt获取证书
sudo apt install certbot
sudo certbot certonly --standalone -d your-domain.com

# 复制证书文件
sudo cp /etc/letsencrypt/live/your-domain.com/fullchain.pem nginx/ssl/
sudo cp /etc/letsencrypt/live/your-domain.com/privkey.pem nginx/ssl/
```

#### 生产环境Docker Compose
```yaml
version: '3.8'

services:
  # 数据库服务
  postgres:
    image: postgres:15-alpine
    environment:
      POSTGRES_DB: polyvisor
      POSTGRES_USER: polyvisor
      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD}
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./scripts/init-db.sql:/docker-entrypoint-initdb.d/01-init.sql
    restart: unless-stopped
    deploy:
      resources:
        limits:
          memory: 4G
        reservations:
          memory: 2G

  # Redis缓存
  redis:
    image: redis:7-alpine
    command: redis-server --requirepass ${REDIS_PASSWORD}
    volumes:
      - redis_data:/data
    restart: unless-stopped
    deploy:
      resources:
        limits:
          memory: 1G

  # Substrate节点
  substrate-node:
    build:
      context: ./blockchain
      dockerfile: Dockerfile
    ports:
      - "9944:9944"
      - "9933:9933"
    volumes:
      - substrate_data:/var/lib/substrate
    restart: unless-stopped
    deploy:
      resources:
        limits:
          memory: 8G
        reservations:
          memory: 4G

  # API网关
  api-gateway:
    build:
      context: ./backend/api-gateway
      dockerfile: Dockerfile
    environment:
      DATABASE_URL: ${DATABASE_URL}
      REDIS_URL: ${REDIS_URL}
      JWT_SECRET: ${JWT_SECRET}
    depends_on:
      - postgres
      - redis
      - substrate-node
    restart: unless-stopped
    deploy:
      replicas: 2
      resources:
        limits:
          memory: 2G

  # 前端应用
  frontend:
    build:
      context: ./frontend
      dockerfile: Dockerfile.prod
    environment:
      NODE_ENV: production
    restart: unless-stopped

  # Nginx负载均衡
  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx/nginx.prod.conf:/etc/nginx/nginx.conf
      - ./nginx/ssl:/etc/nginx/ssl
    depends_on:
      - frontend
      - api-gateway
    restart: unless-stopped

volumes:
  postgres_data:
  redis_data:
  substrate_data:
```

### 3. 服务扩展

#### 水平扩展
```bash
# 扩展API网关实例
docker-compose up -d --scale api-gateway=3

# 扩展前端实例
docker-compose up -d --scale frontend=2
```

#### 负载均衡配置
```nginx
upstream api_backend {
    server api-gateway_1:8080;
    server api-gateway_2:8080;
    server api-gateway_3:8080;
}

upstream frontend_backend {
    server frontend_1:3000;
    server frontend_2:3000;
}
```

## ☸️ Kubernetes部署

### 1. 集群准备

#### 创建命名空间
```yaml
apiVersion: v1
kind: Namespace
metadata:
  name: polyvisor
```

#### 配置Secret
```yaml
apiVersion: v1
kind: Secret
metadata:
  name: polyvisor-secrets
  namespace: polyvisor
type: Opaque
data:
  database-password: <base64-encoded-password>
  jwt-secret: <base64-encoded-secret>
  redis-password: <base64-encoded-password>
```

### 2. 数据库部署

#### PostgreSQL StatefulSet
```yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: postgres
  namespace: polyvisor
spec:
  serviceName: postgres
  replicas: 1
  selector:
    matchLabels:
      app: postgres
  template:
    metadata:
      labels:
        app: postgres
    spec:
      containers:
      - name: postgres
        image: postgres:15-alpine
        env:
        - name: POSTGRES_DB
          value: polyvisor
        - name: POSTGRES_USER
          value: polyvisor
        - name: POSTGRES_PASSWORD
          valueFrom:
            secretKeyRef:
              name: polyvisor-secrets
              key: database-password
        ports:
        - containerPort: 5432
        volumeMounts:
        - name: postgres-storage
          mountPath: /var/lib/postgresql/data
        resources:
          requests:
            memory: "2Gi"
            cpu: "500m"
          limits:
            memory: "4Gi"
            cpu: "2"
  volumeClaimTemplates:
  - metadata:
      name: postgres-storage
    spec:
      accessModes: ["ReadWriteOnce"]
      resources:
        requests:
          storage: 100Gi
```

### 3. 应用部署

#### API网关Deployment
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: api-gateway
  namespace: polyvisor
spec:
  replicas: 3
  selector:
    matchLabels:
      app: api-gateway
  template:
    metadata:
      labels:
        app: api-gateway
    spec:
      containers:
      - name: api-gateway
        image: polyvisor/api-gateway:latest
        env:
        - name: DATABASE_URL
          value: "postgresql://polyvisor:$(DATABASE_PASSWORD)@postgres:5432/polyvisor"
        - name: DATABASE_PASSWORD
          valueFrom:
            secretKeyRef:
              name: polyvisor-secrets
              key: database-password
        ports:
        - containerPort: 8080
        resources:
          requests:
            memory: "1Gi"
            cpu: "500m"
          limits:
            memory: "2Gi"
            cpu: "1"
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /ready
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
```

#### 前端Deployment
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: frontend
  namespace: polyvisor
spec:
  replicas: 2
  selector:
    matchLabels:
      app: frontend
  template:
    metadata:
      labels:
        app: frontend
    spec:
      containers:
      - name: frontend
        image: polyvisor/frontend:latest
        ports:
        - containerPort: 3000
        resources:
          requests:
            memory: "512Mi"
            cpu: "250m"
          limits:
            memory: "1Gi"
            cpu: "500m"
```

### 4. 服务暴露

#### Service配置
```yaml
apiVersion: v1
kind: Service
metadata:
  name: api-gateway-service
  namespace: polyvisor
spec:
  selector:
    app: api-gateway
  ports:
  - port: 8080
    targetPort: 8080
  type: ClusterIP

---
apiVersion: v1
kind: Service
metadata:
  name: frontend-service
  namespace: polyvisor
spec:
  selector:
    app: frontend
  ports:
  - port: 3000
    targetPort: 3000
  type: ClusterIP
```

#### Ingress配置
```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: polyvisor-ingress
  namespace: polyvisor
  annotations:
    nginx.ingress.kubernetes.io/rewrite-target: /
    cert-manager.io/cluster-issuer: letsencrypt-prod
spec:
  tls:
  - hosts:
    - polyvisor.your-domain.com
    secretName: polyvisor-tls
  rules:
  - host: polyvisor.your-domain.com
    http:
      paths:
      - path: /api
        pathType: Prefix
        backend:
          service:
            name: api-gateway-service
            port:
              number: 8080
      - path: /
        pathType: Prefix
        backend:
          service:
            name: frontend-service
            port:
              number: 3000
```

## 🔧 配置优化

### 数据库优化

#### PostgreSQL配置
```sql
-- postgresql.conf优化
shared_buffers = 4GB
effective_cache_size = 12GB
maintenance_work_mem = 1GB
checkpoint_completion_target = 0.9
wal_buffers = 64MB
default_statistics_target = 100
random_page_cost = 1.1
effective_io_concurrency = 200
```

#### 连接池配置
```toml
[database]
max_connections = 100
min_connections = 10
connection_timeout = 30
idle_timeout = 600
max_lifetime = 1800
```

### Redis优化

#### Redis配置
```conf
# redis.conf
maxmemory 4gb
maxmemory-policy allkeys-lru
save 900 1
save 300 10
save 60 10000
```

### Nginx优化

#### 生产环境配置
```nginx
worker_processes auto;
worker_rlimit_nofile 65535;

events {
    worker_connections 65535;
    use epoll;
    multi_accept on;
}

http {
    sendfile on;
    tcp_nopush on;
    tcp_nodelay on;
    keepalive_timeout 65;
    types_hash_max_size 2048;
    
    # 压缩配置
    gzip on;
    gzip_vary on;
    gzip_min_length 1024;
    gzip_comp_level 6;
    gzip_types
        text/plain
        text/css
        text/xml
        text/javascript
        application/json
        application/javascript
        application/xml+rss;
    
    # 缓存配置
    location ~* \.(jpg|jpeg|png|gif|ico|css|js|woff2)$ {
        expires 1y;
        add_header Cache-Control "public, immutable";
    }
    
    # 安全头
    add_header X-Frame-Options DENY;
    add_header X-Content-Type-Options nosniff;
    add_header X-XSS-Protection "1; mode=block";
    add_header Strict-Transport-Security "max-age=31536000";
}
```

## 📊 监控和日志

### Prometheus监控

#### 监控配置
```yaml
global:
  scrape_interval: 15s
  evaluation_interval: 15s

scrape_configs:
  - job_name: 'polyvisor-api'
    static_configs:
      - targets: ['api-gateway:8080']
    metrics_path: /metrics
    scrape_interval: 10s
    
  - job_name: 'polyvisor-substrate'
    static_configs:
      - targets: ['substrate-node:9615']
    metrics_path: /metrics
    
  - job_name: 'postgres'
    static_configs:
      - targets: ['postgres:5432']
```

### Grafana仪表板

#### 数据源配置
```yaml
apiVersion: 1
datasources:
  - name: Prometheus
    type: prometheus
    access: proxy
    url: http://prometheus:9090
    isDefault: true
```

### 日志聚合

#### ELK Stack配置
```yaml
version: '3.8'
services:
  elasticsearch:
    image: elasticsearch:8.5.0
    environment:
      - discovery.type=single-node
      - "ES_JAVA_OPTS=-Xms2g -Xmx2g"
    volumes:
      - elastic_data:/usr/share/elasticsearch/data
      
  logstash:
    image: logstash:8.5.0
    volumes:
      - ./logstash/pipeline:/usr/share/logstash/pipeline
      
  kibana:
    image: kibana:8.5.0
    ports:
      - "5601:5601"
    environment:
      ELASTICSEARCH_HOSTS: http://elasticsearch:9200
```

## 🔐 安全配置

### SSL/TLS配置

#### 自动证书更新
```bash
#!/bin/bash
# renew-certs.sh
certbot renew --quiet
docker-compose exec nginx nginx -s reload
```

#### 定时任务
```bash
# 添加到crontab
0 2 * * 1 /path/to/renew-certs.sh
```

### 防火墙配置

#### UFW配置
```bash
# 允许SSH
sudo ufw allow ssh

# 允许HTTP/HTTPS
sudo ufw allow 80
sudo ufw allow 443

# 允许特定服务端口
sudo ufw allow 9944  # Substrate RPC
sudo ufw allow 9933  # Substrate HTTP

# 启用防火墙
sudo ufw enable
```

### 网络安全

#### fail2ban配置
```ini
[sshd]
enabled = true
port = ssh
filter = sshd
logpath = /var/log/auth.log
maxretry = 3
bantime = 3600

[nginx-http-auth]
enabled = true
filter = nginx-http-auth
port = http,https
logpath = /var/log/nginx/error.log
maxretry = 5
bantime = 600
```

## 🚀 部署脚本

### 自动化部署脚本
```bash
#!/bin/bash
# deploy.sh

set -e

echo "🚀 开始部署PolyVisor..."

# 检查依赖
command -v docker >/dev/null 2>&1 || { echo "Docker未安装"; exit 1; }
command -v docker-compose >/dev/null 2>&1 || { echo "Docker Compose未安装"; exit 1; }

# 拉取最新代码
git pull origin main

# 构建镜像
echo "📦 构建Docker镜像..."
docker-compose build --no-cache

# 启动服务
echo "🔄 启动服务..."
docker-compose down
docker-compose up -d

# 等待服务启动
echo "⏳ 等待服务启动..."
sleep 30

# 健康检查
echo "🏥 执行健康检查..."
curl -f http://localhost:8080/health || { echo "API健康检查失败"; exit 1; }
curl -f http://localhost:3000 || { echo "前端健康检查失败"; exit 1; }

echo "✅ 部署完成！"
echo "🌐 访问地址: http://localhost:3000"
```

### 回滚脚本
```bash
#!/bin/bash
# rollback.sh

set -e

echo "🔄 开始回滚..."

# 获取上一个版本
PREVIOUS_TAG=$(git describe --tags --abbrev=0 HEAD^)

# 检出上一个版本
git checkout $PREVIOUS_TAG

# 重新部署
./deploy.sh

echo "✅ 回滚完成到版本: $PREVIOUS_TAG"
```

## 📝 部署检查清单

### 部署前检查
- [ ] 系统资源充足
- [ ] 依赖软件已安装
- [ ] 环境变量已配置
- [ ] SSL证书已准备
- [ ] 数据库备份完成
- [ ] 防火墙规则已设置

### 部署后验证
- [ ] 所有服务正常启动
- [ ] 健康检查通过
- [ ] 监控系统正常
- [ ] 日志正常输出
- [ ] 用户访问正常
- [ ] 性能指标正常

### 回滚准备
- [ ] 回滚脚本已测试
- [ ] 数据备份可用
- [ ] 回滚流程已演练
- [ ] 通知机制已准备

---

🎯 **部署成功**: 遵循本指南可以确保PolyVisor在各种环境中稳定运行。如遇问题请参考故障排除指南或联系技术支持。