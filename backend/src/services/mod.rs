pub mod health_service;
pub mod privacy_service;
pub mod zkproof_service;
pub mod contributor_service;

use anyhow::Result;
use std::sync::Arc;

use crate::{config::AppConfig, database::Database};

/// 业务服务集合
pub struct Services {
    /// 网络健康度服务
    pub health_service: Arc<health_service::HealthService>,
    /// 隐私保护服务
    pub privacy_service: Arc<privacy_service::PrivacyService>,
    /// 零知识证明服务
    pub zkproof_service: Arc<zkproof_service::ZKProofService>,
    /// 贡献者管理服务
    pub contributor_service: Arc<contributor_service::ContributorService>,
}

impl Services {
    /// 创建新的服务集合实例
    pub async fn new(
        database: Arc<Database>,
        config: Arc<AppConfig>,
    ) -> Result<Self> {
        // 初始化各个服务
        let health_service = Arc::new(
            health_service::HealthService::new(database.clone(), config.clone()).await?
        );
        
        let privacy_service = Arc::new(
            privacy_service::PrivacyService::new(database.clone(), config.clone()).await?
        );
        
        let zkproof_service = Arc::new(
            zkproof_service::ZKProofService::new(database.clone(), config.clone()).await?
        );
        
        let contributor_service = Arc::new(
            contributor_service::ContributorService::new(database.clone(), config.clone()).await?
        );

        Ok(Self {
            health_service,
            privacy_service,
            zkproof_service,
            contributor_service,
        })
    }

    /// 健康检查所有服务
    pub async fn health_check(&self) -> Result<()> {
        // 检查各个服务的健康状态
        self.health_service.health_check().await?;
        self.privacy_service.health_check().await?;
        self.zkproof_service.health_check().await?;
        self.contributor_service.health_check().await?;

        Ok(())
    }

    /// 关闭所有服务
    pub async fn shutdown(&self) -> Result<()> {
        // 优雅关闭各个服务
        if let Err(e) = self.health_service.shutdown().await {
            tracing::warn!("健康度服务关闭失败: {}", e);
        }
        
        if let Err(e) = self.privacy_service.shutdown().await {
            tracing::warn!("隐私服务关闭失败: {}", e);
        }
        
        if let Err(e) = self.zkproof_service.shutdown().await {
            tracing::warn!("零知识证明服务关闭失败: {}", e);
        }
        
        if let Err(e) = self.contributor_service.shutdown().await {
            tracing::warn!("贡献者服务关闭失败: {}", e);
        }

        Ok(())
    }
}