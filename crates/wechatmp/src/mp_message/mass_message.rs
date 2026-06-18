//! 群发消息模块
//!
//! 提供群发消息相关的 API，包括根据标签/OpenID群发、删除、查询、速度控制等
//! [官方文档 - 群发消息](https://developers.weixin.qq.com/doc/service/api/notify/message)

use crate::constants;
use serde::{Deserialize, Serialize};
use tracing::debug;
use wechat_core::utils::{RequestBuilder, ResponseExt};
use wechat_core::Result;
use http::Method;

use crate::WechatMp;

/// 群发消息客户端
#[derive(Debug, Clone)]
pub struct MassMessage {
    pub client: WechatMp,
}

impl MassMessage {
    pub fn new(client: WechatMp) -> Self {
        MassMessage { client }
    }
}

// ========================================
// 通用响应
// ========================================

/// 微信 API 基础响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MpResponse {
    pub errcode: i32,
    pub errmsg: String,
}

// ========================================
// 群发响应
// ========================================

/// 群发消息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MassSendResponse {
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub msg_id: Option<i64>,
    pub msg_data_id: Option<i64>,
}

// ========================================
// 查询群发状态响应
// ========================================

/// 查询群发消息发送状态响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MassMsgGetResponse {
    pub msg_id: String,
    pub msg_status: String,
}

// ========================================
// 群发速度响应
// ========================================

/// 获取群发速度响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSpeedResponse {
    pub speed: i32,
    pub realspeed: i32,
}

// ========================================
// 设置群发速度请求
// ========================================

/// 设置群发速度请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetSpeedArgs {
    pub speed: i32,
}

// ========================================
// 删除群发请求
// ========================================

/// 删除群发消息请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMassMsgArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_id: Option<i64>,
    pub article_idx: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

// ========================================
// 查询群发状态请求
// ========================================

/// 查询群发消息发送状态请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MassMsgGetArgs {
    pub msg_id: String,
}

// ========================================
// 上传图文消息素材
// ========================================

/// 图文消息文章
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub title: String,
    pub thumb_media_id: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_source_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_cover_pic: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_open_comment: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_fans_can_comment: Option<i32>,
}

/// 上传图文消息素材请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadNewsMsgArgs {
    pub articles: Vec<Article>,
}

/// 上传图文消息素材响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadNewsMsgResponse {
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub media_id: Option<String>,
    pub created_at: Option<String>,
}

// ========================================
// 预览消息
// ========================================

/// 预览消息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewResponse {
    pub msg_id: Option<i64>,
}

impl MassMessage {
    /// 根据标签群发消息
    ///
    /// 发送消息体为 `serde_json::Value`，根据 msgtype 不同结构不同：
    ///
    /// ```json
    /// {
    ///   "filter": {"is_to_all": true, "tag_id": "2"},
    ///   "msgtype": "text",
    ///   "text": {"content": "CONTENT"},
    ///   "clientmsgid": "xxx"
    /// }
    /// ```
    ///
    /// 支持的 msgtype: mpnews, text, voice, image, mpvideo, wxcard
    pub async fn send_all(&self, body: serde_json::Value) -> Result<MassSendResponse> {
        debug!("mass send all body {:?}", &body);

        let query = serde_json::json!({
            "access_token": self.client.token().await?
        });

        let request = RequestBuilder::new(constants::MP_MASS_SEND_ALL_END_POINT)
            .query(query)
            .body(body)
            .build()?;

        let client = &self.client.core.client;
        let response = client.execute(request).await?;

        response.to_json::<MassSendResponse>()
    }

    /// 根据 OpenID 列表群发消息
    ///
    /// 发送体为 `serde_json::Value`，结构与 `send_all` 类似但使用 `touser` 字段
    pub async fn mass_send(&self, body: serde_json::Value) -> Result<MassSendResponse> {
        debug!("mass send body {:?}", &body);

        let query = serde_json::json!({
            "access_token": self.client.token().await?
        });

        let request = RequestBuilder::new(constants::MP_MASS_SEND_END_POINT)
            .query(query)
            .body(body)
            .build()?;

        let client = &self.client.core.client;
        let response = client.execute(request).await?;

        response.to_json::<MassSendResponse>()
    }

    /// 删除群发消息
    ///
    /// 群发之后，随时可以通过该接口删除群发
    pub async fn delete(&self, args: DeleteMassMsgArgs) -> Result<MpResponse> {
        debug!("delete mass msg args {:?}", &args);

        let query = serde_json::json!({
            "access_token": self.client.token().await?
        });

        let body = serde_json::to_value(args)?;

        let request = RequestBuilder::new(constants::MP_MASS_DELETE_END_POINT)
            .query(query)
            .body(body)
            .build()?;

        let client = &self.client.core.client;
        let response = client.execute(request).await?;

        response.to_json::<MpResponse>()
    }

    /// 查询群发消息发送状态
    pub async fn get_status(&self, args: MassMsgGetArgs) -> Result<MassMsgGetResponse> {
        debug!("get mass msg status args {:?}", &args);

        let query = serde_json::json!({
            "access_token": self.client.token().await?
        });

        let body = serde_json::to_value(args)?;

        let request = RequestBuilder::new(constants::MP_MASS_GET_END_POINT)
            .query(query)
            .body(body)
            .build()?;

        let client = &self.client.core.client;
        let response = client.execute(request).await?;

        response.to_json::<MassMsgGetResponse>()
    }

    /// 获取群发速度
    pub async fn get_speed(&self) -> Result<GetSpeedResponse> {
        debug!("get mass speed");

        let query = serde_json::json!({
            "access_token": self.client.token().await?
        });

        let request = RequestBuilder::new(constants::MP_MASS_SPEED_GET_END_POINT)
            .method(Method::GET)
            .query(query)
            .build()?;

        let client = &self.client.core.client;
        let response = client.execute(request).await?;

        response.to_json::<GetSpeedResponse>()
    }

    /// 设置群发速度
    ///
    /// speed: 0=80w/分钟, 1=60w/分钟, 2=45w/分钟, 3=30w/分钟, 4=10w/分钟
    pub async fn set_speed(&self, args: SetSpeedArgs) -> Result<MpResponse> {
        debug!("set mass speed args {:?}", &args);

        let query = serde_json::json!({
            "access_token": self.client.token().await?
        });

        let body = serde_json::to_value(args)?;

        let request = RequestBuilder::new(constants::MP_MASS_SPEED_SET_END_POINT)
            .query(query)
            .body(body)
            .build()?;

        let client = &self.client.core.client;
        let response = client.execute(request).await?;

        response.to_json::<MpResponse>()
    }

    /// 上传图文消息素材
    ///
    /// 用于群发图文消息时上传素材
    pub async fn upload_news(&self, args: UploadNewsMsgArgs) -> Result<UploadNewsMsgResponse> {
        debug!("upload news msg args {:?}", &args);

        let query = serde_json::json!({
            "access_token": self.client.token().await?
        });

        let body = serde_json::to_value(args)?;

        let request = RequestBuilder::new(constants::MP_MEDIA_UPLOAD_NEWS_END_POINT)
            .query(query)
            .body(body)
            .build()?;

        let client = &self.client.core.client;
        let response = client.execute(request).await?;

        response.to_json::<UploadNewsMsgResponse>()
    }

    /// 预览消息
    ///
    /// 发送消息给指定用户，在手机端查看消息的样式和排版
    ///
    /// `body` 为消息体，根据 msgtype 不同结构不同：
    ///
    /// ```json
    /// {"touser": "OPENID", "msgtype": "text", "text": {"content": "hello"}}
    /// ```
    ///
    /// 支持 msgtype: mpnews, text, voice, music, image, mpvideo, wxcard
    pub async fn preview(&self, body: serde_json::Value) -> Result<PreviewResponse> {
        debug!("preview body {:?}", &body);

        let query = serde_json::json!({
            "access_token": self.client.token().await?
        });

        let request = RequestBuilder::new(constants::MP_MASS_PREVIEW_END_POINT)
            .query(query)
            .body(body)
            .build()?;

        let client = &self.client.core.client;
        let response = client.execute(request).await?;

        response.to_json::<PreviewResponse>()
    }
}
