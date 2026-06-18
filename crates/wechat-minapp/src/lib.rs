//! 微信小程序服务端常用接口的 RUST SDK
//!
//! [actix web + 小程序端 完整示例](https://github.com/ikeeplearn/wechat-minapp/tree/master/examples)
//!
//! # 功能
//!
//! - 获取访问令牌
//! - 用户登录凭证校验
//! - 解析用户信息
//! - 获取用户手机号
//! - 生成小程序码
//! - 内容安全检测
//! - 生成小程序链接
//! - 发送小程序模板消息
//!
//! # 特性
//! - 异步支持
//! - 丰富的接口支持
//! - HTTP 客户端和接口调用凭据存储读取方式分离，可以按自己的需求实现不同的 HTTP 客户端和接口调用凭据存储读取方式。
//! - 支持稳定版和普通版访问令牌
//! - 良好的错误处理
//! - 简单易用的 API
//! - 详细的文档
//! - 单元测试覆盖

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
pub mod link;
pub mod minapp_security;
pub mod new_type;
pub mod qr;
pub mod template_message;
pub mod user;

use std::sync::Arc;

/// 微信小程序 SDK 客户端，向后兼容
#[derive(Debug, Clone)]
pub struct WechatMinapp {
    pub core: WechatCore,
}

impl WechatMinapp {
    /// 使用默认配置创建客户端
    pub fn new(app_id: &str, secret: &str) -> Self {
        WechatMinapp {
            core: WechatCore::new(app_id, secret),
        }
    }

    /// 使用自定义配置创建客户端
    pub fn custom(http_client: Arc<dyn HttpClient>, token_storage: Arc<dyn TokenStorage>) -> Self {
        WechatMinapp {
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

// 向后兼容别名
pub type WechatMinappSDK = WechatMinapp;
pub type NormalToken = NonStableToken;

// 错误转换实现
impl From<crate::new_type::PagePathError> for Error {
    fn from(value: crate::new_type::PagePathError) -> Self {
        Error::InvalidParameter(value.to_string())
    }
}

impl From<crate::new_type::ValidationSceneError> for Error {
    fn from(value: crate::new_type::ValidationSceneError) -> Self {
        Error::InvalidParameter(value.to_string())
    }
}
