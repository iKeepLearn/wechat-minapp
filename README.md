# wechat-minapp

![WeChat](https://img.shields.io/badge/WeChat-07C160?style=for-the-badge&logo=wechat&logoColor=white)

A rust sdk for wechat miniprogram server api

基于 [open-wechat](https://github.com/headironc/open-wechat) 修改

首先感谢 headironc 的项目，之所以重新发一包，而不是 pr 是因为我改了很多结构，现在 wechat-minapp 的调用方式出现了很大的不同。

## Usage

### Get access token

```rust
use wechat_minapp::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = "your app id";
    let app_secret = "your app secret";

    let client = Client::new(app_id, app_secret);

    let access_token = client.access_token().await?;

    Ok(())
}
```

### Get stable access token

```rust
use wechat_minapp::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = "your app id";
    let app_secret = "your app secret";

    let client = Client::new(app_id, app_secret);
    let force_refresh = Some(true)
    let access_token = client.stable_access_token(force_refresh).await?;

    Ok(())
}
```

### Code to session

```rust
use wechat_minapp::Client;
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
    let credential = state.client.login(&logger.code).await?;

    Ok(())
}
```

### Decrypt data

```rust
use wechat_minapp::Client;
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
    let credential = state.client.login(&payload.code).await?;
    let user = credential.decrypt(&payload.encrypted_data, &payload.iv)?;

    Ok(())
}

```

### Get wx QRCode

```rust
use wechat_minapp::{Client,QrCodeArgs};
use serde::Deserialize;

use crate::{Error, state::AppState};
use actix_web::{Responder, web};


pub async fn get_user_qr(
    user: AuthenticatedUser,
    state: web::Data<AppState>,
) -> Result<impl Responder, Error> {

    let qr_args = QrCodeArgs::builder().path(&page).build()?;
        let buffer = state.client.qr_code(qr_args).await?;
    Ok(buffer)
}

```
