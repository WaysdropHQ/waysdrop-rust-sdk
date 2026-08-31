mod error;
mod client;
mod webhooks;

pub use client::{WaysdropClient, WaysdropClientOptions};
pub use error::{infer_base_url, validate_api_key, WaysdropError};
pub use webhooks::{parse_webhook, verify_signature};
