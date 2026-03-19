//! 微信小程序模板消息模块
//!
//! ## 功能
//! - [`send_message`] 发送模板消息
//!
pub mod send_message;

use crate::client::WechatMinappSDK;
pub use send_message::SendMessageArgs;

pub struct TemplateMessage {
    pub client: WechatMinappSDK,
}

impl TemplateMessage {
    pub fn new(client: WechatMinappSDK) -> Self {
        TemplateMessage { client }
    }
}
