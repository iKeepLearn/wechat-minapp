//! 微信小程序小程序码生成模块
pub mod minapp_code;

use crate::client::Client;
pub use minapp_code::{MinappEnvVersion, QrCode, QrCodeArgs, Rgb};

pub struct Qr {
    pub client: Box<dyn Client>,
}

impl Qr {
    pub fn new<T: Client + 'static>(client: T) -> Self {
        Qr {
            client: Box::new(client),
        }
    }
}
