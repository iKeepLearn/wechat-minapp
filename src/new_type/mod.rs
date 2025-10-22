//! 用于传参验证
//!
mod non_query_page_path;
mod page_path;
mod scene;

use crate::error::Error;
pub use non_query_page_path::NonQueryPagePath;
pub use page_path::PagePath;
pub use scene::{SceneString, ValidationSceneError};
use std::fmt;

/// 页面路径验证错误类型
#[derive(Debug, Clone, PartialEq)]
pub enum PagePathError {
    /// 路径以斜杠开头
    StartsWithSlash,
    /// 路径包含参数（应该放在scene字段）
    ContainsParams,
    /// 路径格式不正确
    InvalidFormat,
    /// 页面路径最大长度 1024 个字符
    InvalidLength,
    /// 页面路径包含系统保留参数 'scancode_time'
    ReservedParameter,
    /// 页面路径不能为空
    Empty,
}

impl std::error::Error for PagePathError {}

impl From<url::ParseError> for PagePathError {
    fn from(_value: url::ParseError) -> Self {
        PagePathError::InvalidFormat
    }
}

impl From<Error> for PagePathError {
    fn from(_value: Error) -> Self {
        PagePathError::InvalidFormat
    }
}
impl fmt::Display for PagePathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PagePathError::StartsWithSlash => write!(f, "页面路径不能以斜杠开头"),
            PagePathError::ContainsParams => {
                write!(f, "页面路径不能携带参数，参数请放在scene字段里")
            }
            PagePathError::InvalidFormat => write!(f, "页面路径格式不正确"),
            PagePathError::InvalidLength => write!(f, "页面路径最大长度 1024 个字符"),
            PagePathError::Empty => write!(f, "页面路径不能为空"),
            PagePathError::ReservedParameter => {
                write!(f, "页面路径包含系统保留参数 'scancode_time'")
            }
        }
    }
}
