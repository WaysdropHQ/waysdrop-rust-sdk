use regex::Regex;
use thiserror::Error;
use std::sync::LazyLock;

static API_KEY_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^wsp_(live|staging)_[a-f0-9]{64}$").unwrap());

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

pub fn validate_api_key(api_key: &str) -> Result<(), WaysdropError> {
    if API_KEY_RE.is_match(api_key) {
        Ok(())
    } else {
        Err(WaysdropError::InvalidApiKey(
            "expected wsp_live_… or wsp_staging_… with 64 hex chars".into(),
        ))
    }
}

pub fn infer_base_url(api_key: &str) -> &'static str {
    if api_key.starts_with("wsp_staging_") {
        "https://staging-api.waysdrop.com"
    } else if api_key.starts_with("wsp_live_") {
        "https://api.waysdrop.com"
    } else {
        "https://staging-api.waysdrop.com"
    }
}
