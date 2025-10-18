use crate::{
    Result,
    access_token::{AccessToken, get_access_token, get_stable_access_token},
    constants,
    credential::{Credential, CredentialBuilder},
    error::Error::InternalServer,
    response::Response,
};
use chrono::{Duration, Utc};
use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
use tokio::sync::{Notify, RwLock};
use tracing::{debug, instrument};

/// 存储微信小程序的 appid 和 secret
#[derive(Debug, Clone)]
pub struct Client {
    inner: Arc<ClientInner>,
    access_token: Arc<RwLock<AccessToken>>,
    refreshing: Arc<AtomicBool>,
    notify: Arc<Notify>,
}

impl Client {
    /// ```ignore
    /// use wechat_minapp::Client;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let app_id = "your app id";
    ///     let secret = "your app secret";
    ///     
    ///     let client = Client::new(app_id, secret);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn new(app_id: &str, secret: &str) -> Self {
        let client = reqwest::Client::new();

        Self {
            inner: Arc::new(ClientInner {
                app_id: app_id.into(),
                secret: secret.into(),
                client,
            }),
            access_token: Arc::new(RwLock::new(AccessToken {
                access_token: "".to_string(),
                expired_at: Utc::now(),
                force_refresh: None,
            })),
            refreshing: Arc::new(AtomicBool::new(false)),
            notify: Arc::new(Notify::new()),
        }
    }

    pub(crate) fn request(&self) -> &reqwest::Client {
        &self.inner.client
    }

    /// 登录凭证校验
    /// https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/user-login/code2Session.html
    /// ```rust
    /// use axum::{extract::State, response::IntoResponse, Json};
    /// use wechat_minapp::{client::Client, Result};
    /// use serde::Deserialize;
    ///
    /// #[derive(Deserialize, Default)]
    /// #[serde(default)]
    /// pub(crate) struct Logger {
    ///     code: String,
    /// }
    ///
    /// pub(crate) async fn login(
    ///     State(client): State<Client>,
    ///     Json(logger): Json<Logger>,
    /// ) -> Result<impl IntoResponse> {
    ///    let credential = client.login(&logger.code).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    #[instrument(skip(self, code))]
    pub async fn login(&self, code: &str) -> Result<Credential> {
        debug!("code: {}", code);

        let mut map: HashMap<&str, &str> = HashMap::new();

        map.insert("appid", &self.inner.app_id);
        map.insert("secret", &self.inner.secret);
        map.insert("js_code", code);
        map.insert("grant_type", "authorization_code");

        let response = self
            .inner
            .client
            .get(constants::AUTHENTICATION_END_POINT)
            .query(&map)
            .send()
            .await?;

        debug!("authentication response: {:#?}", response);

        if response.status().is_success() {
            let response = response.json::<Response<CredentialBuilder>>().await?;

            let credential = response.extract()?.build();

            debug!("credential: {:#?}", credential);

            Ok(credential)
        } else {
            Err(InternalServer(response.text().await?))
        }
    }

    /// ```ignore
    /// use wechat_minapp::Client;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let app_id = "your app id";
    ///     let secret = "your app secret";
    ///     
    ///     let client = Client::new(app_id, secret);
    ///     let access_token = client.access_token().await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    pub async fn access_token(&self) -> Result<String> {
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

    async fn refresh_access_token(&self) -> Result<String> {
        let mut guard = self.access_token.write().await;

        if !is_token_expired(&guard) {
            debug!("token already refreshed by another thread");
            return Ok(guard.access_token.clone());
        }

        debug!("performing network request to refresh token");

        let builder = get_access_token(
            self.inner.client.clone(),
            &self.inner.app_id,
            &self.inner.secret,
        )
        .await?;

        guard.access_token = builder.access_token.clone();
        guard.expired_at = builder.expired_at;

        debug!("fresh access token: {:#?}", guard);

        Ok(guard.access_token.clone())
    }

    /// ```ignore
    /// use wechat_minapp::Client;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let app_id = "your app id";
    ///     let secret = "your app secret";
    ///     
    ///     let client = Client::new(app_id, secret);
    ///     let stable_access_token = client.stable_access_token(Some(true)).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    ///
    pub async fn stable_access_token(
        &self,
        force_refresh: impl Into<Option<bool>> + Clone + Send,
    ) -> Result<String> {
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
            match self.refresh_stable_access_token(force_refresh).await {
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

    async fn refresh_stable_access_token(
        &self,
        force_refresh: impl Into<Option<bool>> + Clone + Send,
    ) -> Result<String> {
        // 1. Acquire the write lock. This blocks if another thread won CAS but is refreshing.
        let mut guard = self.access_token.write().await;

        // 2. Double-check expiration under the write lock (CRITICAL)
        // If another CAS-winner refreshed the token while we were waiting for the write lock,
        // we return the new token without performing a new network call.
        if !is_token_expired(&guard) {
            // Token is now fresh, return it
            debug!("token already refreshed by another thread");
            return Ok(guard.access_token.clone());
        }

        // 3. Perform the network request since the token is still stale
        debug!("performing network request to refresh token");

        let builder = get_stable_access_token(
            self.inner.client.clone(),
            &self.inner.app_id,
            &self.inner.secret,
            force_refresh,
        )
        .await?;

        // 4. Update the token
        guard.access_token = builder.access_token.clone();
        guard.expired_at = builder.expired_at;

        debug!("fresh access token: {:#?}", guard);

        // Return the newly fetched token (cloned here for consistency)
        Ok(guard.access_token.clone())
    }
}

#[derive(Debug)]
struct ClientInner {
    app_id: String,
    secret: String,
    client: reqwest::Client,
}

fn is_token_expired(token: &AccessToken) -> bool {
    // 添加安全边界，提前刷新
    let now = Utc::now();
    token.expired_at.signed_duration_since(now) < Duration::minutes(5)
}
