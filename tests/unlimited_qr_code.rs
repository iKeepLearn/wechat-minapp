use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use wechat_minapp::client::{MemoryTokenStorage, StableToken};
use wechat_minapp::client::{ReqwestHttpClient, WechatMinapp};
use wechat_minapp::qr::{MinappEnvVersion, Qr, Rgb, UnlimitedQrCodeArgs};

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

/// 获取测试用的用户openid
fn get_test_openid() -> String {
    env::var("WECHAT_TEST_OPENID").unwrap_or_else(|_| "test_openid_placeholder".to_string())
}

#[test]
fn test_minapp_env_version_conversion() {
    let develop: String = MinappEnvVersion::Develop.into();
    let release: String = MinappEnvVersion::Release.into();
    let trial: String = MinappEnvVersion::Trial.into();

    assert_eq!(develop, "develop");
    assert_eq!(release, "release");
    assert_eq!(trial, "trial");
}

#[test]
fn test_unlimited_qr_code_args_build_success() {
    let args = UnlimitedQrCodeArgs::builder()
        .page("pages/index/index")
        .scene(format!("invite={}", get_test_openid()))
        .build()
        .expect("构建应该成功");

    assert_eq!(args.page(), "pages/index/index");
    assert!(args.width().is_none());
    assert!(args.auto_color().is_none());
    assert!(args.line_color().is_none());
    assert!(args.is_hyaline().is_none());
    assert!(args.env_version().is_none());
}

#[test]
fn test_unlimited_qr_code_args_build_with_all_fields() {
    let args = UnlimitedQrCodeArgs::builder()
        .page("pages/detail/detail")
        .scene(format!("invite={}", get_test_openid()))
        .width(400)
        // .with_auto_color()
        .line_color(Rgb::new(0, 255, 0))
        .with_is_hyaline()
        .env_version(MinappEnvVersion::Develop)
        .build()
        .expect("构建应该成功");

    assert_eq!(args.page(), "pages/detail/detail");
    assert_eq!(args.width(), Some(400));
    assert!(args.line_color().is_some());
    assert_eq!(args.is_hyaline(), Some(true));
    assert!(matches!(
        args.env_version(),
        Some(MinappEnvVersion::Develop)
    ));
}

#[test]
fn test_unlimited_qr_code_args_build_missing_path() {
    let result = UnlimitedQrCodeArgs::builder()
        .scene(format!("invite={}", get_test_openid()))
        .build();
    assert!(result.is_err());

    if let Err(err) = result {
        assert!(err.to_string().contains("小程序页面路径不能为空"));
    }
}

#[test]
fn test_unlimited_qr_code_args_build_missing_scene() {
    let result = UnlimitedQrCodeArgs::builder()
        .page("page/index/index")
        .build();
    assert!(result.is_err());

    if let Err(err) = result {
        assert!(err.to_string().contains("scene 不能为空"));
    }
}

#[test]
fn test_unlimited_qr_code_args_build_path_too_long() {
    let long_path = "a".repeat(1025);
    let result = UnlimitedQrCodeArgs::builder()
        .page(long_path)
        .scene(format!("invite={}", get_test_openid()))
        .build();
    assert!(result.is_err());

    if let Err(err) = result {
        assert!(err.to_string().contains("页面路径最大长度 1024 个字符"));
    }
}

#[test]
fn test_unlimited_qr_code_args_build_scene_too_long() {
    let long_scene = "a".repeat(1025);
    let result = UnlimitedQrCodeArgs::builder()
        .page("page/index/index")
        .scene(long_scene)
        .build();
    assert!(result.is_err());

    if let Err(err) = result {
        assert!(err.to_string().contains("页面路径最大长度 1024 个字符"));
    }
}

#[test]
fn test_unlimited_qr_code_args_build_path_boundary() {
    // 测试边界情况：正好 1024 个字符
    let boundary_path = "a".repeat(1024);
    let result = UnlimitedQrCodeArgs::builder()
        .page(boundary_path)
        .scene(format!("invite={}", get_test_openid()))
        .build();
    assert!(result.is_ok());
}

#[test]
fn test_unlimited_qr_code_args_build_scene_boundary() {
    // 测试边界情况：正好 32 个字符
    let boundary_scene = "a".repeat(1024);
    let result = UnlimitedQrCodeArgs::builder()
        .page("page/index/index")
        .scene(boundary_scene)
        .build();
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_unlimited_qr_code_with_all_parameters() {
    let client = setup_client();
    let qr = Qr::new(client);

    let args = UnlimitedQrCodeArgs::builder()
        .page("pages/index/index")
        .scene(format!("invite={}", get_test_openid()))
        .width(300)
        .line_color(Rgb::new(255, 0, 0))
        .with_is_hyaline()
        .env_version(MinappEnvVersion::Release)
        .build()
        .unwrap();

    let result = qr.unlimited_qr_code(args).await;

    assert!(result.is_ok());
    let qr_code = result.unwrap();
    tokio::fs::write(
        "test_unlimited_qr_code_with_all_parameters.png",
        qr_code.buffer(),
    )
    .await
    .expect("写入文件失败");
    assert!(!qr_code.buffer().is_empty());
}

#[tokio::test]
async fn test_unlimited_qr_code_with_only_width() {
    let client = setup_client();
    let qr = Qr::new(client);
    let args = UnlimitedQrCodeArgs::builder()
        .page("pages/index/index")
        .scene(format!("invite={}", get_test_openid()))
        .width(200)
        .build()
        .unwrap();

    let result = qr.unlimited_qr_code(args).await;
    assert!(result.is_ok());
    let qr_code = result.unwrap();
    tokio::fs::write(
        "test_unlimited_qr_code_with_only_width.png",
        qr_code.buffer(),
    )
    .await
    .expect("写入文件失败");
    assert!(!qr_code.buffer().is_empty());
}

#[tokio::test]
async fn test_unlimited_qr_code_with_only_env_version() {
    let client = setup_client();
    let qr = Qr::new(client);
    let args = UnlimitedQrCodeArgs::builder()
        .page("pages/index/index")
        .scene(format!("invite={}", get_test_openid()))
        .env_version(MinappEnvVersion::Develop)
        .build()
        .unwrap();

    let result = qr.unlimited_qr_code(args).await;
    assert!(result.is_ok());
    let qr_code = result.unwrap();
    tokio::fs::write(
        "test_unlimited_qr_code_with_only_env_version.png",
        qr_code.buffer(),
    )
    .await
    .expect("写入文件失败");
    assert!(!qr_code.buffer().is_empty());
}
