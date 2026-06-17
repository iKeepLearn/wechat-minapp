//! 微信小程序 API 端点常量模块 - 核心共用常量

/// wechat-core crate 当前版本
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// base url
pub const END_POINT_BASE_URL: &str = "https://api.weixin.qq.com";

/// HTTP 客户端的 User-Agent 字符串
pub const HTTP_CLIENT_USER_AGENT: &str =
    "wechat-core/3.4.3 (A rust sdk for wechat miniprogram and mp server api)";

/// 获取稳定版访问令牌的 API 端点
pub const STABLE_ACCESS_TOKEN_END_POINT: &str = "https://api.weixin.qq.com/cgi-bin/stable_token";

/// 获取普通访问令牌的 API 端点
pub const ACCESS_TOKEN_END_POINT: &str = "https://api.weixin.qq.com/cgi-bin/token";
