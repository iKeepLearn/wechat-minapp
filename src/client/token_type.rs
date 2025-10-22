//! 接口调用凭据类型模块
//! 分为普通接口调用凭据和稳定版接口调用凭据
//!
//! 普通接口调用凭据使用场景：
//! - 适用于大多数微信小程序后台服务API调用场景
//! - 适用于对访问令牌实时性要求较高的场景
//!
//! 稳定版接口调用凭据使用场景：
//! - 适用于需要长期稳定访问令牌的场景
//! - 适用于对访问令牌实时性要求不高，但需要确保访问令牌长期有效的场景
//!

use super::access_token::AccessTokenBuilder;
use super::{AccessToken, AppConfig, HttpClient};
use crate::{Result, constants};
use async_trait::async_trait;
use http::{Method, Request};
use std::collections::HashMap;
use std::sync::Arc;

/// 定义接口调用凭据的行为
#[async_trait]
pub trait TokenType: Send + Sync {
    /// 获取接口调用凭据
    async fn token(&self) -> Result<AccessToken>;
    /// 获取应用配置
    fn app_config(&self) -> AppConfig;
}

/// 稳定版接口调用凭据
/// [官方文档](https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/stable-token/stableToken.html)
#[derive(Clone)]
pub struct StableToken {
    pub app_id: String,
    pub secret: String,
    pub end_point: String,
    pub force_refresh: bool,
    client: Arc<dyn HttpClient>,
}

impl StableToken {
    pub fn new(
        app_id: &str,
        secret: &str,
        force_refresh: bool,
        client: Arc<dyn HttpClient>,
    ) -> Self {
        StableToken {
            app_id: app_id.to_string(),
            secret: secret.to_string(),
            end_point: constants::STABLE_ACCESS_TOKEN_END_POINT.to_string(),
            force_refresh,
            client,
        }
    }
}

/// 稳定版接口调用凭据获取实现
#[async_trait]
impl TokenType for StableToken {
    /// 获取稳定版接口调用凭据
    async fn token(&self) -> Result<AccessToken> {
        let mut body: HashMap<&str, String> = HashMap::new();
        body.insert("grant_type", "client_credential".into());
        body.insert("appid", self.app_id.to_string());
        body.insert("secret", self.secret.to_string());

        if self.force_refresh {
            body.insert("force_refresh", self.force_refresh.to_string());
        }
        let req_body = serde_json::to_vec(&body)?;
        let request = Request::builder()
            .uri(self.end_point.clone())
            .method(Method::POST)
            .header("User-Agent", constants::HTTP_CLIENT_USER_AGENT)
            .body(req_body)?;

        let response = self.client.execute(request).await?;
        let response_body = response.into_body();
        let token_builder = serde_json::from_slice::<AccessTokenBuilder>(&response_body)?;
        Ok(token_builder.build())
    }

    /// 获取应用配置
    fn app_config(&self) -> AppConfig {
        AppConfig {
            app_id: self.app_id.clone(),
            secret: self.secret.clone(),
        }
    }
}

/// 普通接口调用凭据
/// [官方文档](https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/access-token/accessToken.html)
pub struct NonStableToken {
    pub app_id: String,
    pub secret: String,
    pub end_point: String,
    client: Arc<dyn HttpClient>,
}

impl NonStableToken {
    pub fn new(app_id: &str, secret: &str, client: Arc<dyn HttpClient>) -> Self {
        NonStableToken {
            app_id: app_id.to_string(),
            secret: secret.to_string(),
            end_point: constants::ACCESS_TOKEN_END_POINT.to_string(),
            client,
        }
    }
}

/// 普通接口调用凭据获取实现
#[async_trait]
impl TokenType for NonStableToken {
    /// 获取普通接口调用凭据
    async fn token(&self) -> Result<AccessToken> {
        let mut body: HashMap<&str, String> = HashMap::new();
        body.insert("grant_type", "client_credential".into());
        body.insert("appid", self.app_id.to_string());
        body.insert("secret", self.secret.to_string());

        let req_body = serde_json::to_vec(&body)?;
        let request = Request::builder()
            .uri(self.end_point.clone())
            .method(Method::POST)
            .header("User-Agent", constants::HTTP_CLIENT_USER_AGENT)
            .body(req_body)?;

        let response = self.client.execute(request).await?;
        let response_body = response.into_body();
        let token_builder = serde_json::from_slice::<AccessTokenBuilder>(&response_body)?;
        Ok(token_builder.build())
    }

    /// 获取应用配置
    fn app_config(&self) -> AppConfig {
        AppConfig {
            app_id: self.app_id.clone(),
            secret: self.secret.clone(),
        }
    }
}
