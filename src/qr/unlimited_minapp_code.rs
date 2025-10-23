//! 微信小程序小程序码生成模块
//!
//! 该模块提供了生成微信小程序小程序码的功能，支持多种类型的小程序码和自定义参数。
//!
//! # 主要功能
//!
//! - 生成小程序页面小程序码
//! - 支持自定义尺寸、颜色、透明度等参数
//! - 支持不同环境版本（开发版、体验版、正式版）
//! - 链式参数构建器模式
//!
//! # 快速开始
//!
//! ```no_run
//! use wechat_minapp::client::WechatMinapp;
//! use wechat_minapp::qr::{UnlimitedQrCodeArgs,Qr, MinappEnvVersion};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // 初始化客户端
//!     let app_id = "your_app_id";
//!     let secret = "your_app_secret";
//!     let client = WechatMinapp::new(app_id, secret);
//!     let qr = Qr::new(client);
//!
//!     // 构建小程序码参数
//!     let args = UnlimitedQrCodeArgs::builder()
//!         .page("pages/index/index")
//!         .scene("a=1")
//!         .width(300)
//!         .env_version(MinappEnvVersion::Release)
//!         .build()?;
//!
//!     // 生成小程序码
//!     let qr_code = qr.unlimited_qr_code(args).await?;
//!     
//!     // 获取小程序码图片数据
//!     let buffer = qr_code.buffer();
//!     println!("生成的小程序码大小: {} bytes", buffer.len());
//!
//!     // 可以将 buffer 保存为文件或直接返回给前端
//!     // std::fs::write("qrcode.png", buffer)?;
//!     
//!     Ok(())
//! }
//! ```
//!
//! # 参数说明
//!
//! - `path`: 小程序页面路径，必填，最大长度 1024 字符
//! - `width`: 小程序码宽度，单位 px，最小 280px，最大 1280px
//! - `auto_color`: 是否自动配置线条颜色
//! - `line_color`: 自定义线条颜色，RGB 格式
//! - `is_hyaline`: 是否透明背景
//! - `env_version`: 环境版本，默认为正式版
//!
//! # 示例
//!
//! ## 生成带颜色的小程序码
//!
//! ```no_run
//! use wechat_minapp::client::WechatMinapp;
//! use wechat_minapp::qr::{UnlimitedQrCodeArgs,Qr, MinappEnvVersion};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // 初始化客户端
//!     let app_id = "your_app_id";
//!     let secret = "your_app_secret";
//!     let client = WechatMinapp::new(app_id, secret);
//!     let qr = Qr::new(client);
//!
//!     let args = UnlimitedQrCodeArgs::builder()
//!     .page("pages/detail/detail")
//!     .scene("id=123")
//!     .width(400)
//!     .line_color(Rgb::new(255, 0, 0)) // 红色线条
//!     .with_is_hyaline() // 透明背景
//!     .env_version(MinappEnvVersion::Develop)
//!     .build()
//!     .unwrap();
//!     // 生成小程序码
//!     let qr_code = qr.unlimited_qr_code(args).await?;
//!     
//!     // 获取小程序码图片数据
//!     let buffer = qr_code.buffer();
//!     Ok(())
//! }
//! ```
//!
//! ## 生成简单小程序码
//!
//! ```no_run
//! use wechat_minapp::client::WechatMinapp;
//! use wechat_minapp::qr::{UnlimitedQrCodeArgs,Qr, MinappEnvVersion};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // 初始化客户端
//!     let app_id = "your_app_id";
//!     let secret = "your_app_secret";
//!     let client = WechatMinapp::new(app_id, secret);
//!     let qr = Qr::new(client);
//!
//!     let args = UnlimitedQrCodeArgs::builder()
//!     .page("pages/index/index")
//!     .build()
//!     .unwrap();
//!     // 生成小程序码
//!     let qr_code = qr.unlimited_qr_code(args).await?;
//!     
//!     // 获取小程序码图片数据
//!     let buffer = qr_code.buffer();
//!     Ok(())
//! }
//! ```
//!
//! # 错误处理
//!
//! 小程序码生成可能遇到以下错误：
//!
//! - 参数验证错误（路径为空或过长）
//! - 认证错误（access_token 无效）
//! - 网络错误
//! - 微信 API 返回错误
//!
//! 建议在生产环境中妥善处理这些错误。

use super::{MinappEnvVersion, Qr, QrCode, Rgb};
use crate::{
    Result, constants,
    error::Error::{self, InternalServer},
    new_type::{NonQueryPagePath, SceneString},
    utils::RequestBuilder,
};
use http::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use tracing::debug;

/// 无限制小程序码生成参数
///
/// 用于配置无限制小程序码的生成选项，通过 [`UnlimitedQrCodeArgs::builder()`] 方法创建。
#[derive(Debug, Deserialize, Serialize)]
pub struct UnlimitedQrCodeArgs {
    /// 默认是主页，页面 page，例如 pages/index/index，根路径前不要填加 /，不能携带参数（参数请放在scene字段里），如果不填写这个字段，默认跳主页面。scancode_time为系统保留参数，不允许配置
    page: String,
    /// 最大32个可见字符，只支持数字，大小写英文以及部分特殊字符：!#$&'()*+,/:;=?@-._~，其它字符请自行编码为合法字符（因不支持%，中文无法使用 urlencode 处理，请使用其他编码方式）
    scene: String,
    /// 默认430，二维码的宽度，单位 px，最小 280px，最大 1280px
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<i16>,
    /// 默认是true，检查page 是否存在，为 true 时 page 必须是已经发布的小程序存在的页面（否则报错）；为 false 时允许小程序未发布或者 page 不存在， 但page 有数量上限（60000个）请勿滥用。
    #[serde(skip_serializing_if = "Option::is_none")]
    check_path: Option<bool>,
    /// 自动配置线条颜色，如果颜色依然是黑色，则说明不建议配置主色调，默认 false
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_color: Option<bool>,
    /// 默认是{"r":0,"g":0,"b":0} 。auto_color 为 false 时生效，使用 rgb 设置颜色 例如 {"r":"xxx","g":"xxx","b":"xxx"} 十进制表示
    #[serde(skip_serializing_if = "Option::is_none")]
    line_color: Option<Rgb>,
    /// 默认是false，是否需要透明底色，为 true 时，生成透明底色的小程序
    #[serde(skip_serializing_if = "Option::is_none")]
    is_hyaline: Option<bool>,
    /// 要打开的小程序版本。正式版为 "release"，体验版为 "trial"，开发版为 "develop"。默认是正式版。
    #[serde(skip_serializing_if = "Option::is_none")]
    env_version: Option<MinappEnvVersion>,
}

/// 无限制小程序码参数构建器
///
/// 提供链式调用的方式构建无限制小程序码参数，确保参数的正确性。
///
/// # 示例
///
/// ```
/// use wechat_minapp::qr::{UnlimitedQrCodeArgs, Rgb, MinappEnvVersion};
///
/// let args = UnlimitedQrCodeArgs::builder()
///     .page("pages/index/index")
///     .width(300)
///     .line_color(Rgb::new(255, 0, 0))
///     .with_is_hyaline()
///     .env_version(MinappEnvVersion::Release)
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Deserialize)]
pub struct UnlimitedQrCodeArgsBuilder {
    page: Option<String>,
    scene: Option<String>,
    width: Option<i16>,
    check_path: Option<bool>,
    auto_color: Option<bool>,
    line_color: Option<Rgb>,
    is_hyaline: Option<bool>,
    env_version: Option<MinappEnvVersion>,
}

impl UnlimitedQrCodeArgs {
    pub fn builder() -> UnlimitedQrCodeArgsBuilder {
        UnlimitedQrCodeArgsBuilder::new()
    }

    pub fn page(&self) -> String {
        self.page.clone()
    }

    pub fn width(&self) -> Option<i16> {
        self.width
    }

    pub fn auto_color(&self) -> Option<bool> {
        self.auto_color
    }

    pub fn line_color(&self) -> Option<Rgb> {
        self.line_color.clone()
    }

    pub fn is_hyaline(&self) -> Option<bool> {
        self.is_hyaline
    }

    pub fn env_version(&self) -> Option<MinappEnvVersion> {
        self.env_version.clone()
    }
}

impl Default for UnlimitedQrCodeArgsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl UnlimitedQrCodeArgsBuilder {
    pub fn new() -> Self {
        UnlimitedQrCodeArgsBuilder {
            page: None,
            scene: None,
            check_path: None,
            width: None,
            auto_color: None,
            line_color: None,
            is_hyaline: None,
            env_version: None,
        }
    }

    pub fn page(mut self, page: impl Into<String>) -> Self {
        self.page = Some(page.into());
        self
    }

    pub fn scene(mut self, scene: impl Into<String>) -> Self {
        self.scene = Some(scene.into());
        self
    }

    pub fn width(mut self, width: i16) -> Self {
        self.width = Some(width);
        self
    }

    pub fn with_auto_color(mut self) -> Self {
        self.auto_color = Some(true);
        self
    }

    pub fn line_color(mut self, color: Rgb) -> Self {
        self.line_color = Some(color);
        self
    }

    pub fn with_is_hyaline(mut self) -> Self {
        self.is_hyaline = Some(true);
        self
    }

    pub fn env_version(mut self, version: MinappEnvVersion) -> Self {
        self.env_version = Some(version);
        self
    }

    pub fn build(self) -> Result<UnlimitedQrCodeArgs> {
        let page = self.page.map_or_else(
            || {
                Err(Error::InvalidParameter(
                    "小程序页面路径不能为空".to_string(),
                ))
            },
            |v| {
                let path = NonQueryPagePath::try_from(v)?;
                Ok(path.to_string())
            },
        )?;

        let scene = self.scene.map_or_else(
            || Err(Error::InvalidParameter("scene 不能为空".to_string())),
            |v| {
                let valid_scene = SceneString::try_from(v)?;
                Ok(valid_scene.to_string())
            },
        )?;

        if self.auto_color.is_some() && self.line_color.is_some() {
            return Err(Error::InvalidParameter(
                "auto_color 为 true 时，line_color 不能设置".to_string(),
            ));
        }

        Ok(UnlimitedQrCodeArgs {
            page,
            scene,
            check_path: self.check_path,
            width: self.width,
            auto_color: self.auto_color,
            line_color: self.line_color,
            is_hyaline: self.is_hyaline,
            env_version: self.env_version,
        })
    }
}

impl Qr {
    /// 生成小程序无限制小程序码
    ///
    /// 调用微信小程序无限制小程序码生成接口，返回包含无限制小程序码图片数据的 [`QrCode`] 对象。
    ///
    /// # 参数
    ///
    /// - `args`: 无限制小程序码生成参数
    ///
    /// # 返回
    ///
    /// 成功返回 `Ok(QrCode)`，失败返回错误信息。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use wechat_minapp::client::WechatMinappSDK;
    /// use wechat_minapp::qr::{UnlimitedQrCodeArgs,Qr, MinappEnvVersion};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     // 初始化客户端
    ///     let app_id = "your_app_id";
    ///     let secret = "your_app_secret";
    ///     let client = WechatMinappSDK::new(app_id, secret);
    ///     let qr = Qr::new(client);
    ///
    ///     // 构建小程序码参数
    ///     let args = UnlimitedQrCodeArgs::builder()
    ///         .page("pages/index/index")
    ///         .width(300)
    ///         .env_version(MinappEnvVersion::Release)
    ///         .build()?;
    ///
    ///     // 生成小程序码
    ///     let qr_code = qr.unlimited_qr_code(args).await?;
    ///     
    ///     // 获取小程序码图片数据
    ///     let buffer = qr_code.buffer();
    ///     println!("生成的小程序码大小: {} bytes", buffer.len());
    ///
    ///     // 可以将 buffer 保存为文件或直接返回给前端
    ///     // std::fs::write("qrcode.png", buffer)?;
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
    pub async fn unlimited_qr_code(&self, args: UnlimitedQrCodeArgs) -> Result<QrCode> {
        debug!("get unlimited qr code args {:?}", &args);

        let query = serde_json::json!({
            "access_token":self.client.token().await?
        });

        let headers = serde_json::json!({
            "encoding":"null",
            CONTENT_TYPE.to_string():"application/json"
        });

        let body = serde_json::to_value(UnlimitedQrCodeArgs {
            page: args.page,
            scene: args.scene,
            check_path: args.check_path,
            width: args.width,
            auto_color: args.auto_color,
            line_color: args.line_color,
            is_hyaline: args.is_hyaline,
            env_version: args.env_version,
        })?;

        let request = RequestBuilder::new(constants::SHORT_LINK_END_POINT)
            .headers(headers)
            .query(query)
            .body(body)
            .build()?;

        let client = &self.client.client;

        let response = client.execute(request).await?;

        debug!("get unlimited qr code response: {:#?}", response);

        if response.status().is_success() {
            Ok(QrCode {
                buffer: response.into_body(),
            })
        } else {
            let (_parts, body) = response.into_parts();
            let message = String::from_utf8_lossy(&body).to_string();
            Err(InternalServer(message))
        }
    }
}
