use regex::Regex;
use thiserror::Error;
use std::sync::LazyLock;

static SECRET_KEY_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^wsp_(live|staging)_[a-f0-9]{64}$").unwrap());
static PUBLIC_KEY_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^wsp_pub_(live|staging)_[a-f0-9]{64}$").unwrap());

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiKeyType {
    Secret,
    Public,
}

#[derive(Debug, Error)]
pub enum WaysdropError {
    #[error("waysdrop: {message} (status {status_code})")]
    Api {
        message: String,
        status_code: u16,
        details: Option<serde_json::Value>,
        quota: Option<serde_json::Value>,
        path: Option<String>,
    },
    #[error("invalid API key: {0}")]
    InvalidApiKey(String),
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}

pub fn infer_key_type(api_key: &str) -> Result<ApiKeyType, WaysdropError> {
    if PUBLIC_KEY_RE.is_match(api_key) {
        return Ok(ApiKeyType::Public);
    }
    if SECRET_KEY_RE.is_match(api_key) {
        return Ok(ApiKeyType::Secret);
    }
    Err(WaysdropError::InvalidApiKey(
        "expected wsp_live_/wsp_staging_ or wsp_pub_live_/wsp_pub_staging_ with 64 hex chars".into(),
    ))
}

pub fn validate_api_key(api_key: &str) -> Result<(), WaysdropError> {
    infer_key_type(api_key).map(|_| ())
}

pub fn infer_base_url(api_key: &str) -> &'static str {
    if api_key.contains("_staging_") {
        "https://staging-api.waysdrop.com"
    } else if api_key.contains("_live_") {
        "https://api.waysdrop.com"
    } else {
        "https://staging-api.waysdrop.com"
    }
}
