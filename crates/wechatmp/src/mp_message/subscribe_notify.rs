//! 订阅通知模块
//!
//! 提供订阅通知相关的 API，包括发送订阅通知、模板管理等功能
//! [官方文档 - 订阅通知](https://developers.weixin.qq.com/doc/service/api/notify/notify)

use crate::constants;
use serde::{Deserialize, Serialize};
use tracing::debug;
use wechat_core::utils::{RequestBuilder, ResponseExt};
use wechat_core::Result;
use http::Method;

use crate::WechatMp;

/// 订阅通知客户端
#[derive(Debug, Clone)]
pub struct SubscribeNotify {
    pub client: WechatMp,
}

impl SubscribeNotify {
    pub fn new(client: WechatMp) -> Self {
        SubscribeNotify { client }
    }
}

// ========================================
// 发送订阅通知
// ========================================

/// 发送订阅通知请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendNewSubscribeMsgArgs {
    pub touser: String,
    pub template_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    pub data: serde_json::Value,
    pub miniprogram_state: String,
    pub lang: String,
}

// ========================================
// 删除模板
// ========================================

/// 删除模板请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelTemplateArgs {
    #[serde(rename = "priTmplId")]
    pub pri_tmpl_id: String,
}

// ========================================
// 获取已有模板列表 (响应)
// ========================================

/// 关键词枚举值
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeywordEnumValue {
    #[serde(rename = "keywordCode")]
    pub keyword_code: String,
    #[serde(rename = "enumValueList")]
    pub enum_value_list: Vec<String>,
}

/// 模板条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateItem {
    #[serde(rename = "priTmplId")]
    pub pri_tmpl_id: String,
    pub title: String,
    pub content: String,
    pub example: String,
    #[serde(rename = "type")]
    pub type_: i32,
    #[serde(default)]
    #[serde(rename = "keywordEnumValueList")]
    pub keyword_enum_value_list: Vec<KeywordEnumValue>,
}

/// 获取已有模板列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTemplatesResponse {
    pub data: Vec<TemplateItem>,
}

// ========================================
// 获取模板关键词 (响应)
// ========================================

/// 关键词信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeywordInfo {
    pub kid: i32,
    pub name: String,
    pub example: String,
    pub rule: String,
}

/// 获取模板关键词响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTemplateKeywordsResponse {
    pub count: i32,
    pub data: Vec<KeywordInfo>,
}

// ========================================
// 获取类目 (响应)
// ========================================

/// 类目信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryInfo {
    pub id: i32,
    pub name: String,
}

/// 获取类目响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCategoryResponse {
    pub data: Vec<CategoryInfo>,
}

// ========================================
// 获取类目下的公共模板 (响应)
// ========================================

/// 公共模板标题信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubTemplateTitle {
    pub tid: i32,
    pub title: String,
    #[serde(rename = "type")]
    pub type_: i32,
    #[serde(rename = "categoryId")]
    pub category_id: i32,
}

/// 获取类目下的公共模板响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPubTemplateTitlesResponse {
    pub count: i32,
    pub data: Vec<PubTemplateTitle>,
}

// ========================================
// 选用模板
// ========================================

/// 选用模板请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddTemplateArgs {
    pub tid: String,
    #[serde(rename = "kidList")]
    pub kid_list: Vec<i32>,
    #[serde(rename = "sceneDesc")]
    pub scene_desc: String,
}

/// 选用模板响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddTemplateResponse {
    #[serde(rename = "priTmplId")]
    pub pri_tmpl_id: String,
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

impl SubscribeNotify {
    /// 发送订阅通知
    ///
    /// 发送订阅通知给用户
    pub async fn send(&self, args: SendNewSubscribeMsgArgs) -> Result<MpResponse> {
        debug!("send subscribe notify args {:?}", &args);

        let query = serde_json::json!({
            "access_token": self.client.token().await?
        });

        let body = serde_json::to_value(args)?;

        let request = RequestBuilder::new(constants::MP_SUBSCRIBE_SEND_END_POINT)
            .query(query)
            .body(body)
            .build()?;

        let client = &self.client.core.client;
        let response = client.execute(request).await?;

        response.to_json::<MpResponse>()
    }

    /// 删除模板
    ///
    /// 删除私有模板库中的模板
    pub async fn del_template(&self, args: DelTemplateArgs) -> Result<MpResponse> {
        debug!("del template args {:?}", &args);

        let query = serde_json::json!({
            "access_token": self.client.token().await?
        });

        let body = serde_json::to_value(args)?;

        let request = RequestBuilder::new(constants::MP_SUBSCRIBE_TEMPLATE_DEL_END_POINT)
            .query(query)
            .body(body)
            .build()?;

        let client = &self.client.core.client;
        let response = client.execute(request).await?;

        response.to_json::<MpResponse>()
    }

    /// 获取已有模板列表
    ///
    /// 获取当前帐号下的已有模板列表
    pub async fn get_templates(&self) -> Result<GetTemplatesResponse> {
        debug!("get templates");

        let query = serde_json::json!({
            "access_token": self.client.token().await?
        });

        let request = RequestBuilder::new(constants::MP_SUBSCRIBE_TEMPLATE_LIST_END_POINT)
            .method(Method::GET)
            .query(query)
            .build()?;

        let client = &self.client.core.client;
        let response = client.execute(request).await?;

        response.to_json::<GetTemplatesResponse>()
    }

    /// 获取模板关键词
    ///
    /// 获取模板标题下的关键词列表
    ///
    /// # 参数
    ///
    /// - `tid`: 模板标题 id
    pub async fn get_template_keywords(&self, tid: &str) -> Result<GetTemplateKeywordsResponse> {
        debug!("get template keywords tid: {}", tid);

        let query = serde_json::json!({
            "access_token": self.client.token().await?,
            "tid": tid
        });

        let request = RequestBuilder::new(constants::MP_SUBSCRIBE_TEMPLATE_KEYWORDS_END_POINT)
            .method(Method::GET)
            .query(query)
            .build()?;

        let client = &self.client.core.client;
        let response = client.execute(request).await?;

        response.to_json::<GetTemplateKeywordsResponse>()
    }

    /// 获取类目
    ///
    /// 获取小程序、公众号所属类目用于查询公共模板
    pub async fn get_category(&self) -> Result<GetCategoryResponse> {
        debug!("get category");

        let query = serde_json::json!({
            "access_token": self.client.token().await?
        });

        let request = RequestBuilder::new(constants::MP_SUBSCRIBE_CATEGORY_LIST_END_POINT)
            .method(Method::GET)
            .query(query)
            .build()?;

        let client = &self.client.core.client;
        let response = client.execute(request).await?;

        response.to_json::<GetCategoryResponse>()
    }

    /// 获取类目下的公共模板
    ///
    /// 获取帐号所属类目下的公共模板，可从中选用模板使用
    ///
    /// # 参数
    ///
    /// - `ids`: 类目 id，多个用逗号隔开
    /// - `start`: 分页起始位置
    /// - `limit`: 拉取条数，最大 30
    pub async fn get_pub_template_titles(
        &self,
        ids: &str,
        start: i32,
        limit: i32,
    ) -> Result<GetPubTemplateTitlesResponse> {
        debug!("get pub template titles ids: {}, start: {}, limit: {}", ids, start, limit);

        let query = serde_json::json!({
            "access_token": self.client.token().await?,
            "ids": ids,
            "start": start,
            "limit": limit
        });

        let request = RequestBuilder::new(constants::MP_SUBSCRIBE_TEMPLATE_TITLES_END_POINT)
            .method(Method::GET)
            .query(query)
            .build()?;

        let client = &self.client.core.client;
        let response = client.execute(request).await?;

        response.to_json::<GetPubTemplateTitlesResponse>()
    }

    /// 选用模板
    ///
    /// 从公共模板库中选用模板到私有模板库
    pub async fn add_template(&self, args: AddTemplateArgs) -> Result<AddTemplateResponse> {
        debug!("add template args {:?}", &args);

        let query = serde_json::json!({
            "access_token": self.client.token().await?
        });

        let body = serde_json::to_value(args)?;

        let request = RequestBuilder::new(constants::MP_SUBSCRIBE_TEMPLATE_ADD_END_POINT)
            .query(query)
            .body(body)
            .build()?;

        let client = &self.client.core.client;
        let response = client.execute(request).await?;

        response.to_json::<AddTemplateResponse>()
    }
}
