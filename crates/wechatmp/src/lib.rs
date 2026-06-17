//! 微信服务号服务端常用接口的 RUST SDK
//!
//! # 功能
//!
//! - 获取访问令牌
//! - 发送服务号模板消息
//!
//! # 特性
//! - 异步支持
//! - 丰富的接口支持
//! - HTTP 客户端和接口调用凭据存储读取方式分离

// 重新导出 core 的内容
pub use wechat_core::{
    client::{
        HttpClient, MemoryTokenStorage, NonStableToken, ReqwestHttpClient, StableToken, TokenType,
        TokenStorage, WechatCore, AppConfig, AccessToken,
    },
    error::{Error, ErrorCode},
    utils::{RequestBuilder, ResponseExt, MpResponse, build_request, parse_query, parse_url},
    Result,
};

pub mod constants;
pub mod mp_message;

use std::sync::Arc;

/// 微信服务号 SDK 客户端
#[derive(Debug, Clone)]
pub struct WechatMp {
    pub core: WechatCore,
}

impl WechatMp {
    /// 使用默认配置创建客户端
    pub fn new(app_id: &str, secret: &str) -> Self {
        WechatMp {
            core: WechatCore::new(app_id, secret),
        }
    }

    /// 使用自定义配置创建客户端
    pub fn custom(http_client: Arc<dyn HttpClient>, token_storage: Arc<dyn TokenStorage>) -> Self {
        WechatMp {
            core: WechatCore::custom(http_client, token_storage),
        }
    }

    /// 获取 access token
    pub async fn token(&self) -> Result<String> {
        self.core.token().await
    }

    /// 获取 app config
    pub fn app_config(&self) -> AppConfig {
        self.core.app_config()
    }

    /// 获取内部 core client
    pub fn client(&self) -> &WechatCore {
        &self.core
    }
}
