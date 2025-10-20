use super::{
    Client, ClientInner,
    access_token::{AccessToken, AccessTokenBuilder, is_token_expired},
};
use crate::{Result, constants, error::Error::InternalServer, response::Response};
use async_trait::async_trait;
use chrono::Utc;
use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
use tokio::sync::{Notify, RwLock};
use tracing::{debug, instrument};

#[derive(Debug, Clone)]
pub struct NonStableTokenClient {
    inner: Arc<ClientInner>,
    access_token: Arc<RwLock<AccessToken>>,
    refreshing: Arc<AtomicBool>,
    notify: Arc<Notify>,
}

#[async_trait]
impl Client for NonStableTokenClient {
    #[instrument(skip(self))]
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

    fn inner_client(&self) -> &ClientInner {
        &self.inner
    }
}

impl NonStableTokenClient {
    /// 创建新的微信小程序客户端
    ///
    /// # 参数
    ///
    /// - `app_id`: 小程序 AppID
    /// - `secret`: 小程序 AppSecret
    ///
    /// # 返回
    ///
    /// 新的 `StableTokenClient` 实例
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use wechat_minapp::client::NonStableTokenClient;
    ///
    /// let client = NonStableTokenClient::new("your_appid", "your_app_secret_here");
    /// ```
    pub fn new(app_id: &str, secret: &str) -> Self {
        NonStableTokenClient {
            inner: Arc::new(ClientInner {
                app_id: app_id.to_string(),
                secret: secret.to_string(),
                client: reqwest::Client::new(),
            }),
            access_token: Arc::new(RwLock::new(AccessToken {
                access_token: String::new(),
                expired_at: Utc::now(),
            })),
            refreshing: Arc::new(AtomicBool::new(false)),
            notify: Arc::new(Notify::new()),
        }
    }

    /// 获取小程序全局唯一后台接口调用凭据（access_token）
    /// https://developers.weixin.qq.com/miniprogram/dev/api-backend/open-api/access-token/auth.getAccessToken.html
    async fn get_access_token(&self) -> Result<AccessTokenBuilder> {
        let mut map: HashMap<&str, String> = HashMap::new();
        let client = &self.inner.client;
        let appid = &self.inner.app_id;
        let secret = &self.inner.secret;
        map.insert("grant_type", "client_credential".into());
        map.insert("appid", appid.to_string());
        map.insert("secret", secret.to_string());

        let response = client
            .post(constants::ACCESS_TOKEN_END_POINT)
            .json(&map)
            .send()
            .await?;

        debug!("response: {:#?}", response);

        if response.status().is_success() {
            let response = response.json::<Response<AccessTokenBuilder>>().await?;

            let builder = response.extract()?;

            debug!("stable access token builder: {:#?}", builder);

            Ok(builder)
        } else {
            Err(InternalServer(response.text().await?))
        }
    }

    async fn refresh_access_token(&self) -> Result<String> {
        let mut guard = self.access_token.write().await;

        if !is_token_expired(&guard) {
            debug!("token already refreshed by another thread");
            return Ok(guard.access_token.clone());
        }

        debug!("performing network request to refresh token");

        let builder = self.get_access_token().await?;

        guard.access_token = builder.access_token.clone();
        guard.expired_at = builder.expired_at;

        debug!("fresh access token: {:#?}", guard);

        Ok(guard.access_token.clone())
    }
}
