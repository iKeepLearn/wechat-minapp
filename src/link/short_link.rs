//! 微信小程序短链接生成模块
//!
//! 获取小程序 Short Link，适用于微信内拉起小程序的业务场景。
//! 目前只开放给电商类目(具体包含以下一级类目：电商平台、商家自营、跨境电商)。通过该接口，可以选择生成到期失效和永久有效的小程序短链。
//! [官方文档](https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/qrcode-link/short-link/generateShortLink.html)。
//!
//!
//! ## 示例
//!
//! ```no_run
//! use wechat_minapp::client::WechatMinapp;
//! use wechat_minapp::Link::{ShortLinkArgs,Link};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // 初始化客户端
//!     let app_id = "your_app_id";
//!     let secret = "your_app_secret";
//!     let client = WechatMinapp::new(app_id, secret);
//!     let link = Link::new(client);
//!
//!     let args = ShortLinkArgs::builder()
//!     .path("pages/index/index")
//!     .build()
//!     .unwrap();
//!     // 生成短链接
//!     let short_link = link.short_link(args).await?;
//!     
//!     
//!     Ok(())
//! }
//! ```
//!

use super::Link;
use crate::{
    Result, constants,
    error::Error::{self, InternalServer},
    utils::build_request,
};
use http::Method;
use serde::{Deserialize, Serialize};
use tracing::debug;

/// 短链接
///
/// # 示例
///
/// ```no_run
/// use wechat_minapp::client::WechatMinapp;
/// use wechat_minapp::Link::{ShortLinkArgs,Link};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // 初始化客户端
///     let app_id = "your_app_id";
///     let secret = "your_app_secret";
///     let client = WechatMinapp::new(app_id, secret);
///     let link = Link::new(client);
///
///     let args = ShortLinkArgs::builder()
///     .path("pages/index/index")
///     .build()
///     .unwrap();
///     // 生成短链接
///     let short_link = link.short_link(args).await?;
///     
///     
///     Ok(())
/// }
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShortLink {
    link: String,
}

/// 短链接生成参数
///
/// 用于配置短链接的生成选项，通过 [`ShortLinkArgs::builder()`] 方法创建。
#[derive(Debug, Deserialize, Serialize)]
pub struct ShortLinkArgs {
    #[serde(rename = "page_url")]
    path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_title: Option<String>,
    is_permanent: bool,
}

/// 短链接参数构建器
///
/// 提供链式调用的方式构建短链接参数，确保参数的正确性。
///
/// # 示例
///
/// ```
/// use wechat_minapp::Link::ShortLinkArgs;
///
/// let args = ShortLinkArgs::builder()
///     .path("pages/index/index")
///     .page_title("page title")
///     .with_permanent()
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Deserialize)]
pub struct ShortLinkArgsBuilder {
    path: Option<String>,
    page_title: Option<String>,
    is_permanent: Option<bool>,
}

impl ShortLinkArgs {
    pub fn builder() -> ShortLinkArgsBuilder {
        ShortLinkArgsBuilder::new()
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn page_title(&self) -> Option<String> {
        self.page_title.clone()
    }

    pub fn is_permanent(&self) -> bool {
        self.is_permanent
    }
}

impl ShortLinkArgsBuilder {
    pub fn new() -> Self {
        ShortLinkArgsBuilder {
            path: None,
            page_title: None,
            is_permanent: None,
        }
    }

    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    pub fn page_title(mut self, title: impl Into<String>) -> Self {
        self.page_title = Some(title.into());
        self
    }
    pub fn with_permanent(mut self) -> Self {
        self.is_permanent = Some(true);
        self
    }

    pub fn build(self) -> Result<ShortLinkArgs> {
        let path = self.path.map_or_else(
            || {
                Err(Error::InvalidParameter(
                    "小程序页面路径不能为空".to_string(),
                ))
            },
            |v| {
                if v.len() > 1024 {
                    return Err(Error::InvalidParameter(
                        "页面路径最大长度 1024 个字符".to_string(),
                    ));
                }
                Ok(v)
            },
        )?;

        Ok(ShortLinkArgs {
            path,
            page_title: self.page_title,
            is_permanent: self.is_permanent.unwrap_or(false),
        })
    }
}

impl Default for ShortLinkArgsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Link {
    /// 生成短链接
    ///
    /// 调用微信小程序短链接生成接口，返回短链接`ShortLink`。
    ///
    /// # 参数
    ///
    /// - `args`: 短链接生成参数
    ///
    /// # 返回
    ///
    /// 成功返回 `Ok(ShortLink)`，失败返回错误信息。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use wechat_minapp::client::WechatMinapp;
    /// use wechat_minapp::Link::{ShortLinkArgs,Link};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     // 初始化客户端
    ///     let app_id = "your_app_id";
    ///     let secret = "your_app_secret";
    ///     let client = WechatMinapp::new(app_id, secret);
    ///     let link = Link::new(client);
    ///
    ///     let args = ShortLinkArgs::builder()
    ///     .path("pages/index/index")
    ///     .build()
    ///     .unwrap();
    ///     // 生成短链接
    ///     let short_link = link.short_link(args).await?;
    ///     
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # 错误
    ///
    /// - 网络错误
    /// - 认证错误（access_token 无效）
    /// - 微信 API 返回错误
    /// - 参数序列化错误
    pub async fn short_link(&self, args: ShortLinkArgs) -> Result<ShortLink> {
        debug!("get qr code args {:?}", &args);

        let query = serde_json::json!({
            "access_token":self.client.token().await?
        });

        let body = serde_json::to_value(ShortLinkArgs {
            path: args.path.clone(),
            page_title: args.page_title,
            is_permanent: args.is_permanent,
        })?;

        let request = build_request(
            constants::SHORT_LINK_END_POINT,
            Method::POST,
            None,
            Some(query),
            Some(body),
        )?;

        let client = &self.client.client;

        let response = client.execute(request).await?;

        debug!("response: {:#?}", response);

        if response.status().is_success() {
            let (_parts, body) = response.into_parts();
            debug!("short link response body: {:?}", &body);
            eprintln!(
                "short link response body: {:?}",
                String::from_utf8_lossy(&body)
            );
            let json = serde_json::from_slice::<ShortLink>(&body)?;

            debug!("short link: {:#?}", &json);

            Ok(json)
        } else {
            let (_parts, body) = response.into_parts();
            let message = String::from_utf8_lossy(&body).to_string();
            Err(InternalServer(message))
        }
    }
}
