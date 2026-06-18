//! 微信服务号模板消息模块
//!
//! ## 功能
//! - [`send_message`] 发送模板消息
//! - [`template`] 模板消息扩展 API（行业管理、一次性订阅、拦截查询）
//! - [`subscribe_notify`] 订阅通知 API（发送、模板管理）
//! - [`mass_message`] 群发消息 API（标签/OpenID群发、删除、状态查询、速度控制、素材上传、预览）
//! - [`autoreply`] 自动回复规则查询
//!
pub mod send_message;
pub mod template;
pub mod subscribe_notify;
pub mod mass_message;
pub mod autoreply;

pub use send_message::SendMessageArgs;
pub use template::{
    BlockMsgInfo, GetAllTemplatesResponse, GetIndustryResponse, IndustryInfo, MpResponse,
    QueryBlockTmplMsgArgs, QueryBlockTmplMsgResponse, SetIndustryArgs, SubscribeMiniprogram,
    TemplateInfo, TemplateSubscribeArgs,
};
pub use subscribe_notify::{
    AddTemplateArgs, AddTemplateResponse, CategoryInfo, DelTemplateArgs, GetCategoryResponse,
    GetPubTemplateTitlesResponse, GetTemplateKeywordsResponse, GetTemplatesResponse,
    KeywordEnumValue, KeywordInfo, PubTemplateTitle, SendNewSubscribeMsgArgs, SubscribeNotify,
    TemplateItem,
};
pub use mass_message::{
    Article, DeleteMassMsgArgs, GetSpeedResponse, MassMessage, MassMsgGetArgs, MassMsgGetResponse,
    MassSendResponse, PreviewResponse, SetSpeedArgs, UploadNewsMsgArgs, UploadNewsMsgResponse,
};
pub use autoreply::{
    AutoReply, AutoReplyContent, AutoReplyInfoResponse, KeywordAutoReplyInfo, KeywordRule,
    NewsArticle, NewsInfo, ReplyInfo,
};

use crate::WechatMp;

/// 模板消息客户端
pub struct TemplateMessage {
    pub client: WechatMp,
}

impl TemplateMessage {
    pub fn new(client: WechatMp) -> Self {
        TemplateMessage { client }
    }
}
