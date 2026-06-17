//! 微信小程序和服务号服务端API SDK - 核心功能
//!
//! 这个 crate 提供了微信 SDK 的核心功能，包括：
//! - Access Token 管理
//! - HTTP 客户端抽象
//! - Token 存储 trait
//! - 共用的错误类型
//! - 共用的常量

pub mod client;
pub mod constants;
pub mod error;
pub mod utils;

pub use error::{Error, ErrorCode};
pub type Result<T> = std::result::Result<T, Error>;
