# wechat-core

微信小程序和服务号服务端API SDK - 核心功能

[![WeChat](https://img.shields.io/badge/WeChat-07C160?style=for-the-badge&logo=wechat&logoColor=white)](https://github.com/ikeeplearn/wechat-minapp)

## 功能

- **访问令牌管理**
  - 稳定版接口调用凭据 (Stable Token)
  - 普通版接口调用凭据 (Non-Stable Token)
  - 自动刷新和过期检查

- **HTTP 客户端抽象**
  - 统一的 HTTP 请求接口
  - 默认基于 reqwest 的实现
  - 可自定义 HTTP 客户端

- **Token 存储**
  - 内存存储实现 (MemoryTokenStorage)
  - 可自定义存储后端 (Redis, PostgreSQL 等)

- **共用错误类型**
  - 微信官方错误码映射
  - 详细的错误描述

- **工具函数**
  - 请求构建器 (RequestBuilder)
  - 响应解析扩展 (ResponseExt)
  - 微信 API 响应包装 (MpResponse)

## 关于本 crate

`wechat-core` 是微信 SDK 的核心基础库，提供了：
- `wechat-minapp` - 小程序功能
- `wechatmp` - 服务号功能

这两个 crate 都基于 `wechat-core` 构建。

## 快速开始

### 基础使用

```rust
use wechat_core::client::WechatCore;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化客户端（默认使用稳定版 Token）
    let client = WechatCore::new("your_app_id", "your_app_secret");
    
    // 获取 access_token
    let token = client.token().await?;
    println!("Access Token: {}", token);
    
    Ok(())
}
```

### 自定义配置

```rust
use std::sync::Arc;
use wechat_core::client::{
    HttpClient, MemoryTokenStorage, NonStableToken, ReqwestHttpClient, TokenStorage,
    WechatCore,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 使用自定义 HTTP 客户端
    let http_client: Arc<dyn HttpClient> = Arc::new(ReqwestHttpClient::new());
    
    // 使用普通版 Token（而非稳定版）
    let token_type = Arc::new(NonStableToken::new("app_id", "secret", http_client.clone()));
    
    // 使用内存存储
    let token_storage: Arc<dyn TokenStorage> = Arc::new(MemoryTokenStorage::new(token_type));
    
    // 创建自定义客户端
    let client = WechatCore::custom(http_client, token_storage);
    
    let token = client.token().await?;
    println!("Access Token: {}", token);
    
    Ok(())
}
```

## 核心组件

### WechatCore

主要的客户端结构体，封装了 HTTP 客户端和 Token 存储。

```rust
use wechat_core::client::WechatCore;

// 创建默认客户端
let client = WechatCore::new("app_id", "secret");

// 获取 App 配置
let config = client.app_config();
println!("App ID: {}", config.app_id);
```

### HttpClient Trait

定义 HTTP 客户端行为的 Trait，可以自定义实现。

```rust
use async_trait::async_trait;
use http::{Request, Response};
use wechat_core::client::HttpClient;
use wechat_core::Result;

#[async_trait]
impl HttpClient for MyHttpClient {
    async fn execute(&self, request: Request<Vec<u8>>) -> Result<Response<Vec<u8>>> {
        // 你的实现
    }
}
```

### TokenStorage Trait

定义 Token 存储和获取行为的 Trait。

```rust
use async_trait::async_trait;
use wechat_core::client::TokenStorage;
use wechat_core::Result;

#[async_trait]
impl TokenStorage for MyStorage {
    async fn token(&self) -> Result<String> {
        // 获取 Token
    }
    
    async fn refresh_access_token(&self) -> Result<String> {
        // 刷新 Token
    }
    
    fn token_type(&self) -> Arc<dyn TokenType> {
        // 获取 TokenType
    }
}
```

### TokenType Trait

定义不同类型的访问令牌获取方式。

内置实现：
- `StableToken` - 稳定版接口调用凭据
- `NonStableToken` - 普通版接口调用凭据

## 错误处理

所有错误统一使用 `wechat_core::Error` 类型。

```rust
use wechat_core::Error;

match result {
    Ok(_) => {},
    Err(Error::InvalidCredential(msg)) => {
        eprintln!("Invalid credential: {}", msg);
    }
    Err(Error::InvalidCode(msg)) => {
        eprintln!("Invalid code: {}", msg);
    }
    Err(Error::RateLimitExceeded(msg)) => {
        eprintln!("Rate limit exceeded: {}", msg);
    }
    Err(e) => {
        eprintln!("Other error: {}", e);
    }
}
```

## 使用 wechat-minapp

如果只需要小程序功能，请使用 `wechat-minapp` crate：

```toml
[dependencies]
wechat-minapp = "4.0"
```

## 使用 wechatmp

如果只需要服务号功能，请使用 `wechatmp` crate：

```toml
[dependencies]
wechatmp = "1.0"
```

## 相关 Crates

- [wechat-minapp](https://crates.io/crates/wechat-minapp) - 小程序服务端 API
- [wechatmp](https://crates.io/crates/wechatmp) - 服务号服务端 API

## 许可证

MIT
