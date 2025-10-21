//! 微信小程序小程序码生成模块
pub mod minapp_code;

use crate::client::WechatMinappSDK;
pub use minapp_code::{MinappEnvVersion, QrCode, QrCodeArgs, Rgb};

pub struct Qr {
    pub client: WechatMinappSDK,
}

impl Qr {
    pub fn new(client: WechatMinappSDK) -> Self {
        Qr { client }
    }
}
