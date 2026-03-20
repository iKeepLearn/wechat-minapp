use dotenv::dotenv;
use serde_json::json;
use std::env;
use std::sync::Arc;
use wechat_minapp::client::{MemoryTokenStorage, StableToken};
use wechat_minapp::client::{ReqwestHttpClient, WechatMinapp};
use wechat_minapp::mp_message::{SendMessageArgs, TemplateMessage};

/// 初始化测试客户端
fn setup_client() -> WechatMinapp {
    dotenv().ok();

    let app_id = env::var("MP_APP_ID").expect("请设置 MP_APP_ID 环境变量");
    let secret = env::var("MP_APP_SECRET").expect("请设置 MP_APP_SECRET 环境变量");
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
        .url("https://example.com")
        .data(data)
        .build();

    assert!(result.is_ok());
    let msg = result.unwrap();
    assert_eq!(msg.touser(), "openid");
    assert_eq!(msg.template_id(), "template_id");
    assert_eq!(msg.url().unwrap(), "https://example.com");
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

    let template_id = env::var("MP_TEMPLATE_ID").expect("请设置 MP_TEMPLATE_ID 环境变量");
    let openid = env::var("MP_TEST_OPENID").expect("请设置 MP_TEST_OPENID 环境变量");
    let client = setup_client();
    let template_message = TemplateMessage::new(client);

    let args = SendMessageArgs::builder()
        .touser(openid)
        .template_id(template_id)
        .data(json!({
              "first": {
          "value": "测试",
          "color": "#bd2638",
        },
        "keyword1": {
          "value": "2026-06-01",
          "color": "#173177",
        },
        "keyword2": {
          "value": "测试标题",
          "color": "#173177",
        },
        "keyword3": {
          "value": "测试内容",
          "color": "#173177",
        },

        "remark": {
          "value": "有新私信",
          "color": "#e2244e",
        },
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
        "test_mp_message_response.json",
        serde_json::to_string_pretty(&response).unwrap(),
    )
    .await
    .expect("Failed to write response to file");
}
