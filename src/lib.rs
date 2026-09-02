mod error;
mod types;
mod client;
mod webhooks;
pub mod oauth;

pub use types::*;
pub use webhooks::{parse_webhook, verify_signature};
pub use client::{WaysdropClient, WaysdropClientOptions};
pub use error::{infer_base_url, validate_api_key, WaysdropError};
pub use oauth::{
    build_authorize_url, generate_code_challenge, generate_code_verifier, generate_pkce_pair,
    infer_oauth_issuer, validate_client_id, DiscoveryDocument, OAuthClient, OAuthClientOptions,
    OAuthError, PkcePair, TokenResponse, UserInfo,
};
