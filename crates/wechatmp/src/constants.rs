//! 微信服务号 API 端点常量模块

/// wechatmp crate 当前版本
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// 发送服务号模板消息的 API 端点
pub const MP_MESSAGE_SEND_END_POINT: &str =
    "https://api.weixin.qq.com/cgi-bin/message/template/send";
