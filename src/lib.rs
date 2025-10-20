pub mod client;
pub mod constants;
pub mod error;
pub mod minapp_security;
pub mod qr;
mod response;
pub mod user;
pub type Result<T> = std::result::Result<T, error::Error>;
