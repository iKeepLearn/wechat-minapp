//! 模板消息扩展功能模块
//!
//! 提供行业管理、一次性订阅消息、模板查询等扩展功能
//! [官方文档 - 模板消息](https://developers.weixin.qq.com/doc/service/api/notify/template)

use super::TemplateMessage;
use crate::constants;
use serde::{Deserialize, Serialize};
use tracing::debug;
use wechat_core::utils::{RequestBuilder, ResponseExt};
use wechat_core::Result;
use http::Method;

// ========================================
// 获取行业信息
// ========================================

/// 行业信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryInfo {
    pub first_class: String,
    pub second_class: String,
}

/// 获取行业信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetIndustryResponse {
    pub primary_industry: IndustryInfo,
    pub secondary_industry: IndustryInfo,
}

// ========================================
// 设置所属行业
// ========================================

/// 设置所属行业请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetIndustryArgs {
    pub industry_id1: String,
    pub industry_id2: String,
}

// ========================================
// 获取已选用模板列表
// ========================================

/// 模板信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateInfo {
    pub template_id: String,
    pub title: String,
    pub primary_industry: String,
    pub deputy_industry: String,
    pub content: String,
    pub example: String,
}

/// 获取已选用模板列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAllTemplatesResponse {
    pub template_list: Vec<TemplateInfo>,
}

// ========================================
// 查询拦截模板消息
// ========================================

/// 查询拦截模板消息请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryBlockTmplMsgArgs {
    pub tmpl_msg_id: String,
    pub largest_id: i64,
    pub limit: i32,
}

/// 拦截消息信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockMsgInfo {
    pub id: String,
    pub tmpl_msg_id: String,
    pub title: String,
    pub content: String,
    pub send_timestamp: i64,
    pub openid: String,
}

/// 查询拦截模板消息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryBlockTmplMsgResponse {
    pub msginfo: BlockMsgInfo,
}

// ========================================
// 发送一次性订阅消息
// ========================================

/// 一次性订阅消息中的小程序配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeMiniprogram {
    pub appid: String,
    pub pagepath: String,
}

/// 一次性订阅消息请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateSubscribeArgs {
    pub touser: String,
    pub template_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miniprogram: Option<SubscribeMiniprogram>,
    pub scene: String,
    pub title: String,
    pub data: serde_json::Value,
}

// ========================================
// 通用响应
// ========================================

/// 微信 API 基础响应，用于只有 errcode/errmsg 的接口
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MpResponse {
    pub errcode: i32,
    pub errmsg: String,
}

impl TemplateMessage {
    /// 获取行业信息
    ///
    /// 获取账号设置的行业信息
    pub async fn get_industry(&self) -> Result<GetIndustryResponse> {
        debug!("get industry");

        let query = serde_json::json!({
            "access_token": self.client.token().await?
        });

        let request = RequestBuilder::new(constants::MP_TEMPLATE_GET_INDUSTRY_END_POINT)
            .method(Method::GET)
            .query(query)
            .build()?;

        let client = &self.client.core.client;
        let response = client.execute(request).await?;

        response.to_json::<GetIndustryResponse>()
    }

    /// 设置所属行业
    ///
    /// 修改账号所属行业，每月可修改1次
    pub async fn set_industry(&self, args: SetIndustryArgs) -> Result<MpResponse> {
        debug!("set industry args {:?}", &args);

        let query = serde_json::json!({
            "access_token": self.client.token().await?
        });

        let body = serde_json::to_value(args)?;

        let request = RequestBuilder::new(constants::MP_TEMPLATE_SET_INDUSTRY_END_POINT)
            .query(query)
            .body(body)
            .build()?;

        let client = &self.client.core.client;
        let response = client.execute(request).await?;

        response.to_json::<MpResponse>()
    }

    /// 获取已选用模板列表
    ///
    /// 获取已添加至账号下的所有模板列表
    pub async fn get_all_templates(&self) -> Result<GetAllTemplatesResponse> {
        debug!("get all templates");

        let query = serde_json::json!({
            "access_token": self.client.token().await?
        });

        let request = RequestBuilder::new(constants::MP_TEMPLATE_GET_ALL_END_POINT)
            .method(Method::GET)
            .query(query)
            .build()?;

        let client = &self.client.core.client;
        let response = client.execute(request).await?;

        response.to_json::<GetAllTemplatesResponse>()
    }

    /// 发送一次性订阅消息
    ///
    /// 推送订阅模板消息给授权微信用户
    pub async fn template_subscribe(&self, args: TemplateSubscribeArgs) -> Result<MpResponse> {
        debug!("template subscribe args {:?}", &args);

        let query = serde_json::json!({
            "access_token": self.client.token().await?
        });

        let body = serde_json::to_value(args)?;

        let request = RequestBuilder::new(constants::MP_MESSAGE_TEMPLATE_SUBSCRIBE_END_POINT)
            .query(query)
            .body(body)
            .build()?;

        let client = &self.client.core.client;
        let response = client.execute(request).await?;

        response.to_json::<MpResponse>()
    }

    /// 查询拦截模板消息
    ///
    /// 查询被微信拦截的模板消息记录
    pub async fn query_block_tmpl_msg(
        &self,
        args: QueryBlockTmplMsgArgs,
    ) -> Result<QueryBlockTmplMsgResponse> {
        debug!("query block tmpl msg args {:?}", &args);

        let query = serde_json::json!({
            "access_token": self.client.token().await?
        });

        let body = serde_json::to_value(args)?;

        let request = RequestBuilder::new(constants::MP_TEMPLATE_QUERY_BLOCK_END_POINT)
            .query(query)
            .body(body)
            .build()?;

        let client = &self.client.core.client;
        let response = client.execute(request).await?;

        response.to_json::<QueryBlockTmplMsgResponse>()
    }
}
