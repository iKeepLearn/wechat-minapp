use super::user_info::{UserBuilder, UserInfo};
use super::User;
use crate::constants;
use http::Method;
use serde::{Deserialize, Serialize};
use serde_json::from_slice;
use tracing::{debug, instrument};
use wechat_core::utils::{aes_decrypt, hmac_sha256, RequestBuilder, ResponseExt};
use wechat_core::Result;

#[derive(Serialize, Deserialize, Clone)]
pub struct Credential {
    #[serde(rename = "openid")]
    open_id: String,
    session_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    union_id: Option<String>,
}

impl Credential {
    pub fn open_id(&self) -> &str {
        &self.open_id
    }

    pub fn session_key(&self) -> &str {
        &self.session_key
    }

    pub fn union_id(&self) -> Option<&str> {
        self.union_id.as_deref()
    }

    /// 解密用户数据，使用的是 AES-128-CBC 算法，数据采用PKCS#7填充。
    /// https://developers.weixin.qq.com/miniprogram/dev/framework/open-ability/signature.html
    /// ```no_run
    /// use wechat_minapp::WechatMinapp;
    /// use wechat_minapp::user::{User, Contact};
    ///
    ///  #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = WechatMinapp::new("app_id", "secret");
    ///     let user = User::new(client);
    ///     let code = "0816abc123def456";
    ///     let credential = user.login(code).await?;
    ///     let info = credential.decrypt(&encrypted_data, &iv)?;
    ///     println!("昵称: {}", info.nickname());
    ///     println!("性别: {}", info.gender());
    ///     println!("地区: {}-{}-{}", info.country(), info.province(), info.city());
    ///     println!("头像: {}", info.avatar());
    ///     println!("AppID: {}", info.app_id());
    ///     println!("时间戳: {}", info.timestamp());
    ///
    ///     Ok(())
    /// }
    /// ```
    #[instrument(skip(self, encrypted_data, iv))]
    pub fn decrypt(&self, encrypted_data: &str, iv: &str) -> Result<UserInfo> {
        debug!("encrypted_data: {}", encrypted_data);
        debug!("iv: {}", iv);

        let buffer = aes_decrypt(encrypted_data, &self.session_key, iv)?;

        let builder = from_slice::<UserBuilder>(&buffer)?;

        debug!("user builder: {:#?}", builder);

        Ok(builder.build())
    }
}

impl std::fmt::Debug for Credential {
    // 为了安全，不打印 session_key
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Credential")
            .field("open_id", &self.open_id)
            .field("session_key", &"********")
            .field("union_id", &self.union_id)
            .finish()
    }
}

impl User {
    /// 检查登录态是否过期
    /// [官方文档](https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/user-login/checkSessionKey.html)
    #[instrument(skip(self, session_key, open_id))]
    pub async fn check_session_key(&self, session_key: &str, open_id: &str) -> Result<()> {
        let signature = hmac_sha256(b"", session_key)?;

        let query = serde_json::json!({
            "openid": open_id.to_string(),
            "signature":signature,
            "sig_method": "hmac_sha256".to_string()
        });

        let request = RequestBuilder::new(constants::CHECK_SESSION_KEY_END_POINT)
            .query(query)
            .method(Method::GET)
            .build()?;

        let client = &self.client.core.client;

        let response = client.execute(request).await?;

        debug!("response: {:#?}", response);
        response.to_json::<()>()
    }

    /// 重置用户的 session_key
    /// [官方文档](https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/user-login/ResetUserSessionKey.html)
    #[instrument(skip(self, open_id))]
    pub async fn reset_session_key(&self, session_key: &str, open_id: &str) -> Result<Credential> {
        let signature = hmac_sha256(b"", session_key)?;

        let query = serde_json::json!({
            "access_token":self.client.token().await?,
            "openid": open_id.to_string(),
            "signature":signature,
            "sig_method": "hmac_sha256".to_string()
        });

        let request = RequestBuilder::new(constants::RESET_SESSION_KEY_END_POINT)
            .query(query)
            .method(Method::GET)
            .build()?;

        let client = &self.client.core.client;
        let response = client.execute(request).await?;
        debug!("response: {:#?}", &response);

        response.to_json::<Credential>()
    }
}
