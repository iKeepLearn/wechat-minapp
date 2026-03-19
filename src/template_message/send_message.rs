//! 微信小程序模板消息发送模块
//!
//! 提供发送模板消息和订阅消息的功能，用于向用户发送服务通知。
//! [官方文档](https://developers.weixin.qq.com/miniprogram/dev/server/API/mp-message-management/subscribe-message/api_sendmessage.html)
//!
//! ## 示例
//!
//! ```no_run
//! use wechat_minapp::client::WechatMinapp;
//! use wechat_minapp::template_message::{TemplateMessage, SendMessageArgs};
//! use serde_json::json;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // 初始化客户端
//!     let app_id = "your_app_id";
//!     let secret = "your_app_secret";
//!     let client = WechatMinapp::new(app_id, secret);
//!     let message = TemplateMessage::new(client);
//!
//!     // 构建模板消息数据
//!     let data = json!({
//!         "thing1": {"value": "订单支付成功"},
//!         "amount2": {"value": "¥99.00"},
//!         "date3": {"value": "2024-01-01 12:00:00"}
//!     });
//!
//!     let args = TemplateMessage::builder()
//!         .touser("openid")
//!         .template_id("template_id")
//!         .page("pages/index/index")
//!         .data(data)
//!         .miniprogram_state("formal")
//!         .lang("zh_CN")
//!         .build()?;
//!
//!     // 发送模板消息
//!     let result = message.send_message(args).await?;
//!     
//!     Ok(())
//! }
//! ```

use super::TemplateMessage;
use crate::utils::{RequestBuilder, ResponseExt};
use crate::{Result, constants, error::Error};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::debug;

/// 订阅消息
///
/// 用于发送一次性订阅消息的结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageArgs {
    pub touser: String,                    // 接收者openid
    pub template_id: String,               // 模板ID
    pub page: Option<String>,              // 点击跳转页面
    pub data: serde_json::Value,           // 模板数据
    pub miniprogram_state: Option<String>, // 小程序状态
    pub lang: Option<String>,              // 语言
}

/// 订阅消息发送响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageResponse {
    pub msgid: Option<String>,  // 消息ID
    pub errcode: Option<i32>,   // 错误码
    pub errmsg: Option<String>, // 错误信息
}

/// 订阅消息参数构建器
#[derive(Debug, Default)]
pub struct SendMessageArgsBuilder {
    touser: Option<String>,
    template_id: Option<String>,
    page: Option<String>,
    data: Option<serde_json::Value>,
    miniprogram_state: Option<String>,
    lang: Option<String>,
}

impl SendMessageArgs {
    /// 创建订阅消息构建器
    pub fn builder() -> SendMessageArgsBuilder {
        SendMessageArgsBuilder::new()
    }

    /// 获取接收者openid
    pub fn touser(&self) -> &str {
        &self.touser
    }

    /// 获取模板ID
    pub fn template_id(&self) -> &str {
        &self.template_id
    }

    /// 获取跳转页面
    pub fn page(&self) -> Option<&String> {
        self.page.as_ref()
    }

    /// 获取模板数据
    pub fn data(&self) -> &Value {
        &self.data
    }

    /// 获取小程序状态
    pub fn miniprogram_state(&self) -> Option<&String> {
        self.miniprogram_state.as_ref()
    }

    /// 获取语言
    pub fn lang(&self) -> Option<&String> {
        self.lang.as_ref()
    }
}

impl SendMessageArgsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置接收者openid
    pub fn touser(mut self, touser: impl Into<String>) -> Self {
        self.touser = Some(touser.into());
        self
    }

    /// 设置模板ID
    pub fn template_id(mut self, template_id: impl Into<String>) -> Self {
        self.template_id = Some(template_id.into());
        self
    }

    /// 设置跳转页面
    pub fn page(mut self, page: impl Into<String>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// 设置模板数据
    pub fn data(mut self, data: impl Into<serde_json::Value>) -> Self {
        self.data = Some(data.into());
        self
    }

    /// 设置小程序状态
    ///
    /// 可选值：formal（正式版），trial（体验版），developer（开发版）
    pub fn miniprogram_state(mut self, state: impl Into<String>) -> Self {
        self.miniprogram_state = Some(state.into());
        self
    }

    /// 设置语言
    ///
    /// 可选值：zh_CN（简体中文），en_US（英文），zh_HK（繁体中文），zh_TW（繁体中文）
    pub fn lang(mut self, lang: impl Into<String>) -> Self {
        self.lang = Some(lang.into());
        self
    }

    /// 构建订阅消息参数
    pub fn build(self) -> Result<SendMessageArgs> {
        let touser = self
            .touser
            .ok_or_else(|| Error::InvalidParameter("接收者openid不能为空".to_string()))?;

        let template_id = self
            .template_id
            .ok_or_else(|| Error::InvalidParameter("模板ID不能为空".to_string()))?;

        let data = self
            .data
            .ok_or_else(|| Error::InvalidParameter("模板数据不能为空".to_string()))?;

        // 验证数据格式
        Self::validate_data(&data)?;

        Ok(SendMessageArgs {
            touser,
            template_id,
            page: self.page,
            data,
            miniprogram_state: self.miniprogram_state,
            lang: self.lang,
        })
    }

    /// 验证模板数据格式
    fn validate_data(data: &Value) -> Result<()> {
        if let Value::Object(map) = data {
            for (key, value) in map {
                if key.len() > 20 {
                    return Err(Error::InvalidParameter(format!(
                        "字段名'{}'长度不能超过20个字符",
                        key
                    )));
                }

                if let Value::Object(item) = value {
                    if let Some(val) = item.get("value") {
                        if let Value::String(s) = val
                            && s.len() > 50
                        {
                            return Err(Error::InvalidParameter(format!(
                                "字段'{}'的值长度不能超过50个字符",
                                key
                            )));
                        }
                    } else {
                        return Err(Error::InvalidParameter(format!(
                            "字段'{}'缺少value属性",
                            key
                        )));
                    }
                } else {
                    return Err(Error::InvalidParameter(format!(
                        "字段'{}'格式不正确，应为{{value: string}}",
                        key
                    )));
                }
            }
            Ok(())
        } else {
            Err(Error::InvalidParameter(
                "模板数据必须是对象类型".to_string(),
            ))
        }
    }
}

impl TemplateMessage {
    /// 发送模板消息
    ///
    /// 调用微信小程序模板消息发送接口
    ///
    /// # 参数
    ///
    /// - `args`: 模板消息参数
    ///
    /// # 返回
    ///
    /// 成功返回 `Ok(SendMessageResponse)`，失败返回错误信息
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use wechat_minapp::client::WechatMinapp;
    /// use wechat_minapp::template_message::{TemplateMessage, SendMessageArgs};
    /// use serde_json::json;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = WechatMinapp::new("app_id", "secret");
    ///     let message = TemplateMessage::new(client);
    ///
    ///     let data = json!({
    ///         "thing1": {"value": "订单支付成功"},
    ///         "amount2": {"value": "¥99.00"},
    ///     });
    ///
    ///     let args = SendMessageArgs::builder()
    ///         .touser("openid")
    ///         .template_id("template_id")
    ///         .data(data)
    ///         .build()?;
    ///     let result = message.send_message(args).await?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn send_message(&self, args: SendMessageArgs) -> Result<SendMessageResponse> {
        debug!("send template message args {:?}", &args);

        let query = serde_json::json!({
            "access_token": self.client.token().await?
        });

        let body = serde_json::to_value(args)?;

        let request = RequestBuilder::new(constants::TEMPLATE_MESSAGE_SEND_END_POINT)
            .query(query)
            .body(body)
            .build()?;

        let client = &self.client.client;
        let response = client.execute(request).await?;

        debug!("response: {:#?}", response);
        response.to_json::<SendMessageResponse>()
    }
}
