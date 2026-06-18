//! 微信服务号 API 端点常量模块

/// wechatmp crate 当前版本
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// 发送服务号模板消息的 API 端点
pub const MP_MESSAGE_SEND_END_POINT: &str =
    "https://api.weixin.qq.com/cgi-bin/message/template/send";

// ========================================
// 模板消息 (Template Message)
// ========================================

/// 发送一次性订阅消息
pub const MP_MESSAGE_TEMPLATE_SUBSCRIBE_END_POINT: &str =
    "https://api.weixin.qq.com/cgi-bin/message/template/subscribe";

/// 获取行业信息
pub const MP_TEMPLATE_GET_INDUSTRY_END_POINT: &str =
    "https://api.weixin.qq.com/cgi-bin/template/get_industry";

/// 设置所属行业
pub const MP_TEMPLATE_SET_INDUSTRY_END_POINT: &str =
    "https://api.weixin.qq.com/cgi-bin/template/api_set_industry";

/// 获取已选用模板列表
pub const MP_TEMPLATE_GET_ALL_END_POINT: &str =
    "https://api.weixin.qq.com/cgi-bin/template/get_all_template";

/// 查询拦截模板消息
pub const MP_TEMPLATE_QUERY_BLOCK_END_POINT: &str =
    "https://api.weixin.qq.com/cgi-bin/template/query_block_tmpl_msg";

// ========================================
// 订阅通知 (Subscribe Notify)
// ========================================

/// 发送订阅通知
pub const MP_SUBSCRIBE_SEND_END_POINT: &str =
    "https://api.weixin.qq.com/cgi-bin/message/subscribe/bizsend";

/// 删除模板
pub const MP_SUBSCRIBE_TEMPLATE_DEL_END_POINT: &str =
    "https://api.weixin.qq.com/cgi-bin/wxopen/template/del";

/// 获取已有模板列表
pub const MP_SUBSCRIBE_TEMPLATE_LIST_END_POINT: &str =
    "https://api.weixin.qq.com/cgi-bin/wxopen/template/list";

/// 获取模板关键词
pub const MP_SUBSCRIBE_TEMPLATE_KEYWORDS_END_POINT: &str =
    "https://api.weixin.qq.com/cgi-bin/wxopen/template/library/get";

/// 获取类目
pub const MP_SUBSCRIBE_CATEGORY_LIST_END_POINT: &str =
    "https://api.weixin.qq.com/cgi-bin/wxopen/template/category/list";

/// 获取类目下的公共模板
pub const MP_SUBSCRIBE_TEMPLATE_TITLES_END_POINT: &str =
    "https://api.weixin.qq.com/cgi-bin/wxopen/template/library/list";

/// 选用模板
pub const MP_SUBSCRIBE_TEMPLATE_ADD_END_POINT: &str =
    "https://api.weixin.qq.com/cgi-bin/wxopen/template/add";

// ========================================
// 群发消息 (Mass Send)
// ========================================

/// 根据标签群发消息
pub const MP_MASS_SEND_ALL_END_POINT: &str =
    "https://api.weixin.qq.com/cgi-bin/message/mass/sendall";

/// 根据 OpenID 群发消息
pub const MP_MASS_SEND_END_POINT: &str =
    "https://api.weixin.qq.com/cgi-bin/message/mass/send";

/// 删除群发消息
pub const MP_MASS_DELETE_END_POINT: &str =
    "https://api.weixin.qq.com/cgi-bin/message/mass/delete";

/// 查询群发消息发送状态
pub const MP_MASS_GET_END_POINT: &str =
    "https://api.weixin.qq.com/cgi-bin/message/mass/get";

/// 获取群发速度
pub const MP_MASS_SPEED_GET_END_POINT: &str =
    "https://api.weixin.qq.com/cgi-bin/message/mass/speed/get";

/// 设置群发速度
pub const MP_MASS_SPEED_SET_END_POINT: &str =
    "https://api.weixin.qq.com/cgi-bin/message/mass/speed/set";

/// 上传图文消息素材
pub const MP_MEDIA_UPLOAD_NEWS_END_POINT: &str =
    "https://api.weixin.qq.com/cgi-bin/media/uploadnews";

/// 上传发表内容中的图片
pub const MP_MEDIA_UPLOAD_IMG_END_POINT: &str =
    "https://api.weixin.qq.com/cgi-bin/media/uploadimg";

/// 预览消息
pub const MP_MASS_PREVIEW_END_POINT: &str =
    "https://api.weixin.qq.com/cgi-bin/message/mass/preview";

// ========================================
// 自动回复
// ========================================

/// 获取自动回复规则
pub const MP_GET_AUTOREPLY_END_POINT: &str =
    "https://api.weixin.qq.com/cgi-bin/get_current_autoreply_info";
