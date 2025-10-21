use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Clone, Serialize)]
pub struct AccessToken {
    pub access_token: String,
    pub expired_at: DateTime<Utc>,
}

impl std::fmt::Debug for AccessToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AccessToken")
            .field("access_token", &"********")
            .field("expired_at", &self.expired_at)
            .finish()
    }
}

#[derive(Deserialize)]
pub(crate) struct AccessTokenBuilder {
    pub access_token: String,
    #[serde(
        deserialize_with = "AccessTokenBuilder::deserialize_expired_at",
        rename = "expires_in"
    )]
    pub expired_at: DateTime<Utc>,
}

impl AccessTokenBuilder {
    fn deserialize_expired_at<'de, D>(
        deserializer: D,
    ) -> std::result::Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let seconds = Duration::seconds(i64::deserialize(deserializer)?);

        Ok(Utc::now() + seconds)
    }
}

impl std::fmt::Debug for AccessTokenBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AccessTokenBuilder")
            .field("access_token", &"********")
            .field("expired_at", &self.expired_at)
            .finish()
    }
}

impl AccessTokenBuilder {
    pub fn build(self) -> AccessToken {
        AccessToken {
            access_token: self.access_token,
            expired_at: self.expired_at,
        }
    }
}

/// 检查令牌是否过期
///
/// 添加安全边界，在令牌过期前5分钟就认为需要刷新
pub(crate) fn is_token_expired(token: &AccessToken) -> bool {
    // 添加安全边界，提前刷新
    let now = Utc::now();
    token.expired_at.signed_duration_since(now) < Duration::minutes(5)
}
