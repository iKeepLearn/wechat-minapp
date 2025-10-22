use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use wechat_minapp::client::{MemoryTokenStorage, StableToken};
use wechat_minapp::client::{ReqwestHttpClient, WechatMinapp};
use wechat_minapp::link::{Link, ShortLinkArgs};

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
fn test_short_link_args_build_success() {
    let args = ShortLinkArgs::builder()
        .path("pages/index/index")
        .build()
        .expect("构建应该成功");

    assert_eq!(args.path(), "pages/index/index");
    assert!(args.page_title().is_none());
    assert_eq!(args.is_permanent(), false);
}

#[test]
fn test_short_link_args_build_with_all_fields() {
    let args = ShortLinkArgs::builder().path("pages/detail/detail");
    let args = args.page_title("page title");
    let args = args.with_permanent();
    let args = args.build().expect("构建应该成功");

    assert_eq!(args.path(), "pages/detail/detail");
    assert_eq!(args.page_title(), Some("page title".to_string()));
    assert_eq!(args.is_permanent(), true);
}

#[test]
fn test_short_link_args_build_missing_path() {
    let result = ShortLinkArgs::builder().build();
    assert!(result.is_err());

    if let Err(err) = result {
        assert!(err.to_string().contains("小程序页面路径不能为空"));
    }
}

#[test]
fn test_short_link_args_build_path_too_long() {
    let long_path = "a".repeat(1025);
    let result = ShortLinkArgs::builder().path(long_path).build();
    assert!(result.is_err());

    if let Err(err) = result {
        assert!(err.to_string().contains("页面路径最大长度 1024 个字符"));
    }
}

#[test]
fn test_short_link_args_build_path_boundary() {
    // 测试边界情况：正好 1024 个字符
    let boundary_path = "a".repeat(1024);
    let result = ShortLinkArgs::builder().path(boundary_path).build();
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_short_link_with_all_parameters() {
    let client = setup_client();
    let link = Link::new(client);

    let args = ShortLinkArgs::builder()
        .path("pages/index/index")
        .page_title("page title")
        .with_permanent()
        .build()
        .unwrap();

    let result = link.short_link(args).await;

    if result.is_err() {
        eprintln!("Error: {:?}", result);
    }

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_short_linke_with_only_path() {
    let client = setup_client();
    let link = Link::new(client);
    let args = ShortLinkArgs::builder()
        .path("pages/index/index")
        .build()
        .unwrap();

    let result = link.short_link(args).await;
    if result.is_err() {
        eprintln!("Error: {:?}", result);
    }
    assert!(result.is_ok());
}
