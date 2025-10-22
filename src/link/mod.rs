//! 微信小程序小程序链接生成模块
//! 
//! ## 功能
//! - [`short_link`] 生成电商短链接
//! 
pub mod short_link;

use crate::client::WechatMinappSDK;
pub use short_link::{ShortLink, ShortLinkArgs};

pub struct Link {
    pub client: WechatMinappSDK,
}

impl Link {
    pub fn new(client: WechatMinappSDK) -> Self {
        Link { client }
    }
}
