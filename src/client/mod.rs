//! 微信小程序服务端接口 Client 模块
//!
//! 分为使用普通接口调用凭据和稳定版接口调用凭据
//!
//! 普通接口调用凭据
//!  ```no_run
//！ use wechat_minapp::client::NonStableTokenClient;
//！
//！ let client = NonStableTokenClient::new("your_appid", "your_app_secret_here");
//！ ```
//！稳定版接口调用凭据
//! ```no_run
//！ use wechat_minapp::client::StableTokenClient;
//！
//！ let client = StableTokenClient::new("your_appid", "your_app_secret_here");
//！ ```
//!
mod access_token;
mod non_stable_token;
mod stable_token;

pub use access_token::AccessToken;
pub use non_stable_token::NonStableTokenClient;
pub use stable_token::StableTokenClient;

use crate::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Client: Send + Sync {
    async fn token(&self) -> Result<String>;
    fn inner_client(&self) -> &ClientInner;
}

#[derive(Debug, Clone)]
pub struct ClientInner {
    pub app_id: String,
    pub secret: String,
    pub client: reqwest::Client,
}
