use crate::{
    Client, Result, constants,
    error::Error::{self, InternalServer},
};
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::debug;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QrCode {
    buffer: Vec<u8>,
}

impl QrCode {
    pub fn buffer(&self) -> &Vec<u8> {
        &self.buffer
    }
}

#[derive(Debug, Deserialize)]
pub struct QrCodeArgs {
    path: String,
    width: Option<i16>,
    auto_color: Option<bool>,
    line_color: Option<Rgb>,
    is_hyaline: Option<bool>,
    env_version: Option<MinappEnvVersion>,
}

#[derive(Debug, Deserialize)]
pub struct QrCodeArgBuilder {
    path: Option<String>,
    width: Option<i16>,
    auto_color: Option<bool>,
    line_color: Option<Rgb>,
    is_hyaline: Option<bool>,
    env_version: Option<MinappEnvVersion>,
}
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Rgb {
    r: i16,
    g: i16,
    b: i16,
}

impl Rgb {
    pub fn new(r: i16, g: i16, b: i16) -> Self {
        Rgb { r, g, b }
    }
}
impl QrCodeArgs {
    pub fn builder() -> QrCodeArgBuilder {
        QrCodeArgBuilder::new()
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn width(&self) -> Option<i16> {
        self.width
    }

    pub fn auto_color(&self) -> Option<bool> {
        self.auto_color
    }

    pub fn line_color(&self) -> Option<Rgb> {
        self.line_color.clone()
    }

    pub fn is_hyaline(&self) -> Option<bool> {
        self.is_hyaline
    }

    pub fn env_version(&self) -> Option<MinappEnvVersion> {
        self.env_version.clone()
    }
}

impl Default for QrCodeArgBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MinappEnvVersion {
    Release,
    Trial,
    Develop,
}

impl From<MinappEnvVersion> for String {
    fn from(value: MinappEnvVersion) -> Self {
        match value {
            MinappEnvVersion::Develop => "develop".to_string(),
            MinappEnvVersion::Release => "release".to_string(),
            MinappEnvVersion::Trial => "trial".to_string(),
        }
    }
}
impl QrCodeArgBuilder {
    pub fn new() -> Self {
        QrCodeArgBuilder {
            path: None,
            width: None,
            auto_color: None,
            line_color: None,
            is_hyaline: None,
            env_version: None,
        }
    }

    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    pub fn width(mut self, width: i16) -> Self {
        self.width = Some(width);
        self
    }

    pub fn with_auto_color(mut self) -> Self {
        self.auto_color = Some(true);
        self
    }

    pub fn line_color(mut self, color: Rgb) -> Self {
        self.line_color = Some(color);
        self
    }

    pub fn with_is_hyaline(mut self) -> Self {
        self.is_hyaline = Some(true);
        self
    }

    pub fn env_version(mut self, version: MinappEnvVersion) -> Self {
        self.env_version = Some(version);
        self
    }

    pub fn build(self) -> Result<QrCodeArgs> {
        let path = self.path.map_or_else(
            || {
                Err(Error::InvalidParameter(
                    "小程序页面路径不能为空".to_string(),
                ))
            },
            |v| {
                if v.len() > 1024 {
                    return Err(Error::InvalidParameter(
                        "页面路径最大长度 1024 个字符".to_string(),
                    ));
                }
                Ok(v)
            },
        )?;

        Ok(QrCodeArgs {
            path,
            width: self.width,
            auto_color: self.auto_color,
            line_color: self.line_color,
            is_hyaline: self.is_hyaline,
            env_version: self.env_version,
        })
    }
}

impl Client {
    /// ```ignore
    /// use wechat_minapp::{Client,QrCodeArgs};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let app_id = "your app id";
    ///     let secret = "your app secret";
    ///     
    ///     let client = Client::new(app_id, secret);
    ///     let qr_args = QrCodeArgs::builder().path(&page).build()?;
    ///     let buffer = minapp.qr_code(qr_args).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    pub async fn qr_code(&self, args: QrCodeArgs) -> Result<QrCode> {
        debug!("get qr code args {:?}", &args);

        let mut query = HashMap::new();
        let mut body = HashMap::new();

        query.insert("access_token", self.access_token().await?);
        body.insert("path", args.path);

        if let Some(width) = args.width {
            body.insert("width", width.to_string());
        }

        if let Some(auto_color) = args.auto_color {
            body.insert("auto_color", auto_color.to_string());
        }

        if let Some(line_color) = args.line_color {
            let value = serde_json::to_string(&line_color)?;
            body.insert("line_color", value);
        }

        if let Some(is_hyaline) = args.is_hyaline {
            body.insert("is_hyaline", is_hyaline.to_string());
        }

        if let Some(env_version) = args.env_version {
            body.insert("env_version", env_version.into());
        }

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert("encoding", HeaderValue::from_static("null"));

        let response = self
            .request()
            .post(constants::QR_CODE_ENDPOINT)
            .headers(headers)
            .query(&query)
            .json(&body)
            .send()
            .await?;

        debug!("response: {:#?}", response);

        if response.status().is_success() {
            let response = response.bytes().await?;

            Ok(QrCode {
                buffer: response.to_vec(),
            })
        } else {
            Err(InternalServer(response.text().await?))
        }
    }
}
