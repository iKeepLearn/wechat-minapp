//! 接口调用凭据存储读取模块
//! 默认使用内存Arc结构,可参考实现读取保存方式，比如 redis、postgresql、mysql 等。
//! 
//! 

use super::access_token::{AccessToken, is_token_expired};
use super::token_type::TokenType;
use crate::Result;
use async_trait::async_trait;
use chrono::Utc;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use tokio::sync::{Notify, RwLock};
use tracing::debug;


/// 定义接口调用凭据读取存储的行为
#[async_trait]
pub trait TokenStorage: Send + Sync {
    async fn token(&self) -> Result<String>;
    async fn refresh_access_token(&self) -> Result<String>;
    fn token_type(&self) -> Arc<dyn TokenType>;
}

/// 接口调用凭据内存存储结构
pub struct MemoryTokenStorage {
    access_token: Arc<RwLock<AccessToken>>,
    refreshing: Arc<AtomicBool>,
    notify: Arc<Notify>,
    token_type: Arc<dyn TokenType>,
}

impl MemoryTokenStorage {
    pub fn new(token_type: Arc<dyn TokenType>) -> Self {
        MemoryTokenStorage {
            access_token: Arc::new(RwLock::new(AccessToken {
                access_token: String::new(),
                expired_at: Utc::now(),
            })),
            refreshing: Arc::new(AtomicBool::new(false)),
            notify: Arc::new(Notify::new()),
            token_type,
        }
    }
}


/// 内存存储方式的接口调用凭据存储读取实现
#[async_trait]
impl TokenStorage for MemoryTokenStorage {
    /// 获取接口调用凭据
    async fn token(&self) -> Result<String> {
        // 第一次检查：快速路径
        {
            let guard = self.access_token.read().await;
            if !is_token_expired(&guard) {
                return Ok(guard.access_token.clone());
            }
        }

        // 使用CAS竞争刷新权
        if self
            .refreshing
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            // 获得刷新权
            match self.refresh_access_token().await {
                Ok(token) => {
                    self.refreshing.store(false, Ordering::Release);
                    self.notify.notify_waiters();
                    Ok(token)
                }
                Err(e) => {
                    self.refreshing.store(false, Ordering::Release);
                    self.notify.notify_waiters();
                    Err(e)
                }
            }
        } else {
            // 等待其他线程刷新完成
            self.notify.notified().await;
            // 刷新完成后重新读取
            let guard = self.access_token.read().await;
            Ok(guard.access_token.clone())
        }
    }

    /// 刷新接口调用凭据
    async fn refresh_access_token(&self) -> Result<String> {
        let mut guard = self.access_token.write().await;

        if !is_token_expired(&guard) {
            debug!("token already refreshed by another thread");
            return Ok(guard.access_token.clone());
        }

        debug!("performing network request to refresh token");

        let builder = self.token_type.token().await?;

        guard.access_token = builder.access_token.clone();
        guard.expired_at = builder.expired_at;

        debug!("fresh access token: {:#?}", guard);

        Ok(guard.access_token.clone())
    }

    fn token_type(&self) -> Arc<dyn TokenType> {
        self.token_type.clone()
    }
}
