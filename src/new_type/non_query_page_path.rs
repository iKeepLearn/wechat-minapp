use super::PagePathError;
use crate::constants;
use crate::utils::parse_url;
use std::fmt;
use std::str::FromStr;

/// 页面路径 newtype
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NonQueryPagePath(String);

impl NonQueryPagePath {
    /// 创建一个新的页面路径，进行验证
    pub fn new(path: impl Into<String>) -> Result<Self, PagePathError> {
        let path = path.into();
        Self::validate(&path)?;
        Ok(NonQueryPagePath(path))
    }

    /// 验证页面路径的规则
    fn validate(path: &str) -> Result<(), PagePathError> {
        // if path.len() > 1024 {
        //     return Err(PagePathError::InvalidLength);
        // }
        // 检查是否以斜杠开头
        if path.starts_with('/') {
            return Err(PagePathError::StartsWithSlash);
        }

        let url = parse_url(format!("{}/{}", constants::END_POINT_BASE_URL, path).as_str())?;
        if url.query_pairs().count() > 0 {
            return Err(PagePathError::ContainsParams);
        }
        if path.split("/").any(|s| {
            let trim_len = s.trim().len();
            s.is_empty() || trim_len != s.len()
        }) {
            return Err(PagePathError::InvalidFormat);
        }

        Ok(())
    }

    /// 获取内部字符串引用
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// 获取内部字符串
    pub fn into_inner(self) -> String {
        self.0
    }
}

// 实现 FromStr 以便于解析
impl FromStr for NonQueryPagePath {
    type Err = PagePathError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        NonQueryPagePath::new(s)
    }
}

// 实现 From<NonQueryPagePath> for String
impl From<NonQueryPagePath> for String {
    fn from(page_path: NonQueryPagePath) -> Self {
        page_path.0
    }
}

// 实现 AsRef<str> 以便于作为字符串引用使用
impl AsRef<str> for NonQueryPagePath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

// 实现 Display
impl fmt::Display for NonQueryPagePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// 为 &str 实现快速创建（可能失败）
impl<'a> TryFrom<&'a str> for NonQueryPagePath {
    type Error = PagePathError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        NonQueryPagePath::new(value)
    }
}

impl TryFrom<String> for NonQueryPagePath {
    type Error = PagePathError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        NonQueryPagePath::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_page_paths() {
        let valid_paths = vec![
            "pages/index/index",
            "pages/home/home",
            "pages/user/profile",
            "pages/product/detail",
        ];

        for path in valid_paths {
            assert!(
                NonQueryPagePath::new(path).is_ok(),
                "Path '{}' should be valid",
                path
            );
        }
    }

    #[test]
    fn test_invalid_page_paths() {
        // 以斜杠开头
        assert_eq!(
            NonQueryPagePath::new("/pages/index/index"),
            Err(PagePathError::StartsWithSlash)
        );

        // 包含参数
        assert_eq!(
            NonQueryPagePath::new("pages/index/index?foo=bar"),
            Err(PagePathError::ContainsParams)
        );

        // 格式不正确
        assert_eq!(
            NonQueryPagePath::new("pages/"),
            Err(PagePathError::InvalidFormat)
        );

        assert_eq!(
            NonQueryPagePath::new("/pages/"),
            Err(PagePathError::StartsWithSlash)
        );

        assert_eq!(
            NonQueryPagePath::new("pages//index"),
            Err(PagePathError::InvalidFormat)
        );
    }

    #[test]
    fn test_conversions() {
        // From str
        let path: Result<NonQueryPagePath, _> = "pages/index/index".try_into();
        assert!(path.is_ok());

        // From String
        let s = String::from("pages/home/home");
        let path: Result<NonQueryPagePath, _> = s.try_into();
        assert!(path.is_ok());

        // Into String
        let page_path = NonQueryPagePath::new("pages/index/index").unwrap();
        let s: String = page_path.into();
        assert_eq!(s, "pages/index/index");
    }

    #[test]
    fn test_display() {
        let path = NonQueryPagePath::new("pages/index/index").unwrap();
        assert_eq!(format!("{}", path), "pages/index/index");
    }
}
