use dotenv::dotenv;
use serde_json::json;
use std::env;
use std::sync::Arc;
use wechat_minapp::client::{MemoryTokenStorage, StableToken};
use wechat_minapp::client::{ReqwestHttpClient, WechatMinapp};
use wechat_minapp::template_message::{SendMessageArgs, TemplateMessage};

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

#[test]
fn test_template_message_builder() {
    let data = json!({
        "thing1": {"value": "测试"},
        "amount2": {"value": "¥99.00"}
    });

    let result = SendMessageArgs::builder()
        .touser("openid")
        .template_id("template_id")
        .page("pages/index/index")
        .data(data)
        .miniprogram_state("formal")
        .lang("zh_CN")
        .build();

    assert!(result.is_ok());
    let msg = result.unwrap();
    assert_eq!(msg.touser(), "openid");
    assert_eq!(msg.template_id(), "template_id");
    assert_eq!(msg.page().unwrap(), "pages/index/index");
    assert_eq!(msg.miniprogram_state().unwrap(), "formal");
    assert_eq!(msg.lang().unwrap(), "zh_CN");
}

#[test]
fn test_template_message_validation() {
    // 测试缺少必要字段
    let result = SendMessageArgs::builder().touser("openid").build();
    assert!(result.is_err());

    // 测试字段名过长
    let data = json!({
        "this_is_a_very_long_field_name_that_exceeds_twenty_chars": {"value": "test"}
    });
    let result = SendMessageArgs::builder()
        .touser("openid")
        .template_id("template_id")
        .data(data)
        .build();
    assert!(result.is_err());

    // 测试值过长
    let data = json!({
        "thing1": {"value": "a".repeat(51)}
    });
    let result = SendMessageArgs::builder()
        .touser("openid")
        .template_id("template_id")
        .data(data)
        .build();
    assert!(result.is_err());

    // 测试格式错误
    let data = json!({
        "thing1": "test"
    });
    let result = SendMessageArgs::builder()
        .touser("openid")
        .template_id("template_id")
        .data(data)
        .build();
    assert!(result.is_err());
}

#[tokio::test]
async fn test_send_message() {
    dotenv().ok();

    let template_id = env::var("TEMPLATE_ID").expect("请设置 TEMPLATE_ID 环境变量");
    let openid = env::var("OPENID").expect("请设置 OPENID 环境变量");
    let client = setup_client();
    let template_message = TemplateMessage::new(client);

    let args = SendMessageArgs::builder()
        .touser(openid)
        .template_id(template_id)
        .data(json!({
            "thing6": {"value": "测试"},
            "date4": {"value": "2026-06-01"},
            "thing7": {"value": "¥99.00"}
        }))
        .build()
        .unwrap();

    let result = template_message.send_message(args).await;

    if result.is_err() {
        eprintln!("Error: {:?}", result);
    }

    assert!(result.is_ok());

    let response = result.unwrap();
    tokio::fs::write(
        "test_send_message_response.json",
        serde_json::to_string_pretty(&response).unwrap(),
    )
    .await
    .expect("Failed to write response to file");
}
