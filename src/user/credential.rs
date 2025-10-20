use super::User;
use super::user_info::{UserBuilder, UserInfo};
#[allow(deprecated)]
use aes::{
    Aes128,
    cipher::{BlockDecryptMut, KeyIvInit, block_padding::Pkcs7, generic_array::GenericArray},
};
use base64::{Engine, engine::general_purpose::STANDARD};
use cbc::Decryptor;
use hex::encode;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use serde_json::from_slice;
use sha2::Sha256;
use std::collections::HashMap;
use tracing::{debug, instrument};

use crate::{Result, constants, error::Error::InternalServer, response::Response};

type Aes128CbcDec = Decryptor<Aes128>;

#[derive(Serialize, Deserialize, Clone)]
pub struct Credential {
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
    /// use wechat_minapp::client::StableTokenClient;
    /// use wechat_minapp::user::{User, Contact};
    ///
    ///  #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = StableTokenClient::new("app_id", "secret");
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

        let key = STANDARD.decode(self.session_key.as_bytes())?;
        let iv = STANDARD.decode(iv.as_bytes())?;
        #[allow(deprecated)]
        let decryptor = Aes128CbcDec::new(
            &GenericArray::clone_from_slice(&key),
            &GenericArray::clone_from_slice(&iv),
        );

        let encrypted_data = STANDARD.decode(encrypted_data.as_bytes())?;

        let buffer = decryptor.decrypt_padded_vec_mut::<Pkcs7>(&encrypted_data)?;

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

#[derive(Deserialize)]
pub(crate) struct CredentialBuilder {
    #[serde(rename = "openid")]
    open_id: String,
    session_key: String,
    #[serde(rename = "unionid")]
    union_id: Option<String>,
}

impl CredentialBuilder {
    pub(crate) fn build(self) -> Credential {
        Credential {
            open_id: self.open_id,
            session_key: self.session_key,
            union_id: self.union_id,
        }
    }
}

impl std::fmt::Debug for CredentialBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CredentialBuilder")
            .field("open_id", &self.open_id)
            .field("session_key", &"********")
            .field("union_id", &self.union_id)
            .finish()
    }
}

type HmacSha256 = Hmac<Sha256>;

impl User {
    /// 检查登录态是否过期
    /// https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/user-login/checkSessionKey.html
    #[instrument(skip(self, session_key, open_id))]
    pub async fn check_session_key(&self, session_key: &str, open_id: &str) -> Result<()> {
        let mut mac = HmacSha256::new_from_slice(session_key.as_bytes())?;
        mac.update(b"");
        let hasher = mac.finalize();
        let signature = encode(hasher.into_bytes());

        let mut map = HashMap::new();

        map.insert("openid", open_id.to_string());
        map.insert("signature", signature);
        map.insert("sig_method", "hmac_sha256".into());
        let client = &self.client.inner_client().client;
        let response = client
            .get(constants::CHECK_SESSION_KEY_END_POINT)
            .query(&map)
            .send()
            .await?;

        debug!("response: {:#?}", response);

        if response.status().is_success() {
            let response = response.json::<Response<()>>().await?;

            response.extract()
        } else {
            Err(crate::error::Error::InternalServer(response.text().await?))
        }
    }

    /// 重置用户的 session_key
    /// https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/user-login/ResetUserSessionKey.html
    #[instrument(skip(self, open_id))]
    pub async fn reset_session_key(&self, session_key: &str, open_id: &str) -> Result<Credential> {
        let mut mac = HmacSha256::new_from_slice(session_key.as_bytes())?;
        mac.update(b"");
        let hasher = mac.finalize();
        let signature = encode(hasher.into_bytes());

        let mut map = HashMap::new();
        let client = &self.client.inner_client().client;
        map.insert("access_token", self.client.token().await?);
        map.insert("openid", open_id.to_string());
        map.insert("signature", signature);
        map.insert("sig_method", "hmac_sha256".into());

        let response = client
            .get(constants::RESET_SESSION_KEY_END_POINT)
            .query(&map)
            .send()
            .await?;

        debug!("response: {:#?}", response);

        if response.status().is_success() {
            let response = response.json::<Response<CredentialBuilder>>().await?;

            let credential = response.extract()?.build();

            debug!("credential: {:#?}", credential);

            Ok(credential)
        } else {
            Err(InternalServer(response.text().await?))
        }
    }
}
