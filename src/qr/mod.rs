//! 微信小程序小程序码生成模块
pub mod minapp_code;

use crate::client::Client;
pub use minapp_code::{MinappEnvVersion, QrCode, QrCodeArgs, Rgb};

pub struct Qr<'a> {
    pub client: Box<&'a dyn Client>,
}

impl<'a> Qr<'a> {
    pub fn new<T: Client + 'static>(client: &'a T) -> Self {
        Qr {
            client: Box::new(client),
        }
    }
}
