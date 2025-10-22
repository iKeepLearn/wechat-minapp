mod dto;
pub mod error;
pub mod routes;

use wechat_minapp::client::WechatMinappSDK;
use error::Error;

#[derive(Debug, Clone)]
pub struct MinappConfig {
    pub secret_id: String,
    pub secret_key: String,
}

#[derive(Debug, Clone)]
pub struct WechatMinapp {
    client: WechatMinappSDK,
}


#[derive(Debug, Clone)]
pub struct AppState {
    wechat_minapp: WechatMinapp,
}

impl AppState {
    pub fn new(config:&MinappConfig) -> Self {
        let client=WechatMinappSDK::new(&config.secret_id,&config.secret_key);
        let wechat_minapp = WechatMinapp{
            client
        };
        AppState {
            wechat_minapp,
        }
    }
}
impl WechatMinapp {
    pub fn new(config: &MinappConfig) -> Self {
        let client = WechatMinappSDK::new(&config.secret_id, &config.secret_key);

        WechatMinapp { client }
    }
    pub fn client(&self) -> WechatMinappSDK {
        self.client.clone()
    }
}

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub port: u16,
    pub minapp: MinappConfig,
}

const DEFAULT_PORT: u16 = 9999;

impl AppConfig {
    pub fn load() -> Result<AppConfig, Error> {
        dotenv::dotenv().ok();

        let port = std::env::var("PORT")
            .ok()
            .map_or(Ok(DEFAULT_PORT), |env_val| env_val.parse::<u16>())?;

        let secret_id = std::env::var("WECHAT_APP_ID").unwrap();
        let secret_key = std::env::var("WECHAT_APP_SECRET").unwrap();
        let minapp = MinappConfig {
            secret_id,
            secret_key,
        };

        Ok(AppConfig { port, minapp })
    }
}
