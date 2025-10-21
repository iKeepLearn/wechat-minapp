//! 微信小程序用户信息模块
//!
//! 该模块提供了获取和处理微信小程序用户信息的功能，包括用户基本信息和手机号信息。
//!
//! # 登录信息
//!
//! ```no_run
//! use wechat_minapp::client::WechatMinappSDK;
//! use wechat_minapp::user::{User, Contact};
//!
//!  #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = WechatMinappSDK::new("app_id", "secret");
//!     let user = User::new(client);
//!     let code = "0816abc123def456";
//!     let credential = user.login(code).await?;
//!
//!     println!("用户OpenID: {}", credential.open_id());
//!     println!("会话密钥: {}", credential.session_key());
//!     
//!     Ok(())
//! }
//! ```
//!
//! 解析用户基本信息（需要前端传递加密数据）
//! ```no_run
//! use wechat_minapp::client::WechatMinappSDK;
//! use wechat_minapp::user::{User, Contact};
//!
//!  #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = WechatMinappSDK::new("app_id", "secret");
//!     let user = User::new(client);
//!     let code = "0816abc123def456";
//!     let credential = user.login(code).await?;
//!     let info = credential.decrypt(&encrypted_data, &iv)?;
//!     println!("昵称: {}", info.nickname());
//!     println!("性别: {}", info.gender());
//!     println!("地区: {}-{}-{}", info.country(), info.province(), info.city());
//!     println!("头像: {}", info.avatar());
//!     println!("AppID: {}", info.app_id());
//!     println!("时间戳: {}", info.timestamp());
//!     
//!     Ok(())
//! }
//! ```
//!
//!  获取用户手机号
//! ```no_run
//! use wechat_minapp::client::WechatMinappSDK;
//! use wechat_minapp::user::{User, Contact};
//!
//!  #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = WechatMinappSDK::new("app_id", "secret");
//!     let user = User::new(client);
//!     let code = "0816abc123def456";
//!     let contact = user.get_contact(code, None).await?;
//!     let info = credential.decrypt(&encrypted_data, &iv)?;
//!     println!("用户手机号: {}", contact.phone_number());
//!     
//!     Ok(())
//! }
//! ```
//!
mod credential;
mod user_info;
use crate::client::WechatMinappSDK;

pub struct User {
    pub client: WechatMinappSDK,
}

impl User {
    pub fn new(client: WechatMinappSDK) -> Self {
        User { client }
    }
}
