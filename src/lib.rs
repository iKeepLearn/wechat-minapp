//! `wechat_minapp` - 微信小程序服务端 API 封装库
//!
//! 这是一个为微信小程序服务端 API 提供的 Rust 封装库，旨在简化与微信小程序后端的交互。
//! 提供了诸如用户登录、内容安全检测、小程序码生成等常用功能的易用接口。
//!
//! ## 主要模块
//!
//! - [`client`]: 核心客户端，负责 API 请求和访问令牌管理。
//! - [`user`][]: 用户信息管理，包括用户数据解密和手机号获取。
//! - [`minapp_security`][]: 内容安全检测，用于审核用户生成的内容。
//! - [`qr_code`][]: 小程序码生成，支持自定义参数和环境。
//!
//! ## 核心特性
//!
//! - **易用性**: 提供简洁的 API 和链式构建器，简化开发流程。
//! - **安全性**: 自动处理访问令牌的获取和刷新，保障数据安全。
//! - **可靠性**: 针对网络请求和 API 错误进行处理，提供稳定的服务。
//! - **灵活性**: 支持自定义 HTTP 客户端配置，方便集成和测试。
//! - **并发性**: 支持在并发环境中使用。
//!
//! ## 快速开始
//!
//! ```no_run
//! use wechat_minapp::{Client, minapp_security::{Args, Scene}, QrCodeArgs};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // 初始化客户端
//!     let client = Client::new("your_app_id", "your_app_secret");
//!
//!     // 1) 登录（code -> openid, session_key）
//!     let cred = client.login("code").await?;
//!
//!     // 2) 内容安全检测
//!     let args = Args::builder()
//!         .content("需要检测的文本内容")
//!         .scene(Scene::Comment)
//!         .openid("user_openid")
//!         .build()?;
//!     let sec = client.msg_sec_check(&args).await?;
//!     if sec.is_pass() {
//!         println!("内容通过");
//!     }
//!
//!     // 3) 生成小程序码
//!     let qr_args = QrCodeArgs::builder()
//!         .path("pages/index/index")
//!         .width(300)
//!         .build()?;
//!     let qr = client.qr_code(qr_args).await?;
//!     std::fs::write("qrcode.png", qr.buffer())?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## 模块详解
//!
//! ### [`client`] 模块
//!
//! 提供了与微信小程序后端 API 交互的核心功能。
//!
//! - [`Client`]: 客户端实例，用于发起 API 请求。
//! - `login()`: 用户登录凭证校验，换取 OpenID 和 Session Key。
//! - `access_token()`: 获取访问令牌，自动处理令牌刷新。
//! - `stable_access_token()`: 获取稳定版访问令牌，有效期更长。
//!
//! ### [`user`][] 模块
//!
//! 提供了获取和处理微信小程序用户信息的功能。
//!
//! - [`User`][]: 微信用户基本信息，包括昵称、性别、地区等。
//! - [`Contact`][]: 用户手机号信息，需要前端传递 code 获取。
//! - `get_contact()`: 通过 code 换取用户手机号。
//!
//! ### [`minapp_security`] 模块
//!
//! 提供了微信小程序内容安全检测功能，用于检测文本内容是否包含违规信息。
//!
//! - [`Args`][minapp_security]: 内容安全检测参数，包括检测内容、场景、用户 OpenID 等。
//! - [`Scene`][minapp_security]: 内容安全检测场景，如资料、评论、论坛等。
//! - `msg_sec_check()`: 发起内容安全检测请求。
//! - [`MsgSecCheckResult`][minapp_security]: 内容安全检测结果，包含详细的检测信息。
//!
//! ### [`qr_code`] 模块
//!
//! 提供了生成微信小程序码的功能，支持自定义参数和环境。
//!
//! - [`QrCodeArgs`][]: 小程序码生成参数，包括页面路径、宽度、颜色等。
//! - [`MinappEnvVersion`][]: 小程序环境版本，如开发版、体验版、正式版。
//! - `qr_code()`: 生成小程序码图片数据。
//! - [`QrCode`][]: 小程序码图片数据，可以保存为文件或直接返回给前端。
//!
//! ## 错误处理
//!
//! 所有 API 调用都可能返回 [`Result`]，其中 [`Error`][error] 枚举类型包含了各种可能的错误情况，
//! 例如网络错误、API 错误、参数错误等。建议在应用层统一处理这些错误。
//!

mod access_token;
mod client;
mod credential;
mod qr_code;
mod response;

pub mod constants;
pub mod error;
pub mod minapp_security;
pub mod user;

pub type Result<T> = std::result::Result<T, error::Error>;
pub use client::Client;
pub use qr_code::{MinappEnvVersion, QrCode, QrCodeArgs, Rgb};
