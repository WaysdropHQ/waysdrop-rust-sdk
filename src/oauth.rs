use regex::Regex;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OAuthError {
    #[error("invalid client id: {0}")]
    InvalidClientId(String),
    #[error("oauth request failed ({status}): {message}")]
    Request { status: u16, message: String },
}

#[derive(Debug, Clone, Deserialize)]
pub struct DiscoveryDocument {
    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub userinfo_endpoint: String,
    pub revocation_endpoint: String,
    pub response_types_supported: Vec<String>,
    pub grant_types_supported: Vec<String>,
    pub scopes_supported: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub refresh_token: String,
    pub scope: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserInfo {
    pub sub: String,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PkcePair {
    pub code_verifier: String,
    pub code_challenge: String,
    pub code_challenge_method: String,
}

pub struct OAuthClientOptions {
    pub client_id: String,
    pub client_secret: Option<String>,
    pub redirect_uri: String,
    pub issuer: Option<String>,
    pub timeout_ms: Option<u64>,
}

pub struct OAuthClient {
    client_id: String,
    client_secret: Option<String>,
    redirect_uri: String,
    issuer: String,
    http: reqwest::Client,
    discovery: Option<DiscoveryDocument>,
}

pub fn infer_oauth_issuer(client_id: &str) -> String {
    if client_id.starts_with("wdo_staging_") {
        "https://staging-api.waysdrop.com".into()
    } else if client_id.starts_with("wdo_live_") {
        "https://api.waysdrop.com".into()
    } else {
        "https://staging-api.waysdrop.com".into()
    }
}

pub fn validate_client_id(client_id: &str) -> Result<(), OAuthError> {
    let re = Regex::new(r"^wdo_(live|staging)_[a-f0-9]{32}$").unwrap();
    if re.is_match(client_id) {
        Ok(())
    } else {
        Err(OAuthError::InvalidClientId(client_id.to_string()))
    }
}

pub fn generate_code_verifier(length: usize) -> String {
    use rand::RngCore;
    let mut bytes = vec![0u8; length];
    rand::thread_rng().fill_bytes(&mut bytes);
    base64_url(&bytes)[..length].to_string()
}

pub fn generate_code_challenge(code_verifier: &str, method: &str) -> String {
    if method == "plain" {
        return code_verifier.to_string();
    }
    let digest = Sha256::digest(code_verifier.as_bytes());
    base64_url(&digest)
}

pub fn generate_pkce_pair() -> PkcePair {
    let code_verifier = generate_code_verifier(64);
    PkcePair {
        code_challenge: generate_code_challenge(&code_verifier, "S256"),
        code_verifier,
        code_challenge_method: "S256".into(),
    }
}

pub fn build_authorize_url(
    authorization_endpoint: &str,
    client_id: &str,
    redirect_uri: &str,
    scope: Option<&str>,
    state: Option<&str>,
    pkce: Option<&PkcePair>,
) -> String {
    let mut url = url::Url::parse(authorization_endpoint).expect("authorize url");
    {
        let mut q = url.query_pairs_mut();
        q.append_pair("client_id", client_id);
        q.append_pair("redirect_uri", redirect_uri);
        q.append_pair("response_type", "code");
        q.append_pair("scope", scope.unwrap_or("openid profile email"));
        if let Some(state) = state {
            q.append_pair("state", state);
        }
        if let Some(pkce) = pkce {
            q.append_pair("code_challenge", &pkce.code_challenge);
            q.append_pair("code_challenge_method", &pkce.code_challenge_method);
        }
    }
    url.to_string()
}

impl OAuthClient {
    pub fn new(opts: OAuthClientOptions) -> Result<Self, OAuthError> {
        validate_client_id(&opts.client_id)?;
        let issuer = opts
            .issuer
            .unwrap_or_else(|| infer_oauth_issuer(&opts.client_id))
            .trim_end_matches('/')
            .to_string();
        let timeout = opts.timeout_ms.unwrap_or(30_000);
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_millis(timeout))
            .build()
            .map_err(|e| OAuthError::Request {
                status: 500,
                message: e.to_string(),
            })?;
        Ok(Self {
            client_id: opts.client_id,
            client_secret: opts.client_secret,
            redirect_uri: opts.redirect_uri,
            issuer,
            http,
            discovery: None,
        })
    }

    pub async fn get_discovery(&mut self) -> Result<&DiscoveryDocument, OAuthError> {
        if self.discovery.is_none() {
            let url = format!("{}/oauth/.well-known/openid-configuration", self.issuer);
            let res = self.http.get(url).send().await.map_err(map_reqwest)?;
            let status = res.status().as_u16();
            if status >= 400 {
                return Err(OAuthError::Request {
                    status,
                    message: res.text().await.unwrap_or_default(),
                });
            }
            self.discovery = Some(res.json().await.map_err(map_reqwest)?);
        }
        Ok(self.discovery.as_ref().unwrap())
    }

    pub fn build_authorize_url(
        &self,
        scope: Option<&str>,
        state: Option<&str>,
        pkce: Option<&PkcePair>,
    ) -> String {
        build_authorize_url(
            &format!("{}/oauth/authorize", self.issuer),
            &self.client_id,
            &self.redirect_uri,
            scope,
            state,
            pkce,
        )
    }

    pub async fn exchange_code(
        &self,
        code: &str,
        code_verifier: Option<&str>,
    ) -> Result<TokenResponse, OAuthError> {
        let mut body = serde_json::Map::new();
        body.insert("grant_type".into(), "authorization_code".into());
        body.insert("code".into(), code.into());
        body.insert("redirect_uri".into(), self.redirect_uri.clone().into());
        body.insert("client_id".into(), self.client_id.clone().into());
        if let Some(secret) = &self.client_secret {
            body.insert("client_secret".into(), secret.clone().into());
        }
        if let Some(verifier) = code_verifier {
            body.insert("code_verifier".into(), verifier.into());
        }
        self.token_request(body).await
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> Result<TokenResponse, OAuthError> {
        let mut body = serde_json::Map::new();
        body.insert("grant_type".into(), "refresh_token".into());
        body.insert("refresh_token".into(), refresh_token.into());
        body.insert("client_id".into(), self.client_id.clone().into());
        if let Some(secret) = &self.client_secret {
            body.insert("client_secret".into(), secret.clone().into());
        }
        self.token_request(body).await
    }

    pub async fn revoke_token(&self, token: &str) -> Result<(), OAuthError> {
        let mut body = serde_json::Map::new();
        body.insert("token".into(), token.into());
        body.insert("client_id".into(), self.client_id.clone().into());
        if let Some(secret) = &self.client_secret {
            body.insert("client_secret".into(), secret.clone().into());
        }
        let res = self
            .http
            .post(format!("{}/oauth/revoke", self.issuer))
            .header(CONTENT_TYPE, "application/json")
            .json(&body)
            .send()
            .await
            .map_err(map_reqwest)?;
        if res.status().as_u16() >= 400 {
            return Err(OAuthError::Request {
                status: res.status().as_u16(),
                message: res.text().await.unwrap_or_default(),
            });
        }
        Ok(())
    }

    pub async fn get_user_info(&self, access_token: &str) -> Result<UserInfo, OAuthError> {
        let res = self
            .http
            .get(format!("{}/oauth/userinfo", self.issuer))
            .header(AUTHORIZATION, format!("Bearer {access_token}"))
            .send()
            .await
            .map_err(map_reqwest)?;
        let status = res.status().as_u16();
        if status >= 400 {
            return Err(OAuthError::Request {
                status,
                message: res.text().await.unwrap_or_default(),
            });
        }
        res.json().await.map_err(map_reqwest)
    }

    async fn token_request(&self, body: serde_json::Map<String, serde_json::Value>) -> Result<TokenResponse, OAuthError> {
        let res = self
            .http
            .post(format!("{}/oauth/token", self.issuer))
            .header(CONTENT_TYPE, "application/json")
            .json(&body)
            .send()
            .await
            .map_err(map_reqwest)?;
        let status = res.status().as_u16();
        if status >= 400 {
            return Err(OAuthError::Request {
                status,
                message: res.text().await.unwrap_or_default(),
            });
        }
        res.json().await.map_err(map_reqwest)
    }
}

fn base64_url(bytes: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

fn map_reqwest(err: reqwest::Error) -> OAuthError {
    OAuthError::Request {
        status: 500,
        message: err.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn pkce_vector_matches_fixture() {
        let raw = fs::read_to_string("tests/fixtures/oauth.json").expect("fixture");
        let fx: serde_json::Value = serde_json::from_str(&raw).unwrap();
        let verifier = fx["code_verifier"].as_str().unwrap();
        let challenge = fx["code_challenge"].as_str().unwrap();
        assert_eq!(generate_code_challenge(verifier, "S256"), challenge);
    }

    #[test]
    fn validate_client_id_ok() {
        validate_client_id("wdo_staging_a1b2c3d4e5f6789012345678901234ab").unwrap();
    }
}
