//! 微信服务号模板消息模块
//!
//! ## 功能
//! - [`send_message`] 发送模板消息
//!
pub mod send_message;

pub use send_message::SendMessageArgs;
use crate::WechatMp;

pub struct TemplateMessage {
    pub client: WechatMp,
}

impl TemplateMessage {
    pub fn new(client: WechatMp) -> Self {
        TemplateMessage { client }
    }
}
