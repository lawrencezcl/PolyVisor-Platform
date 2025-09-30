use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::env;

/// 应用配置结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// 服务器配置
    pub server: ServerConfig,
    /// 数据库配置
    pub database_url: String,
    /// Redis配置
    pub redis_url: String,
    /// 区块链配置
    pub blockchain: BlockchainConfig,
    /// 零知识证明配置
    pub zkproof: ZKProofConfig,
    /// 日志配置
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// 服务器监听地址
    pub host: String,
    /// 服务器端口
    pub port: u16,
    /// 是否启用TLS
    pub enable_tls: bool,
    /// 请求超时时间（秒）
    pub request_timeout: u64,
    /// 最大并发连接数
    pub max_connections: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainConfig {
    /// WebSocket连接URL
    pub ws_url: String,
    /// HTTP RPC连接URL
    pub rpc_url: String,
    /// 连接超时时间（秒）
    pub connection_timeout: u64,
    /// 重连间隔（秒）
    pub reconnect_interval: u64,
    /// 最大重连次数
    pub max_reconnect_attempts: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZKProofConfig {
    /// 是否启用证明缓存
    pub enable_cache: bool,
    /// 缓存过期时间（秒）
    pub cache_ttl: u64,
    /// 最大缓存大小
    pub max_cache_size: usize,
    /// 批量验证大小
    pub batch_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// 日志级别
    pub level: String,
    /// 是否启用JSON格式
    pub json_format: bool,
    /// 日志文件路径（可选）
    pub file_path: Option<String>,
}

impl AppConfig {
    /// 从环境变量加载配置
    pub fn from_env() -> Result<Self> {
        dotenv::dotenv().ok(); // 忽略.env文件不存在的错误

        let config = Self {
            server: ServerConfig {
                host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "8080".to_string())
                    .parse()
                    .unwrap_or(8080),
                enable_tls: env::var("ENABLE_TLS")
                    .unwrap_or_else(|_| "false".to_string())
                    .parse()
                    .unwrap_or(false),
                request_timeout: env::var("REQUEST_TIMEOUT")
                    .unwrap_or_else(|_| "30".to_string())
                    .parse()
                    .unwrap_or(30),
                max_connections: env::var("MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "1000".to_string())
                    .parse()
                    .unwrap_or(1000),
            },
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| {
                    "postgresql://polyvisor:password@localhost:5432/polyvisor".to_string()
                }),
            redis_url: env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            blockchain: BlockchainConfig {
                ws_url: env::var("BLOCKCHAIN_WS_URL")
                    .unwrap_or_else(|_| "ws://localhost:9944".to_string()),
                rpc_url: env::var("BLOCKCHAIN_RPC_URL")
                    .unwrap_or_else(|_| "http://localhost:9933".to_string()),
                connection_timeout: env::var("BLOCKCHAIN_TIMEOUT")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()
                    .unwrap_or(10),
                reconnect_interval: env::var("RECONNECT_INTERVAL")
                    .unwrap_or_else(|_| "5".to_string())
                    .parse()
                    .unwrap_or(5),
                max_reconnect_attempts: env::var("MAX_RECONNECT_ATTEMPTS")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()
                    .unwrap_or(10),
            },
            zkproof: ZKProofConfig {
                enable_cache: env::var("ZKPROOF_ENABLE_CACHE")
                    .unwrap_or_else(|_| "true".to_string())
                    .parse()
                    .unwrap_or(true),
                cache_ttl: env::var("ZKPROOF_CACHE_TTL")
                    .unwrap_or_else(|_| "3600".to_string())
                    .parse()
                    .unwrap_or(3600),
                max_cache_size: env::var("ZKPROOF_MAX_CACHE_SIZE")
                    .unwrap_or_else(|_| "1000".to_string())
                    .parse()
                    .unwrap_or(1000),
                batch_size: env::var("ZKPROOF_BATCH_SIZE")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()
                    .unwrap_or(10),
            },
            logging: LoggingConfig {
                level: env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
                json_format: env::var("LOG_JSON_FORMAT")
                    .unwrap_or_else(|_| "false".to_string())
                    .parse()
                    .unwrap_or(false),
                file_path: env::var("LOG_FILE_PATH").ok(),
            },
        };

        Ok(config)
    }

    /// 验证配置有效性
    pub fn validate(&self) -> Result<()> {
        // 验证服务器配置
        if self.server.port == 0 {
            return Err(anyhow::anyhow!("Server port cannot be 0"));
        }

        if self.server.max_connections == 0 {
            return Err(anyhow::anyhow!("Max connections cannot be 0"));
        }

        // 验证数据库URL
        if !self.database_url.starts_with("postgresql://") {
            return Err(anyhow::anyhow!("Invalid database URL format"));
        }

        // 验证Redis URL
        if !self.redis_url.starts_with("redis://") {
            return Err(anyhow::anyhow!("Invalid Redis URL format"));
        }

        // 验证区块链配置
        if !self.blockchain.ws_url.starts_with("ws://") && !self.blockchain.ws_url.starts_with("wss://") {
            return Err(anyhow::anyhow!("Invalid blockchain WebSocket URL format"));
        }

        Ok(())
    }

    /// 获取数据库最大连接数
    pub fn get_db_max_connections(&self) -> u32 {
        (self.server.max_connections / 10).max(5) as u32
    }

    /// 获取Redis连接池大小
    pub fn get_redis_pool_size(&self) -> u32 {
        (self.server.max_connections / 20).max(2) as u32
    }

    /// 是否为开发环境
    pub fn is_development(&self) -> bool {
        env::var("ENVIRONMENT")
            .unwrap_or_else(|_| "development".to_string())
            .to_lowercase() == "development"
    }

    /// 是否为生产环境
    pub fn is_production(&self) -> bool {
        env::var("ENVIRONMENT")
            .unwrap_or_else(|_| "development".to_string())
            .to_lowercase() == "production"
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8080,
                enable_tls: false,
                request_timeout: 30,
                max_connections: 1000,
            },
            database_url: "postgresql://polyvisor:password@localhost:5432/polyvisor".to_string(),
            redis_url: "redis://localhost:6379".to_string(),
            blockchain: BlockchainConfig {
                ws_url: "ws://localhost:9944".to_string(),
                rpc_url: "http://localhost:9933".to_string(),
                connection_timeout: 10,
                reconnect_interval: 5,
                max_reconnect_attempts: 10,
            },
            zkproof: ZKProofConfig {
                enable_cache: true,
                cache_ttl: 3600,
                max_cache_size: 1000,
                batch_size: 10,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                json_format: false,
                file_path: None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.server.host, "0.0.0.0");
        assert_eq!(config.server.port, 8080);
        assert!(!config.server.enable_tls);
    }

    #[test]
    fn test_config_validation() {
        let config = AppConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_config_validation() {
        let mut config = AppConfig::default();
        config.server.port = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_db_max_connections() {
        let config = AppConfig::default();
        let max_conn = config.get_db_max_connections();
        assert!(max_conn >= 5);
    }

    #[test]
    fn test_redis_pool_size() {
        let config = AppConfig::default();
        let pool_size = config.get_redis_pool_size();
        assert!(pool_size >= 2);
    }
}"