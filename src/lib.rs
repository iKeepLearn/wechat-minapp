mod access_token;
mod client;
mod credential;
mod qr_code;
mod response;

pub mod constants;
pub mod error;
pub mod minapp_security;
pub mod user;

pub type Result<T> = std::result::Result<T, error::Error>;
pub use client::Client;
pub use qr_code::{MinappEnvVersion, QrCode, QrCodeArgs, Rgb};
