use crate::{
    Result,
    access_token::{AccessToken, get_access_token, get_stable_access_token},
    constants,
    credential::{Credential, CredentialBuilder},
    error::Error::InternalServer,
    response::Response,
};
use chrono::Utc;
use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
use tokio::sync::{Notify, RwLock};
use tracing::{Level, event, instrument};

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
        event!(Level::DEBUG, "code: {}", code);

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

        event!(Level::DEBUG, "authentication response: {:#?}", response);

        if response.status().is_success() {
            let response = response.json::<Response<CredentialBuilder>>().await?;

            let credential = response.extract()?.build();

            event!(Level::DEBUG, "credential: {:#?}", credential);

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
        let guard = self.access_token.read().await;
        event!(Level::DEBUG, "expired at: {}", guard.expired_at);

        if self.refreshing.load(Ordering::Acquire) {
            event!(Level::DEBUG, "refreshing");

            self.notify.notified().await;
        } else {
            event!(Level::DEBUG, "prepare to fresh");

            self.refreshing.store(true, Ordering::Release);

            drop(guard);

            event!(Level::DEBUG, "write access token guard");

            let mut guard = self.access_token.write().await;

            let builder = get_access_token(
                self.inner.client.clone(),
                &self.inner.app_id,
                &self.inner.secret,
            )
            .await?;

            guard.access_token = builder.access_token;
            guard.expired_at = builder.expired_at;

            self.refreshing.store(false, Ordering::Release);

            self.notify.notify_waiters();

            event!(Level::DEBUG, "fresh access token: {:#?}", guard);

            return Ok(guard.access_token.clone());
        }
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
        let guard = self.access_token.read().await;
        event!(Level::DEBUG, "expired at: {}", guard.expired_at);

        if self.refreshing.load(Ordering::Acquire) {
            event!(Level::DEBUG, "refreshing");

            self.notify.notified().await;
        } else {
            event!(Level::DEBUG, "prepare to fresh");

            self.refreshing.store(true, Ordering::Release);

            drop(guard);

            event!(Level::DEBUG, "write access token guard");

            let mut guard = self.access_token.write().await;

            let builder = get_stable_access_token(
                self.inner.client.clone(),
                &self.inner.app_id,
                &self.inner.secret,
                force_refresh,
            )
            .await?;

            guard.access_token = builder.access_token;
            guard.expired_at = builder.expired_at;

            self.refreshing.store(false, Ordering::Release);

            self.notify.notify_waiters();

            event!(Level::DEBUG, "fresh access token: {:#?}", guard);

            return Ok(guard.access_token.clone());
        }
        Ok(guard.access_token.clone())
    }
}

#[derive(Debug)]
struct ClientInner {
    app_id: String,
    secret: String,
    client: reqwest::Client,
}
