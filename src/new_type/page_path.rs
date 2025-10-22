use crate::constants;
use crate::utils::{parse_query, parse_url};

use super::PagePathError;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PagePath(String);

impl PagePath {
    /// 创建新的页面路径，进行验证
    pub fn new(path: &str) -> Result<Self, PagePathError> {
        // 检查是否为空
        if path.trim().is_empty() {
            return Err(PagePathError::Empty);
        }

        // 检查长度
        if path.len() > 1024 {
            return Err(PagePathError::InvalidLength);
        }

        // 检查是否包含保留参数
        let url = parse_url(format!("{}/{}", constants::END_POINT_BASE_URL, path))?;
        let query_pairs = parse_query(url)?;
        if query_pairs
            .iter()
            .any(|item| item.0.contains("scancode_time"))
        {
            return Err(PagePathError::ReservedParameter);
        }
        Ok(PagePath(path.to_string()))
    }

    /// 获取内部路径引用
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// 获取内部路径
    pub fn into_inner(self) -> String {
        self.0
    }

    /// 获取路径长度
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// 检查路径是否为空（理论上不会为空，因为创建时已验证）
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl FromStr for PagePath {
    type Err = PagePathError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        PagePath::new(s)
    }
}

impl fmt::Display for PagePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for PagePath {
    type Error = PagePathError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        PagePath::new(&value)
    }
}

impl TryFrom<&str> for PagePath {
    type Error = PagePathError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        PagePath::new(value)
    }
}

// 为方便使用，实现 Deref
impl std::ops::Deref for PagePath {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// 实现 AsRef<str> 以便于转换为 &str
impl AsRef<str> for PagePath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
