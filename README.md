# wechat-minapp

[![WeChat](https://img.shields.io/badge/WeChat-07C160?style=for-the-badge&logo=wechat&logoColor=white)](https://github.com/ikeeplearn/wechat-minapp)

微信小程序和服务号服务端API SDK

## 项目结构

本项目已拆分为多个 crate，便于按需使用：

```
wechat-minapp/
├── wechat-core/      # 核心功能（AccessToken, HTTP 客户端）
├── wechat-minapp/    # 小程序 API
└── wechatmp/        # 服务号 API
```

### Crates

- **[wechat-core](./crates/wechat-core)** - 核心功能库，提供访问令牌管理和 HTTP 客户端抽象
- **[wechat-minapp](./crates/wechat-minapp)** - 小程序服务端 API
- **[wechatmp](./crates/wechatmp)** - 服务号服务端 API

## 功能

### wechat-minapp（小程序）

- 获取访问令牌
- 用户登录凭证校验
- 解析用户信息
- 获取用户手机号
- 生成小程序码
- 内容安全检测
- 生成小程序链接
- 发送小程序模板消息

### wechatmp（服务号）

- 获取访问令牌
- 发送服务号模板消息

## 快速开始

### 使用小程序功能

```toml
[dependencies]
wechat-minapp = "4.0"
```

```rust
use wechat_minapp::WechatMinapp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = WechatMinapp::new("your_app_id", "your_app_secret");
    let token = client.token().await?;
    Ok(())
}
```

### 使用服务号功能

```toml
[dependencies]
wechatmp = "1.0"
```

```rust
use wechat_mp::WechatMp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = WechatMp::new("your_app_id", "your_app_secret");
    let token = client.token().await?;
    Ok(())
}
```


## 详细文档

各 crate 的详细文档请参考对应目录下的 README：

- [wechat-core 文档](./crates/wechat-core/README.md)
- [wechat-minapp 文档](./crates/wechat-minapp/README.md)
- [wechatmp 文档](./crates/wechatmp/README.md)

## 许可证

MIT
