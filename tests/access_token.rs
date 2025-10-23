use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use wechat_minapp::client::{MemoryTokenStorage, StableToken};
use wechat_minapp::client::{ReqwestHttpClient, WechatMinapp};

/// 初始化测试客户端
fn setup_client() -> WechatMinapp {
    dotenv().ok();

    let app_id = env::var("WECHAT_APP_ID").expect("请设置 WECHAT_APP_ID 环境变量");
    let secret = env::var("WECHAT_APP_SECRET").expect("请设置 WECHAT_APP_SECRET 环境变量");
    let http_client = Arc::new(ReqwestHttpClient::new());
    let token_type = Arc::new(StableToken::new(
        &app_id,
        &secret,
        false,
        http_client.clone(),
    ));
    let token_storage = Arc::new(MemoryTokenStorage::new(token_type));
    WechatMinapp::custom(http_client, token_storage)
}

#[tokio::test]
async fn test_get_access_token() {
    let client = setup_client();
    let token = client.token().await;

    assert!(token.is_ok());
    let token = token.unwrap();
    assert!(!token.is_empty());
}
