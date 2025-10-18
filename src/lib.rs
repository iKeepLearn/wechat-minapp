mod access_token;
mod client;
mod credential;
mod response;
mod user;
mod qr_code;


pub mod constants;
pub mod error;

pub type Result<T> = std::result::Result<T, error::Error>;
pub use client::Client;
pub use qr_code::{QrCodeArgs,MinappEnvVersion,Rgb,QrCode};
