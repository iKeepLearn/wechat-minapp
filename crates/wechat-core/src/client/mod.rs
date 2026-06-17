//! 微信小程序服务端接口 Client 模块 - 核心功能
//!
//! HTTP 客户端和令牌存储方式分离，可以按自己的需求实现不同的客户端和存储方式。
//! 支持稳定版接口调用凭据和普通版接口调用凭据。

mod access_token;
mod token_storage;
pub mod token_type;

pub use access_token::AccessToken;
pub use token_storage::{MemoryTokenStorage, TokenStorage};
pub use token_type::{NonStableToken, StableToken, TokenType};

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

/// 微信 SDK 核心客户端结构。
///
/// 封装了 `HttpClient` 和 `TokenStorage`，提供发送请求和获取 Access Token 的方法。
pub struct WechatCore {
    pub client: Arc<dyn HttpClient>,
    pub token_storage: Arc<dyn TokenStorage>,
}

impl fmt::Debug for WechatCore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WechatCore")
            .field("client", &"Arc<dyn HttpClient>")
            .field("token_storage", &"Arc<dyn TokenStorage>")
            .finish()
    }
}

impl Clone for WechatCore {
    fn clone(&self) -> Self {
        WechatCore {
            client: self.client.clone(),
            token_storage: self.token_storage.clone(),
        }
    }
}

impl WechatCore {
    /// 使用默认配置（`ReqwestHttpClient` 和 `MemoryTokenStorage` 与 `StableToken`）创建客户端。
    pub fn new(app_id: &str, secret: &str) -> Self {
        let http_client = Arc::new(ReqwestHttpClient::new());
        let token_type = Arc::new(StableToken::new(app_id, secret, false, http_client.clone()));
        let token_storage = Arc::new(MemoryTokenStorage::new(token_type));

        WechatCore {
            client: http_client,
            token_storage,
        }
    }

    /// 使用自定义的 `HttpClient` 和 `TokenStorage` 创建客户端。
    pub fn custom(http_client: Arc<dyn HttpClient>, token_storage: Arc<dyn TokenStorage>) -> Self {
        WechatCore {
            client: http_client,
            token_storage,
        }
    }

    /// 获取接口调用凭据（Access Token）。
    pub async fn token(&self) -> Result<String> {
        self.token_storage.token().await
    }

    /// 获取当前客户端的 App ID 和 Secret 配置。
    pub fn app_config(&self) -> AppConfig {
        self.token_storage.token_type().app_config()
    }
}

/// 定义 HTTP 客户端行为的 Trait。
#[async_trait]
pub trait HttpClient: Send + Sync {
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

#[async_trait]
impl HttpClient for ReqwestHttpClient {
    async fn execute(&self, req: Request<Vec<u8>>) -> Result<Response<Vec<u8>>> {
        let reqwest_req: ReqwestRequest = req.try_into()?;

        #[cfg(test)]
        eprintln!("reqwest url: {:?}", reqwest_req.url());

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
