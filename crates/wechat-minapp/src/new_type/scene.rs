use std::fmt;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationSceneError {
    #[error("字符串长度超过32个字符限制")]
    TooLong,
    #[error("包含非法字符: {0}")]
    InvalidChar(char),
}

// 合法的字符集：数字、大小写英文、!#$&'()*+,/:;=?@-._~
const VALID_CHARS: &str =
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!#$&'()*+,/:;=?@-._~";

/// 最大32个可见字符，只支持数字，大小写英文以及部分特殊字符：!#$&'()*+,/:;=?@-._~，其它字符请自行编码为合法字符（因不支持%，中文无法使用 urlencode 处理，请使用其他编码方式）
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SceneString(String);

impl SceneString {
    /// 创建新的 SceneString，进行验证
    pub fn new(s: &str) -> Result<Self, ValidationSceneError> {
        if s.len() > 32 {
            return Err(ValidationSceneError::TooLong);
        }

        for c in s.chars() {
            if !VALID_CHARS.contains(c) {
                return Err(ValidationSceneError::InvalidChar(c));
            }
        }

        Ok(SceneString(s.to_string()))
    }

    /// 获取内部字符串引用
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// 获取内部字符串
    pub fn into_inner(self) -> String {
        self.0
    }

    /// 检查字符串是否为空
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// 获取字符串长度
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl FromStr for SceneString {
    type Err = ValidationSceneError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        SceneString::new(s)
    }
}

impl fmt::Display for SceneString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for SceneString {
    type Error = ValidationSceneError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        SceneString::new(&value)
    }
}

impl TryFrom<&str> for SceneString {
    type Error = ValidationSceneError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        SceneString::new(value)
    }
}

// 为方便使用，实现 Deref
impl std::ops::Deref for SceneString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_string() {
        // 测试合法字符串
        assert!(SceneString::new("hello123").is_ok());
        assert!(SceneString::new("HELLO_WORLD-123").is_ok());
        assert!(SceneString::new("test!#$&'()*+,/:;=?@-._~").is_ok());

        // 测试长度限制
        let long_string = "a".repeat(33);
        assert!(matches!(
            SceneString::new(&long_string),
            Err(ValidationSceneError::TooLong)
        ));

        // 测试边界情况
        let max_length_string = "a".repeat(32);
        assert!(SceneString::new(&max_length_string).is_ok());

        // 测试非法字符
        assert!(matches!(
            SceneString::new("hello world"), // 包含空格
            Err(ValidationSceneError::InvalidChar(' '))
        ));

        assert!(matches!(
            SceneString::new("中文"), // 包含中文
            Err(ValidationSceneError::InvalidChar(_))
        ));
    }

    #[test]
    fn test_conversions() {
        // 测试 FromStr
        let from_str = "test123".parse::<SceneString>();
        assert!(from_str.is_ok());

        // 测试 TryFrom
        let from_string = SceneString::try_from("test123".to_string());
        assert!(from_string.is_ok());

        let from_str_ref = SceneString::try_from("test123");
        assert!(from_str_ref.is_ok());
    }
}
