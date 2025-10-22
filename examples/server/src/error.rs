use actix_web::{HttpResponse, ResponseError, http::StatusCode, mime};
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::convert::Infallible;
use thiserror::Error;
use wechat_minapp::error::Error as WechatMinappError;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Internal error")]
    Internal(String),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    InvalidArgument(String),
    #[error("{0}")]
    InvalidAuth(String),
    #[error("{0}")]
    BadRequest(String),
}


impl From<Infallible> for Error {
    fn from(value: Infallible) -> Self {
        Error::InvalidArgument(value.to_string())
    }
}

impl From<Vec<u8>> for Error {
    fn from(value: Vec<u8>) -> Self {
        Error::InvalidArgument(String::from_utf8_lossy(&value).to_string())
    }
}

impl From<anyhow::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        Error::Internal(value.to_string())
    }
}


impl From<WechatMinappError> for Error {
    fn from(value: WechatMinappError) -> Self {
        match value {
            WechatMinappError::MissingCode(e) => Error::InvalidArgument(e),
            WechatMinappError::InvalidCode(e) => Error::InvalidArgument(e),
            _ => Error::Internal("wechat minapp error".to_string()),
        }
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::InvalidArgument(err.to_string())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ErrorResponse {
    status_code: u16,
    timestamp: DateTime<Utc>,
    message: String,
    path: String,
}

impl ErrorResponse {
    pub fn new(message: &str, status_code: u16) -> Self {
        ErrorResponse {
            status_code,
            timestamp: Utc::now(),
            message: message.to_string(),
            path: "/".to_string(),
        }
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::InvalidArgument(_) => StatusCode::BAD_REQUEST,
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::InvalidAuth(_) => StatusCode::UNAUTHORIZED,
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let res = match self {
            Error::Internal(message) => ErrorResponse::new(message, self.status_code().as_u16()),
            Error::InvalidArgument(message) => {
                ErrorResponse::new(message, self.status_code().as_u16())
            }
            Error::NotFound(message) => ErrorResponse::new(message, self.status_code().as_u16()),
            Error::InvalidAuth(message) => ErrorResponse::new(message, self.status_code().as_u16()),
            Error::BadRequest(message) => ErrorResponse::new(message, self.status_code().as_u16()),
        };
        HttpResponse::build(self.status_code())
            .content_type(mime::APPLICATION_JSON)
            .json(res)
    }
}
