use super::User;
use super::credential::Credential;
use crate::{
    Result, constants, error::Error::InternalServer, response::Response,
    user::credential::CredentialBuilder,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, instrument};

/// 微信用户基本信息
///
/// 包含用户的昵称、性别、地区、头像等基本信息。
/// 这些数据通常通过前端 `wx.getUserInfo()` 获取并解密得到。
///
/// # 示例
///
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
///
/// # 数据来源
///
/// 用户信息需要通过以下步骤获取：
///
/// 1. 前端调用 `wx.getUserInfo()` 获取加密数据
/// 2. 后端使用会话密钥解密数据
/// 3. 解析为 `User` 结构体
///
/// # 字段说明
///
/// - `gender`: 性别，0-未知，1-男性，2-女性
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserInfo {
    nickname: String,
    gender: u8,
    country: String,
    province: String,
    city: String,
    avatar: String,
    watermark: Watermark,
}

impl UserInfo {
    pub fn nickname(&self) -> &str {
        &self.nickname
    }

    pub fn gender(&self) -> u8 {
        self.gender
    }

    pub fn country(&self) -> &str {
        &self.country
    }

    pub fn province(&self) -> &str {
        &self.province
    }

    pub fn city(&self) -> &str {
        &self.city
    }

    pub fn avatar(&self) -> &str {
        &self.avatar
    }

    pub fn app_id(&self) -> &str {
        &self.watermark.app_id
    }

    pub fn timestamp(&self) -> u64 {
        self.watermark.timestamp
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UserBuilder {
    #[serde(rename = "nickName")]
    nickname: String,
    gender: u8,
    country: String,
    province: String,
    city: String,
    #[serde(rename = "avatarUrl")]
    avatar: String,
    watermark: WatermarkBuilder,
}

impl UserBuilder {
    pub(crate) fn build(self) -> UserInfo {
        UserInfo {
            nickname: self.nickname,
            gender: self.gender,
            country: self.country,
            province: self.province,
            city: self.city,
            avatar: self.avatar,
            watermark: self.watermark.build(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Contact {
    phone_number: String,
    pure_phone_number: String,
    country_code: String,
    watermark: Watermark,
}

impl Contact {
    pub fn phone_number(&self) -> &str {
        &self.phone_number
    }

    pub fn pure_phone_number(&self) -> &str {
        &self.pure_phone_number
    }

    pub fn country_code(&self) -> &str {
        &self.country_code
    }

    pub fn app_id(&self) -> &str {
        &self.watermark.app_id
    }

    pub fn timestamp(&self) -> u64 {
        self.watermark.timestamp
    }
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct ContactBuilder {
    #[serde(rename = "phone_info")]
    inner: PhoneInner,
}

impl ContactBuilder {
    pub(crate) fn build(self) -> Contact {
        Contact {
            phone_number: self.inner.phone_number,
            pure_phone_number: self.inner.pure_phone_number,
            country_code: self.inner.country_code,
            watermark: self.inner.watermark.build(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct PhoneInner {
    #[serde(rename = "phoneNumber")]
    phone_number: String,
    #[serde(rename = "purePhoneNumber")]
    pure_phone_number: String,
    country_code: String,
    watermark: WatermarkBuilder,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Watermark {
    app_id: String,
    timestamp: u64,
}

#[derive(Debug, Deserialize, Clone)]
struct WatermarkBuilder {
    #[serde(rename = "appid")]
    app_id: String,
    timestamp: u64,
}

impl WatermarkBuilder {
    fn build(self) -> Watermark {
        Watermark {
            app_id: self.app_id,
            timestamp: self.timestamp,
        }
    }
}

impl<'a> User<'a> {
    /// 用户登录凭证校验
    ///
    /// 通过微信前端获取的临时登录凭证 code，换取用户的唯一标识 OpenID 和会话密钥。
    ///
    /// # 参数
    ///
    /// - `code`: 微信前端通过 `wx.login()` 获取的临时登录凭证
    ///
    /// # 返回
    ///
    /// 成功返回 `Ok(Credential)`，包含用户身份信息
    ///
    /// # 错误
    ///
    /// - 网络错误
    /// - 微信 API 返回错误
    /// - 响应解析错误
    ///
    /// # 示例
    ///
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
    ///     println!("用户OpenID: {}", credential.open_id());
    ///     println!("会话密钥: {}", credential.session_key());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # API 文档
    ///
    /// [微信官方文档 - code2Session](https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/user-login/code2Session.html)
    #[instrument(skip(self, code))]
    pub async fn login(&self, code: &str) -> Result<Credential> {
        debug!("code: {}", code);

        let mut map: HashMap<&str, &str> = HashMap::new();
        let inner = self.client.inner_client();
        map.insert("appid", &inner.app_id);
        map.insert("secret", &inner.secret);
        map.insert("js_code", code);
        map.insert("grant_type", "authorization_code");

        let response = inner
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

    /// 获取用户手机号信息
    ///
    /// 通过前端获取的临时凭证 code 换取用户的手机号信息。
    ///
    /// # 参数
    ///
    /// - `code`: 前端通过 `wx.getPhoneNumber` 获取的临时凭证
    /// - `open_id`: 用户 OpenID（可选），如果提供可以提升安全性
    ///
    /// # 返回
    ///
    /// 成功返回 `Ok(Contact)`，包含用户手机号信息
    ///
    /// # 错误
    ///
    /// - 网络错误
    /// - 微信 API 返回错误
    /// - 访问令牌无效或过期
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use wechat_minapp::client::StableTokenClient;
    /// use wechat_minapp::user::{User, Contact};
    ///
    ///  #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = StableTokenClient::new("app_id", "secret");
    ///     let user = User::new(client);
    ///     let code = "0816abc123def456";
    ///     let contact = user.get_contact(code, None).await?;
    ///     println!("用户手机号: {}", contact.phone_number());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # 前端配合
    ///
    /// 前端需要调用 `wx.getPhoneNumber` 获取临时凭证：
    ///
    /// ```javascript
    /// wx.getPhoneNumber({
    ///   success: (res) => {
    ///     console.log(res.code); // 将这个 code 发送到后端
    ///   },
    ///   fail: (err) => {
    ///     console.error(err);
    ///   }
    /// });
    /// ```
    ///
    /// # API 文档
    ///
    /// [获取手机号](https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/user-info/phone-number/getPhoneNumber.html)
    pub async fn get_contact(&self, code: &str, open_id: Option<&str>) -> Result<Contact> {
        debug!("code: {}, open_id: {:?}", code, open_id);

        let mut query = HashMap::new();
        let mut body = HashMap::new();
        let client = &self.client.inner_client().client;
        query.insert("access_token", self.client.token().await?);
        body.insert("code", code);

        if let Some(open_id) = open_id {
            body.insert("openid", open_id);
        }

        let response = client
            .post(constants::PHONE_END_POINT)
            .query(&query)
            .json(&body)
            .send()
            .await?;

        debug!("response: {:#?}", response);

        if response.status().is_success() {
            let response = response.json::<Response<ContactBuilder>>().await?;

            let builder = response.extract()?;

            debug!("contact builder: {:#?}", builder);

            Ok(builder.build())
        } else {
            Err(InternalServer(response.text().await?))
        }
    }
}
