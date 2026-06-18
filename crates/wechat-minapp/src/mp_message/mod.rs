//! 微信服务号模板消息模块
//!
//! ## 功能
//! - [`send_message`] 发送模板消息
//!
pub mod send_message;

use crate::client::WechatMinapp;
pub use send_message::SendMessageArgs;

pub struct TemplateMessage {
    pub client: WechatMinapp,
}

impl TemplateMessage {
    pub fn new(client: WechatMinapp) -> Self {
        TemplateMessage { client }
    }
}
