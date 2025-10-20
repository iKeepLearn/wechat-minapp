# wechat-minapp

![WeChat](https://img.shields.io/badge/WeChat-07C160?style=for-the-badge&logo=wechat&logoColor=white)

A rust sdk for wechat miniprogram server api

基于 [open-wechat](https://github.com/headironc/open-wechat) 修改

首先感谢 headironc 的项目，之所以重新发一包，而不是 pr 是因为我改了很多结构，现在 wechat-minapp 的调用方式出现了很大的不同。

1.x 版本的说明请切换到 v1 分支查看

## 用法

### 获取 access token

```rust
use use wechat_minapp::client::NonStableTokenClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = "your app id";
    let app_secret = "your app secret";

    let client = NonStableTokenClient::new(app_id, app_secret);

    let access_token = client.token().await?;

    Ok(())
}
```

### 获取 stable access token

```rust
use wechat_minapp::client::StableTokenClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = "your app id";
    let app_secret = "your app secret";

    let client = StableTokenClient::new(app_id, app_secret);
    let access_token = client.token().await?;

    // 需要强制刷新时可用下面的方法
    let mut client = StableTokenClient::new(app_id, app_secret);
    let client.with_force_refresh();
    let access_token = client.token().await?;

    Ok(())
}
```

### 登录

```rust
use wechat_minapp::client::StableTokenClient;
use wechat_minapp::user::{User, Contact};
use serde::Deserialize;

use crate::{Error, state::AppState};
use actix_web::{Responder, web};

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct Logger {
    code: String,
}

pub async fn login(
    state: web::Data<AppState>,
    logger: web::Json<Logger>,
) -> Result<impl Responder, Error> {
    let user = User::new(state.client);
    let credential = user.login(&logger.code).await?;

    Ok(())
}
```

### 解码用户信息

```rust
use wechat_minapp::client::StableTokenClient;
use wechat_minapp::user::{User, Contact};
use serde::Deserialize;

use crate::{Error, state::AppState};
use actix_web::{Responder, web};

#[derive(Deserialize, Default)]
pub struct EncryptedPayload {
    code: String,
    encrypted_data: String,
    iv: String,
}

pub async fn decrypt(
    state: web::Data<AppState>,
    payload: web::Json<EncryptedPayload>,
) -> Result<impl Responder, Error> {
     let user = User::new(state.client);
    let credential = user.login(&logger.code).await?;
    let user = credential.decrypt(&payload.encrypted_data, &payload.iv)?;

    Ok(())
}

```

### 获取用户手机号

```rust
use wechat_minapp::client::StableTokenClient;
use wechat_minapp::user::User;
use serde::Deserialize;

use crate::{Error, state::AppState};
use actix_web::{Responder, web};

#[derive(Deserialize, Default)]
pub struct PhonePayload {
    code: String,
    openid: Option<String>,
}

pub async fn get_phone_num(
    state: web::Data<AppState>,
    payload: web::Json<PhonePayload>,
) -> Result<impl Responder, Error> {
     let user = User::new(state.client);
     let phone = user.get_contact(&payload.code,&payload.openid).await?;

    Ok(web::Json(phone))
}

```

### 生成小程序码

```rust
use use wechat_minapp::client::StableTokenClient;
use wechat_minapp::qr::{QrCodeArgs,Qr, MinappEnvVersion};
use serde::Deserialize;

use crate::{Error, state::AppState};
use actix_web::{Responder, web};


pub async fn get_user_qr(
    user: AuthenticatedUser,
    state: web::Data<AppState>,
) -> Result<impl Responder, Error> {

    let page:&str = "/index";
    let qr_args = QrCodeArgs::builder().path(&page).build()?;
    let qr = Qr::new(state.client);
    let buffer = qr.qr_code(qr_args).await?;

    Ok(buffer)
}

```


### 检查文本内容安全

```rust
use wechat_minapp::minapp_security::{Args, Scene,MinappSecurity};
use wechat_minapp::client::StableTokenClient;
use crate::{Error, state::AppState};
use actix_web::{Responder, web};
use serde::{Deserialize,Serialize};

#[derive(Deserialize,Serialize)]
pub struct ContentCheck {
    content: String,
    scene: Scene,
}

pub async fn get_user_qr(
    user: AuthenticatedUser,
    state: web::Data<AppState>,
    date:web::Json<ContentCheck>
) -> Result<impl Responder, Error> {

let args = Args::builder()
        .content(&data.content)
        .scene(data.scene)
        .openid(user.openid)
        .build()?;
    let security = MinappSecurity::new(state.client);
    let result = security.msg_sec_check(args).await?;
    
    if result.is_pass() {
        println!("内容安全，可以发布");
    } else if result.needs_review() {
        println!("内容需要人工审核");
    } else {
        println!("内容有风险，建议修改");
    }
    
    Ok(web::Json(result))
}

```
