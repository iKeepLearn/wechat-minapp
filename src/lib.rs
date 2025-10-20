mod access_token;
mod client;
mod credential;
mod qr_code;
mod response;
mod user;

pub mod constants;
pub mod error;
pub mod minapp_security;

pub type Result<T> = std::result::Result<T, error::Error>;
pub use client::Client;
pub use qr_code::{MinappEnvVersion, QrCode, QrCodeArgs, Rgb};
