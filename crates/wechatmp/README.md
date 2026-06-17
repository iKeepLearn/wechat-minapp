# wechat-mp

微信服务号服务端API SDK

[![WeChat](https://img.shields.io/badge/WeChat-07C160?style=for-the-badge&logo=wechat&logoColor=white)](https://github.com/ikeeplearn/wechat-minapp)

## 功能

- 获取访问令牌
- 发送服务号模板消息

## 快速开始

```rust
use wechat_mp::WechatMp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化客户端
    let client = WechatMp::new("your_app_id", "your_app_secret");
    
    // 获取 access_token
    let token = client.token().await?;
    println!("Access Token: {}", token);
    
    Ok(())
}
```

### 发送模板消息

```rust
use wechat_mp::WechatMp;
use wechat_mp::mp_message::{TemplateMessage, TemplateMessageArgs};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = WechatMp::new("your_app_id", "your_app_secret");
    let template = TemplateMessage::new(client);
    
    let args = TemplateMessageArgs::builder()
        .touser("openid")
        .template_id("template_id")
        .url("https://example.com")
        .build()?;
    
    let result = template.send_message(args).await?;
    
    Ok(())
}
```

## 相关 Crates

- [wechat-core](https://crates.io/crates/wechat-core) - 核心功能库
- [wechat-minapp](https://crates.io/crates/wechat-minapp) - 小程序服务端 API

## 许可证

MIT
