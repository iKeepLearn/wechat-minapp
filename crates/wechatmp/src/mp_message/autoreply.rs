//! 自动回复模块
//!
//! 提供获取公众号自动回复规则的功能
//! [官方文档 - 获取自动回复规则](https://developers.weixin.qq.com/doc/service/api/notify/autoreplies/api_getcurrentautoreplyinfo.html)

use crate::constants;
use serde::{Deserialize, Serialize};
use tracing::debug;
use wechat_core::utils::{RequestBuilder, ResponseExt};
use wechat_core::Result;
use http::Method;

use crate::WechatMp;

/// 自动回复客户端
#[derive(Debug, Clone)]
pub struct AutoReply {
    pub client: WechatMp,
}

impl AutoReply {
    pub fn new(client: WechatMp) -> Self {
        AutoReply { client }
    }

    /// 获取自动回复规则
    ///
    /// 获取公众号当前使用的自动回复规则，包括关注后自动回复、消息自动回复、关键词自动回复
    pub async fn get_current_autoreply_info(&self) -> Result<AutoReplyInfoResponse> {
        debug!("get current autoreply info");

        let query = serde_json::json!({
            "access_token": self.client.token().await?
        });

        let request = RequestBuilder::new(constants::MP_GET_AUTOREPLY_END_POINT)
            .method(Method::GET)
            .query(query)
            .build()?;

        let client = &self.client.core.client;
        let response = client.execute(request).await?;

        response.to_json::<AutoReplyInfoResponse>()
    }
}

// ========================================
// 响应类型定义
// ========================================

/// 自动回复内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoReplyContent {
    #[serde(rename = "type")]
    pub type_: String,
    pub content: String,
}

/// 图文消息条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsArticle {
    pub title: String,
    pub digest: String,
    pub author: String,
    pub show_cover: i32,
    pub cover_url: String,
    pub content_url: String,
    pub source_url: String,
}

/// 图文消息信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsInfo {
    pub list: Vec<NewsArticle>,
}

/// 关键词信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeywordInfo {
    #[serde(rename = "type")]
    pub type_: String,
    pub content: String,
    pub match_mode: String,
}

/// 回复信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyInfo {
    #[serde(rename = "type")]
    pub type_: String,
    pub content: String,
    #[serde(default)]
    pub news_info: Option<NewsInfo>,
}

/// 关键词自动回复规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeywordRule {
    pub rule_name: String,
    pub create_time: i64,
    pub reply_mode: String,
    pub keyword_list_info: Vec<KeywordInfo>,
    pub reply_list_info: Vec<ReplyInfo>,
}

/// 关键词自动回复信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeywordAutoReplyInfo {
    pub list: Vec<KeywordRule>,
}

/// 获取自动回复规则响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoReplyInfoResponse {
    pub is_add_friend_reply_open: i32,
    pub is_autoreply_open: i32,
    pub add_friend_autoreply_info: AutoReplyContent,
    pub message_default_autoreply_info: AutoReplyContent,
    pub keyword_autoreply_info: KeywordAutoReplyInfo,
}
