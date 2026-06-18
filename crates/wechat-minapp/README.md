# wechat-minapp

微信小程序服务端API SDK

[![WeChat](https://img.shields.io/badge/WeChat-07C160?style=for-the-badge&logo=wechat&logoColor=white)](https://github.com/ikeeplearn/wechat-minapp)

## 功能

- 获取访问令牌
- 用户登录凭证校验
- 解析用户信息
- 获取用户手机号
- 生成小程序码
- 内容安全检测
- 生成小程序链接
- 发送小程序模板消息

## 快速开始

```rust
use wechat_minapp::WechatMinapp;
use wechat_minapp::user::User;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化客户端
    let client = WechatMinapp::new("your_app_id", "your_app_secret");
    
    // 获取 access_token
    let token = client.token().await?;
    println!("Access Token: {}", token);
    
    // 用户登录
    let user = User::new(client);
    let credential = user.login("code").await?;
    println!("OpenID: {}", credential.open_id());
    
    Ok(())
}
```

### 生成小程序码

```rust
use wechat_minapp::WechatMinapp;
use wechat_minapp::qr::{Qr, QrCodeArgs};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = WechatMinapp::new("your_app_id", "your_app_secret");
    let qr = Qr::new(client);
    
    let args = QrCodeArgs::builder()
        .path("pages/index/index")
        .width(300)
        .build()?;
    
    let qr_code = qr.qr_code(args).await?;
    let buffer = qr_code.buffer();
    
    Ok(())
}
```

## 相关 Crates

- [wechat-core](https://crates.io/crates/wechat-core) - 核心功能库
- [wechatmp](https://crates.io/crates/wechatmp) - 服务号服务端 API

## 许可证

MIT
