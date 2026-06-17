//! 接口调用凭据类型模块
//! 分为普通接口调用凭据和稳定版接口调用凭据

use super::access_token::AccessTokenBuilder;
use super::{AccessToken, AppConfig, HttpClient};
use crate::utils::MpResponse;
use crate::utils::build_request;
use crate::{Result, constants};
use async_trait::async_trait;
use http::Method;
use std::sync::Arc;
use tracing::debug;

/// 定义接口调用凭据的行为
#[async_trait]
pub trait TokenType: Send + Sync {
    async fn token(&self) -> Result<AccessToken>;
    fn app_config(&self) -> AppConfig;
}

/// 稳定版接口调用凭据
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

#[async_trait]
impl TokenType for StableToken {
    async fn token(&self) -> Result<AccessToken> {
        let body = serde_json::json!({
            "grant_type":"client_credential",
            "appid":self.app_id.to_string(),
            "secret":self.secret.to_string(),
            "force_refresh":self.force_refresh,
        });
        let request = build_request(&self.end_point, Method::POST, None, None, Some(body))?;

        let response = self.client.execute(request).await?;
        let response_body = response.into_body();
        let token_builder = serde_json::from_slice::<MpResponse<AccessTokenBuilder>>(&response_body)?;
        let token = token_builder.extract()?;
        let token = token.build();
        debug!("stable access token: {:#?}", token);
        Ok(token)
    }

    fn app_config(&self) -> AppConfig {
        AppConfig {
            app_id: self.app_id.clone(),
            secret: self.secret.clone(),
        }
    }
}

/// 普通接口调用凭据
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

#[async_trait]
impl TokenType for NonStableToken {
    async fn token(&self) -> Result<AccessToken> {
        let body = serde_json::json!({
            "grant_type":"client_credential",
            "appid":self.app_id.to_string(),
            "secret":self.secret.to_string(),
        });
        let request = build_request(&self.end_point, Method::POST, None, None, Some(body))?;

        let response = self.client.execute(request).await?;
        let response_body = response.into_body();
        let token_builder = serde_json::from_slice::<MpResponse<AccessTokenBuilder>>(&response_body)?;
        let token = token_builder.extract()?;
        let token = token.build();
        Ok(token)
    }

    fn app_config(&self) -> AppConfig {
        AppConfig {
            app_id: self.app_id.clone(),
            secret: self.secret.clone(),
        }
    }
}
