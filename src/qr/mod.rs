//! 微信小程序小程序码生成模块
//!
//! # 细分模块
//!
//! - [minapp_code] 生成普通小程序码，适用于需要的码数量较少的业务场景。通过该接口生成的小程序码，永久有效，有数量限制。
//! - [unlimited_minapp_code] 不限制的小程序码,适用于需要的码数量极多的业务场景。通过该接口生成的小程序码，永久有效，数量暂无限制。
//!
//!
//!
pub mod minapp_code;
mod unlimited_minapp_code;

use crate::client::WechatMinappSDK;
pub use minapp_code::{MinappEnvVersion, QrCode, QrCodeArgs, Rgb};
pub use unlimited_minapp_code::UnlimitedQrCodeArgs;

pub struct Qr {
    pub client: WechatMinappSDK,
}

impl Qr {
    pub fn new(client: WechatMinappSDK) -> Self {
        Qr { client }
    }
}
