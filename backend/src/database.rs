use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Row};
use std::time::Duration;
use tracing::{info, warn};

/// 数据库连接和管理
#[derive(Debug, Clone)]
pub struct Database {
    /// PostgreSQL连接池
    pool: Pool<Postgres>,
}

impl Database {
    /// 创建新的数据库连接
    pub async fn new(database_url: &str) -> Result<Self> {
        info!("🔌 连接数据库: {}", database_url);
        
        let pool = PgPoolOptions::new()
            .max_connections(20)
            .min_connections(5)
            .acquire_timeout(Duration::from_secs(30))
            .idle_timeout(Duration::from_secs(600))
            .max_lifetime(Duration::from_secs(1800))
            .connect(database_url)
            .await
            .map_err(|e| anyhow::anyhow!("数据库连接失败: {}", e))?;

        info!("✅ 数据库连接成功");
        
        Ok(Self { pool })
    }

    /// 运行数据库迁移
    pub async fn migrate(&self) -> Result<()> {
        info!("🔄 开始数据库迁移...");
        
        // 创建表结构
        self.create_tables().await?;
        
        info!("✅ 数据库迁移完成");
        Ok(())
    }

    /// 检查数据库连接状态
    pub async fn is_connected(&self) -> bool {
        match sqlx::query("SELECT 1").fetch_one(&self.pool).await {
            Ok(_) => true,
            Err(e) => {
                warn!("数据库连接检查失败: {}", e);
                false
            }
        }
    }

    /// 获取连接池引用
    pub fn pool(&self) -> &Pool<Postgres> {
        &self.pool
    }

    /// 创建数据库表结构
    async fn create_tables(&self) -> Result<()> {
        // 创建网络指标表
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS network_metrics (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                metric_type VARCHAR(50) NOT NULL,
                value NUMERIC NOT NULL,
                quality_score SMALLINT NOT NULL CHECK (quality_score >= 0 AND quality_score <= 100),
                privacy_level VARCHAR(20) NOT NULL,
                proof_id VARCHAR(64),
                source_node TEXT,
                data_sources JSONB,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
            );
            "#,
        )
        .execute(&self.pool)
        .await?;

        // 创建零知识证明表
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS zk_proofs (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                proof_id VARCHAR(64) UNIQUE NOT NULL,
                circuit_id INTEGER NOT NULL,
                proof_data BYTEA NOT NULL,
                public_inputs JSONB NOT NULL,
                verification_key BYTEA NOT NULL,
                verified BOOLEAN DEFAULT FALSE,
                verification_time TIMESTAMP WITH TIME ZONE,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
            );
            "#,
        )
        .execute(&self.pool)
        .await?;

        // 创建用户隐私设置表
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS user_privacy_settings (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                user_address TEXT UNIQUE NOT NULL,
                privacy_level VARCHAR(20) NOT NULL DEFAULT 'high',
                data_sharing_consent BOOLEAN DEFAULT FALSE,
                analytics_participation BOOLEAN DEFAULT FALSE,
                settings JSONB,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
            );
            "#,
        )
        .execute(&self.pool)
        .await?;

        // 创建数据贡献者表
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS contributors (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                address TEXT UNIQUE NOT NULL,
                total_contributions INTEGER DEFAULT 0,
                data_quality_average NUMERIC(5,2) DEFAULT 0.0,
                reputation_score INTEGER DEFAULT 0,
                verification_count INTEGER DEFAULT 0,
                last_contribution TIMESTAMP WITH TIME ZONE,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
            );
            "#,
        )
        .execute(&self.pool)
        .await?;

        // 创建可信节点表
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS trusted_nodes (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                address TEXT UNIQUE NOT NULL,
                node_type VARCHAR(50) NOT NULL,
                reliability_score SMALLINT NOT NULL CHECK (reliability_score >= 0 AND reliability_score <= 100),
                status VARCHAR(20) DEFAULT 'active',
                metadata JSONB,
                registered_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                last_seen TIMESTAMP WITH TIME ZONE DEFAULT NOW()
            );
            "#,
        )
        .execute(&self.pool)
        .await?;

        // 创建网络健康度历史表
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS network_health_history (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                overall_score NUMERIC(5,2) NOT NULL,
                block_time_score NUMERIC(5,2) NOT NULL,
                transaction_score NUMERIC(5,2) NOT NULL,
                validator_score NUMERIC(5,2) NOT NULL,
                congestion_score NUMERIC(5,2) NOT NULL,
                data_freshness SMALLINT NOT NULL,
                metadata JSONB,
                recorded_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
            );
            "#,
        )
        .execute(&self.pool)
        .await?;

        // 创建索引以提升查询性能
        self.create_indexes().await?;

        Ok(())
    }

    /// 创建数据库索引
    async fn create_indexes(&self) -> Result<()> {
        let indexes = vec![
            "CREATE INDEX IF NOT EXISTS idx_network_metrics_type ON network_metrics(metric_type);",
            "CREATE INDEX IF NOT EXISTS idx_network_metrics_created_at ON network_metrics(created_at);",
            "CREATE INDEX IF NOT EXISTS idx_zk_proofs_proof_id ON zk_proofs(proof_id);",
            "CREATE INDEX IF NOT EXISTS idx_zk_proofs_circuit_id ON zk_proofs(circuit_id);",
            "CREATE INDEX IF NOT EXISTS idx_user_privacy_user_address ON user_privacy_settings(user_address);",
            "CREATE INDEX IF NOT EXISTS idx_contributors_address ON contributors(address);",
            "CREATE INDEX IF NOT EXISTS idx_trusted_nodes_address ON trusted_nodes(address);",
            "CREATE INDEX IF NOT EXISTS idx_trusted_nodes_status ON trusted_nodes(status);",
            "CREATE INDEX IF NOT EXISTS idx_network_health_recorded_at ON network_health_history(recorded_at);",
        ];

        for index_sql in indexes {
            sqlx::query(index_sql).execute(&self.pool).await?;
        }

        Ok(())
    }

    /// 清理过期数据
    pub async fn cleanup_expired_data(&self, days: i32) -> Result<()> {
        info!("🧹 清理{}天前的过期数据...", days);
        
        // 清理过期的网络指标数据
        let result = sqlx::query(
            "DELETE FROM network_metrics WHERE created_at < NOW() - INTERVAL '{} days'"
        )
        .bind(days)
        .execute(&self.pool)
        .await?;
        
        info!("🗑️ 清理了 {} 条过期网络指标记录", result.rows_affected());
        
        // 清理过期的网络健康度历史数据
        let result = sqlx::query(
            "DELETE FROM network_health_history WHERE recorded_at < NOW() - INTERVAL '{} days'"
        )
        .bind(days)
        .execute(&self.pool)
        .await?;
        
        info!("🗑️ 清理了 {} 条过期健康度历史记录", result.rows_affected());
        
        Ok(())
    }

    /// 获取数据库统计信息
    pub async fn get_statistics(&self) -> Result<DatabaseStatistics> {
        let metrics_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM network_metrics")
            .fetch_one(&self.pool)
            .await?;

        let proofs_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM zk_proofs")
            .fetch_one(&self.pool)
            .await?;

        let users_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM user_privacy_settings")
            .fetch_one(&self.pool)
            .await?;

        let contributors_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM contributors")
            .fetch_one(&self.pool)
            .await?;

        let trusted_nodes_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM trusted_nodes WHERE status = 'active'"
        )
        .fetch_one(&self.pool)
        .await?;

        // 获取数据库大小
        let db_size: Option<i64> = sqlx::query_scalar(
            "SELECT pg_database_size(current_database())"
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(DatabaseStatistics {
            total_metrics: metrics_count as u64,
            total_proofs: proofs_count as u64,
            total_users: users_count as u64,
            total_contributors: contributors_count as u64,
            active_trusted_nodes: trusted_nodes_count as u64,
            database_size_bytes: db_size.unwrap_or(0) as u64,
            last_updated: chrono::Utc::now(),
        })
    }
}

/// 数据库统计信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DatabaseStatistics {
    /// 总网络指标数量
    pub total_metrics: u64,
    /// 总零知识证明数量
    pub total_proofs: u64,
    /// 总用户数量
    pub total_users: u64,
    /// 总贡献者数量
    pub total_contributors: u64,
    /// 活跃可信节点数量
    pub active_trusted_nodes: u64,
    /// 数据库大小（字节）
    pub database_size_bytes: u64,
    /// 最后更新时间
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_database_connection() {
        // 只有在有测试数据库时才运行
        if let Ok(db_url) = std::env::var("TEST_DATABASE_URL") {
            let db = Database::new(&db_url).await;
            assert!(db.is_ok());
            
            if let Ok(database) = db {
                assert!(database.is_connected().await);
            }
        }
    }
}"