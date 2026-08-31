use sha2::Sha256;
use hmac::{Hmac, Mac};
use subtle::ConstantTimeEq;

type HmacSha256 = Hmac<Sha256>;

pub fn verify_signature(raw_body: &[u8], signature_header: &str, api_key: &str) -> bool {
    if signature_header.is_empty() {
        return false;
    }
    let mut mac = match HmacSha256::new_from_slice(api_key.as_bytes()) {
        Ok(m) => m,
        Err(_) => return false,
    };
    mac.update(raw_body);
    let expected = hex::encode(mac.finalize().into_bytes());
    expected.as_bytes().ct_eq(signature_header.as_bytes()).into()
}

pub fn parse_webhook(raw_body: &[u8]) -> Result<(String, serde_json::Value), String> {
    let payload: serde_json::Value =
        serde_json::from_slice(raw_body).map_err(|e| e.to_string())?;
    let event = payload
        .get("event")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "invalid webhook payload: missing event".to_string())?
        .to_string();
    let data = payload.get("data").cloned().unwrap_or(serde_json::Value::Null);
    Ok((event, data))
}

mod hex {
    pub fn encode(bytes: impl AsRef<[u8]>) -> String {
        bytes.as_ref().iter().map(|b| format!("{b:02x}")).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn verify_and_parse() {
        let fixture: serde_json::Value = serde_json::from_str(
            &fs::read_to_string("../waysdrop-api-spec/fixtures/signature.json").unwrap(),
        )
        .unwrap();
        let raw = fixture["rawBody"].as_str().unwrap();
        let sig = fixture["signature"].as_str().unwrap();
        let key = fixture["apiKey"].as_str().unwrap();
        assert!(verify_signature(raw.as_bytes(), sig, key));
        assert!(!verify_signature(raw.as_bytes(), "bad", key));
        let (event, data) = parse_webhook(raw.as_bytes()).unwrap();
        assert_eq!(event, "p2p.delivery.created");
        assert_eq!(data["trackingId"], "P2P-TEST-001");
    }
}
