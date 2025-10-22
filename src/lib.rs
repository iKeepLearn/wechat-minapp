//! 微信小程序服务端常用接口的 RUST SDK
//!
//! # 特性
//! - 异步支持
//! - 丰富的接口支持
//! - HTTP 客户端和接口调用凭据存储读取方式分离，可以按自己的需求实现不同的 HTTP 客户端和接口调用凭据存储读取方式。
//! - 支持稳定版和普通版访问令牌
//! - 良好的错误处理
//! - 简单易用的 API
//! - 详细的文档
//! - 单元测试覆盖
//!
//!
//! # 快速开始
//!
//! ## 默认客户端和存储方式
//! ```no_run
//! use wechat_minapp::client::WechatMinappSDK;
//! let client = WechatMinappSDK::new("your_app_id", "your_app_secret");
//! ```   
//!
//! ## 自定义 HTTP 客户端和存储方式
//! ```no_run
//! use wechat_minapp::client::{MemoryTokenStorage, StableToken};
//! use wechat_minapp::client::{ReqwestHttpClient, WechatMinappSDK};
//!
//! let http_client = Arc::new(ReqwestHttpClient::new());
//! let token_type = Arc::new(StableToken::new(
//!        &app_id,
//!        &secret,
//!        false,
//!        http_client.clone(),
//!    ));
//! let token_storage = Arc::new(MemoryTokenStorage::new(token_type));
//! let client = WechatMinappSDK::custom(http_client, token_storage)
//!
//! ```
//!
//! # 功能
//!
//! - 获取访问令牌
//! - 用户登录凭证校验
//! - 解析用户信息
//! - 获取用户手机号
//! - 生成小程序码
//! - 内容安全检测  
//! - 生成小程序链接
//!         
//!     

mod response;

pub mod client;
pub mod constants;
pub mod error;
pub mod minapp_security;
pub mod qr;
pub mod user;
pub mod link;
pub type Result<T> = std::result::Result<T, error::Error>;
