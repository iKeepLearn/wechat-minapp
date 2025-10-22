//! 微信小程序服务端接口 Client 模块
//!
//! HTTP 客户端和令牌存储方式分离，可以按自己的需求实现不同的客户端和存储方式。
//! 支持稳定版接口调用凭据和普通版接口调用凭据。默认使用稳定版接口调用凭据。
//!
//! ## Traits
//! - [`HttpClient`]: 定义了 HTTP 客户端的行为,默认使用 [`ReqwestHttpClient`], 可参考实现其他 http client ,比如 `ureq` 。
//! - [`TokenStorage`]: 定义了接口调用凭据读取保存的行为，默认使用 [`MemoryTokenStorage`], 可参考实现读取保存方式，比如 `redis`、`postgresql`、`mysql` 等。
//! - [`TokenType`][token_type::TokenType]: 定义了接口调用凭据的行为，包括 [`StableToken`] (稳定版) 和 [`NonStableToken`] (普通版)。
//!
//! ## 默认客户端和存储方式
//!
//! 使用默认的 `reqwest` HTTP 客户端和内存 `Arc` 结构存储 Access Token。
//!
//! ```no_run
//! use wechat_minapp::client::WechatMinappSDK;
//!
//! let app_id = "your_app_id";
//! let secret = "your_app_secret";
//! // 默认使用 StableToken (稳定版接口调用凭据)
//! let client = WechatMinappSDK::new(app_id, secret);
//! ```
//!
//! ## 自定义 HTTP 客户端和存储方式
//!
//! 示例展示了如何使用默认的 `ReqwestHttpClient` 和 `MemoryTokenStorage` (稳定版 `StableToken`) 来构造自定义客户端。
//! 实际应用中，您可以替换为自己的实现。
//!
//! ```no_run
//! use std::sync::Arc;
//! use wechat_minapp::client::{MemoryTokenStorage, StableToken};
//! use wechat_minapp::client::{ReqwestHttpClient, WechatMinappSDK, HttpClient};
//!
//! let app_id = "your_app_id";
//! let secret = "your_app_secret";
//!
//! // 使用 reqwest 客户端
//! let http_client: Arc<dyn HttpClient> = Arc::new(ReqwestHttpClient::new());
//!
//! // 使用 StableToken (稳定版接口调用凭据)
//! let token_type = Arc::new(StableToken::new(
//!        app_id,
//!        secret,
//!        false, // 不强制刷新
//!        http_client.clone(),
//!    ));
//!
//! // 使用内存存储
//! let token_storage = Arc::new(MemoryTokenStorage::new(token_type));
//!
//! // 创建自定义客户端
//! let client = WechatMinappSDK::custom(http_client, token_storage);
//!
//! // ... 客户端现在可以使用了
//! ```
//!
mod access_token;
mod token_storage;
pub mod token_type;

pub use access_token::AccessToken;
pub use token_storage::{MemoryTokenStorage, TokenStorage};
pub use token_type::{NonStableToken, StableToken};
pub type WechatMinapp = WechatMinappSDK;

use crate::Result;
use async_trait::async_trait;
use http::{Request, Response};
use reqwest::Request as ReqwestRequest;
use std::{fmt, sync::Arc};

/// 微信小程序的 App ID 和 Secret 配置。
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub app_id: String,
    pub secret: String,
}

/// 微信小程序 SDK 主客户端结构。
///
/// 封装了 `HttpClient` 和 `TokenStorage`，提供发送请求和获取 Access Token 的方法。
///
pub struct WechatMinappSDK {
    pub client: Arc<dyn HttpClient>,
    pub token_storage: Arc<dyn TokenStorage>,
}

impl fmt::Debug for WechatMinappSDK {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WechatMinappSDK")
            .field("client", &"Arc<dyn HttpClient>")
            .field("token_storage", &"Arc<dyn TokenStorage>")
            .finish()
    }
}

impl Clone for WechatMinappSDK {
    fn clone(&self) -> Self {
        WechatMinappSDK {
            client: self.client.clone(),
            token_storage: self.token_storage.clone(),
        }
    }
}

impl WechatMinappSDK {
    /// 使用默认配置（`ReqwestHttpClient` 和 `MemoryTokenStorage` 与 `StableToken`）创建客户端。
    ///
    /// # 参数
    /// - `app_id`: 小程序 App ID。
    /// - `secret`: 小程序 App Secret。
    pub fn new(app_id: &str, secret: &str) -> Self {
        let http_client = Arc::new(ReqwestHttpClient::new());
        let token_type = Arc::new(StableToken::new(app_id, secret, false, http_client.clone()));
        let token_storage = Arc::new(MemoryTokenStorage::new(token_type));

        WechatMinappSDK {
            client: http_client,
            token_storage,
        }
    }

    /// 使用自定义的 `HttpClient` 和 `TokenStorage` 创建客户端。
    ///
    /// # 参数
    /// - `http_client`: 实现 [`HttpClient`] Trait 的实例。
    /// - `token_storage`: 实现 [`TokenStorage`] Trait 的实例。
    pub fn custom(http_client: Arc<dyn HttpClient>, token_storage: Arc<dyn TokenStorage>) -> Self {
        WechatMinappSDK {
            client: http_client,
            token_storage,
        }
    }

    /// 获取接口调用凭据（Access Token）。
    ///
    /// 此方法会通过 `TokenStorage` 自动处理 Token 的获取、缓存和刷新逻辑。
    ///
    /// # 返回
    /// Access Token 字符串的 Result。
    pub async fn token(&self) -> Result<String> {
        self.token_storage.token().await
    }

    /// 获取当前客户端的 App ID 和 Secret 配置。
    pub fn app_config(&self) -> AppConfig {
        self.token_storage.token_type().app_config()
    }
}

/// 定义 HTTP 客户端行为的 Trait。
///
/// 可根据爱好替换底层的 HTTP 实现。
#[async_trait]
pub trait HttpClient: Send + Sync {
    /// 执行一个 HTTP 请求并返回响应。
    ///
    /// # 参数
    /// - `request`: 要执行的 HTTP 请求。
    ///
    /// # 返回
    /// 包含 HTTP 响应的 Result。
    async fn execute(&self, request: Request<Vec<u8>>) -> Result<Response<Vec<u8>>>;
}

/// 基于 `reqwest` 库的默认 HTTP 客户端实现。
#[derive(Default, Clone)]
pub struct ReqwestHttpClient {
    pub client: Arc<reqwest::Client>,
}

impl ReqwestHttpClient {
    pub fn new() -> Self {
        ReqwestHttpClient {
            client: Arc::new(reqwest::Client::new()),
        }
    }
}

/// 基于 `reqwest` 库的默认 HTTP 客户端实现。
#[async_trait]
impl HttpClient for ReqwestHttpClient {
    /// 使用 `reqwest` 执行请求，并将 `http::Request` 转换为 `reqwest::Request`，
    /// 再将 `reqwest::Response` 转换为 `http::Response`。
    async fn execute(&self, req: Request<Vec<u8>>) -> Result<Response<Vec<u8>>> {
        let reqwest_req: ReqwestRequest = req.try_into()?;
        
        let reqwest_res = self.client.execute(reqwest_req).await?;
        
        let status = reqwest_res.status();
        let version = reqwest_res.version();
        let headers = reqwest_res.headers().clone();

        let body = reqwest_res.bytes().await?.to_vec();

        let mut http_res_builder = Response::builder().status(status).version(version);

        if let Some(headers_map) = http_res_builder.headers_mut() {
            headers_map.extend(headers);
        }

        let http_res = http_res_builder.body(body)?;

        Ok(http_res)
    }
}
