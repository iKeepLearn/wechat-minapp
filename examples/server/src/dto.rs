use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct AuthSign {
    #[validate(length(min = 10, message = "code 不能为空"))]
    pub code: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct EncryptedPayload {
    #[validate(length(min = 10, message = "code 不能为空"))]
    pub code: String,
    #[validate(length(min = 5, message = "encrypted_data 不能为空"))]
    pub encrypted_data: String,
    #[validate(length(min = 3, message = "iv 不能为空"))]
    pub iv: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct PhonePayload {
    #[validate(length(min = 10, message = "code 不能为空"))]
    pub code: String,
    pub openid: Option<String>,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct ContentCheck {
    #[validate(length(min = 10, message = "code 不能为空"))]
    pub code: String,
    #[validate(length(min = 3, message = "content 不能为空"))]
    pub content: String,
}

#[derive(Deserialize, Serialize, Validate,Clone)]
pub struct ShortLinkDto {
    #[validate(length(min = 10, message = " page_url 不能为空"))]
    pub page_url: String,
    pub page_title: Option<String>,
    pub is_permanent: Option<bool>,
}
