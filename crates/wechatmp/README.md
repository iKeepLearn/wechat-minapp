# wechat-mp

微信服务号服务端 API SDK

[![WeChat](https://img.shields.io/badge/WeChat-07C160?style=for-the-badge&logo=wechat&logoColor=white)](https://github.com/ikeeplearn/wechat-minapp)

## 功能

- **Access Token** - 获取访问令牌
- **模板消息** - 发送模板消息、一次性订阅消息、行业管理、模板列表、拦截查询
- **订阅通知** - 发送订阅通知、模板选用/删除/列表/关键词/类目查询
- **群发消息** - 按标签/OpenID 群发、删除、状态查询、速度控制、素材上传、预览
- **自动回复** - 获取自动回复规则

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

## 使用示例

### 发送模板消息

```rust
use wechat_mp::WechatMp;
use wechat_mp::mp_message::{TemplateMessage, SendMessageArgs};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = WechatMp::new("your_app_id", "your_app_secret");
    let template = TemplateMessage::new(client);

    let data = json!({
        "thing1": {"value": "订单支付成功"},
        "amount2": {"value": "¥99.00"},
    });

    let args = SendMessageArgs::builder()
        .touser("openid")
        .template_id("template_id")
        .data(data)
        .build()?;

    let result = template.send_message(args).await?;

    Ok(())
}
```

### 一次性订阅消息 & 行业管理

```rust
use wechat_mp::WechatMp;
use wechat_mp::mp_message::{TemplateMessage, TemplateSubscribeArgs, SetIndustryArgs, SubscribeMiniprogram};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = WechatMp::new("your_app_id", "your_app_secret");
    let tmpl = TemplateMessage::new(client);

    // 获取行业信息
    let industry = tmpl.get_industry().await?;

    // 设置所属行业
    tmpl.set_industry(SetIndustryArgs {
        industry_id1: "1".into(),
        industry_id2: "4".into(),
    }).await?;

    // 获取已选用模板列表
    let templates = tmpl.get_all_templates().await?;

    // 发送一次性订阅消息
    let args = TemplateSubscribeArgs {
        touser: "openid".into(),
        template_id: "template_id".into(),
        url: None,
        miniprogram: Some(SubscribeMiniprogram {
            appid: "appid".into(),
            pagepath: "index?foo=bar".into(),
        }),
        scene: "1000".into(),
        title: "订阅通知".into(),
        data: json!({"content": {"value": "您好！", "color": "#FF0000"}}),
    };
    tmpl.template_subscribe(args).await?;

    Ok(())
}
```

### 发送订阅通知 & 模板管理

```rust
use wechat_mp::WechatMp;
use wechat_mp::mp_message::{SubscribeNotify, SendNewSubscribeMsgArgs, AddTemplateArgs};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = WechatMp::new("your_app_id", "your_app_secret");
    let notify = SubscribeNotify::new(client);

    // 获取类目
    let categories = notify.get_category().await?;

    // 获取类目下的公共模板
    let titles = notify.get_pub_template_titles("123,456", 0, 20).await?;

    // 获取模板关键词
    let keywords = notify.get_template_keywords("tid").await?;

    // 选用模板
    let result = notify.add_template(AddTemplateArgs {
        tid: "tid".into(),
        kid_list: vec![3, 5, 4],
        scene_desc: "订单通知".into(),
    }).await?;

    // 发送订阅通知
    notify.send(SendNewSubscribeMsgArgs {
        touser: "openid".into(),
        template_id: "template_id".into(),
        page: Some("index?foo=bar".into()),
        data: serde_json::json!({
            "phrase3": {"value": "审核通过"},
            "name1": {"value": "订阅"},
        }),
        miniprogram_state: "formal".into(),
        lang: "zh_CN".into(),
    }).await?;

    Ok(())
}
```

### 群发消息

```rust
use wechat_mp::WechatMp;
use wechat_mp::mp_message::{MassMessage, Article, UploadNewsMsgArgs};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = WechatMp::new("your_app_id", "your_app_secret");
    let mass = MassMessage::new(client);

    // 上传图文消息素材
    let upload = mass.upload_news(UploadNewsMsgArgs {
        articles: vec![Article {
            title: "标题".into(),
            thumb_media_id: "media_id".into(),
            content: "<p>内容</p>".into(),
            author: Some("作者".into()),
            content_source_url: Some("https://example.com".into()),
            digest: None,
            show_cover_pic: Some(1),
            need_open_comment: Some(1),
            only_fans_can_comment: Some(0),
        }],
    }).await?;

    // 根据标签群发
    mass.send_all(serde_json::json!({
        "filter": {"is_to_all": true},
        "msgtype": "text",
        "text": {"content": "群发消息"},
        "clientmsgid": "unique_id"
    })).await?;

    // 查询发送状态
    let status = mass.get_status(wechat_mp::mp_message::MassMsgGetArgs {
        msg_id: "msg_id".into(),
    }).await?;

    Ok(())
}
```

### 获取自动回复规则

```rust
use wechat_mp::WechatMp;
use wechat_mp::mp_message::AutoReply;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = WechatMp::new("your_app_id", "your_app_secret");
    let autoreply = AutoReply::new(client);

    let info = autoreply.get_current_autoreply_info().await?;

    Ok(())
}
```

## API 列表

| 分类 | 方法 | 说明 |
|------|------|------|
| **Access Token** | `token()` | 获取接口调用凭据 |
| **模板消息** | `send_message()` | 发送模板消息 |
| | `template_subscribe()` | 发送一次性订阅消息 |
| | `get_industry()` | 获取行业信息 |
| | `set_industry()` | 设置所属行业 |
| | `get_all_templates()` | 获取已选用模板列表 |
| | `query_block_tmpl_msg()` | 查询拦截模板消息 |
| **订阅通知** | `send()` | 发送订阅通知 |
| | `add_template()` | 选用模板 |
| | `del_template()` | 删除模板 |
| | `get_templates()` | 获取已有模板列表 |
| | `get_template_keywords()` | 获取模板关键词 |
| | `get_category()` | 获取类目 |
| | `get_pub_template_titles()` | 获取类目下的公共模板 |
| **群发消息** | `send_all()` | 根据标签群发 |
| | `mass_send()` | 根据 OpenID 群发 |
| | `delete()` | 删除群发消息 |
| | `get_status()` | 查询群发状态 |
| | `get_speed()` / `set_speed()` | 群发速度控制 |
| | `upload_news()` | 上传图文消息素材 |
| | `preview()` | 预览消息 |
| **自动回复** | `get_current_autoreply_info()` | 获取自动回复规则 |

## 相关 Crates

- [wechat-core](https://crates.io/crates/wechat-core) - 核心功能库
- [wechat-minapp](https://crates.io/crates/wechat-minapp) - 小程序服务端 API

## 许可证

MIT
