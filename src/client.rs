use std::time::Duration;
use serde_json::{json, Value};
use std::collections::HashMap;
use reqwest::Client as HttpClient;
use crate::error::{infer_base_url, validate_api_key, WaysdropError};

pub struct WaysdropClient {
    api_key: String,
    base_url: String,
    http: HttpClient,
    display_currency: Option<String>,
    correlation_id: Option<String>,
}

pub struct WaysdropClientOptions {
    pub api_key: String,
    pub base_url: Option<String>,
    pub timeout: Option<Duration>,
    pub display_currency: Option<String>,
    pub correlation_id: Option<String>,
}

impl WaysdropClientOptions {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: None,
            timeout: None,
            display_currency: None,
            correlation_id: None,
        }
    }
}

impl WaysdropClient {
    pub fn new(opts: WaysdropClientOptions) -> Result<Self, WaysdropError> {
        validate_api_key(&opts.api_key)?;
        let timeout = opts.timeout.unwrap_or(Duration::from_secs(30));
        let http = HttpClient::builder().timeout(timeout).build()?;
        Ok(Self {
            base_url: opts
                .base_url
                .unwrap_or_else(|| infer_base_url(&opts.api_key).to_string())
                .trim_end_matches('/')
                .to_string(),
            api_key: opts.api_key,
            http,
            display_currency: opts.display_currency,
            correlation_id: opts.correlation_id,
        })
    }

    pub async fn get_account(&self) -> Result<Value, WaysdropError> {
        self.get("/api/account", None).await
    }

    pub async fn list_fleet_types(&self) -> Result<Value, WaysdropError> {
        self.get("/api/fleet-types", None).await
    }

    pub async fn list_countries(
        &self,
        search: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Value, WaysdropError> {
        let mut q = HashMap::new();
        if let Some(s) = search {
            q.insert("search".into(), s.to_string());
        }
        if let Some(l) = limit {
            q.insert("limit".into(), l.to_string());
        }
        self.get("/api/countries", Some(q)).await
    }

    pub async fn list_states(
        &self,
        search: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Value, WaysdropError> {
        let mut q = HashMap::new();
        if let Some(s) = search {
            q.insert("search".into(), s.to_string());
        }
        if let Some(l) = limit {
            q.insert("limit".into(), l.to_string());
        }
        self.get("/api/states", Some(q)).await
    }

    pub async fn list_cities(
        &self,
        search: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Value, WaysdropError> {
        let mut q = HashMap::new();
        if let Some(s) = search {
            q.insert("search".into(), s.to_string());
        }
        if let Some(l) = limit {
            q.insert("limit".into(), l.to_string());
        }
        self.get("/api/cities", Some(q)).await
    }

    pub async fn get_route(&self, origin: Value, destination: Value) -> Result<Value, WaysdropError> {
        self.post("/api/route", json!({ "origin": origin, "destination": destination }), None)
            .await
    }

    pub async fn get_pricing(&self, body: Value, currency: Option<&str>) -> Result<Value, WaysdropError> {
        self.post("/api/pricing", self.with_currency(body, currency), currency)
            .await
    }

    pub async fn create_delivery_request(
        &self,
        body: Value,
        currency: Option<&str>,
    ) -> Result<Value, WaysdropError> {
        self.post("/api/request", self.with_currency(body, currency), currency)
            .await
    }

    pub async fn cancel_delivery_request(&self, delivery_id: &str) -> Result<Value, WaysdropError> {
        self.post(&format!("/api/request/{delivery_id}/cancel"), json!({}), None)
            .await
    }

    pub async fn create_or_update_package(
        &self,
        body: Value,
        currency: Option<&str>,
    ) -> Result<Value, WaysdropError> {
        self.post("/api/package", self.with_currency(body, currency), currency)
            .await
    }

    pub async fn delete_package(&self, package_id: &str) -> Result<(), WaysdropError> {
        self.request("DELETE", &format!("/api/package/{package_id}"), None, None)
            .await?;
        Ok(())
    }

    pub async fn list_packages(&self, currency: Option<&str>) -> Result<Value, WaysdropError> {
        self.get("/api/packages", self.currency_query(currency)).await
    }

    pub async fn get_wallet(&self, currency: Option<&str>) -> Result<Value, WaysdropError> {
        self.get("/api/wallet", self.currency_query(currency)).await
    }

    pub async fn create_payment_checkout(
        &self,
        body: Value,
        currency: Option<&str>,
    ) -> Result<Value, WaysdropError> {
        self.post("/api/payments/checkout", self.with_currency(body, currency), currency)
            .await
    }

    pub async fn get_exchange_rate(&self, from: &str, to: &str) -> Result<Value, WaysdropError> {
        let mut q = HashMap::new();
        q.insert("from".into(), from.to_string());
        q.insert("to".into(), to.to_string());
        self.get("/api/exchange-rate", Some(q)).await
    }

    pub async fn convert_currency(
        &self,
        amount: f64,
        from: &str,
        to: &str,
    ) -> Result<Value, WaysdropError> {
        let mut q = HashMap::new();
        q.insert("amount".into(), amount.to_string());
        q.insert("from".into(), from.to_string());
        q.insert("to".into(), to.to_string());
        self.get("/api/convert", Some(q)).await
    }

    pub async fn list_deliveries(&self, params: HashMap<String, String>) -> Result<Value, WaysdropError> {
        self.get("/api/deliveries", Some(params)).await
    }

    pub async fn get_delivery(&self, delivery_id: &str, currency: Option<&str>) -> Result<Value, WaysdropError> {
        self.get(
            &format!("/api/deliveries/{delivery_id}"),
            self.currency_query(currency),
        )
        .await
    }

    async fn get(
        &self,
        path: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, WaysdropError> {
        self.request("GET", path, None, query).await
    }

    async fn post(
        &self,
        path: &str,
        body: Value,
        currency: Option<&str>,
    ) -> Result<Value, WaysdropError> {
        self.request("POST", path, Some(body), self.currency_query(currency))
            .await
    }

    async fn request(
        &self,
        method: &str,
        path: &str,
        body: Option<Value>,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, WaysdropError> {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self.http.request(method.parse().unwrap(), &url);
        req = req.header("api-key", &self.api_key).header("Accept", "application/json");
        if let Some(id) = &self.correlation_id {
            req = req.header("X-Correlation-Id", id);
        }
        if let Some(q) = query {
            req = req.query(&q);
        }
        if let Some(b) = body {
            req = req.json(&b);
        }

        let resp = req.send().await?;
        if resp.status().as_u16() == 204 {
            return Ok(Value::Null);
        }

        let status = resp.status().as_u16();
        let parsed: Value = resp.json().await.unwrap_or(Value::Null);

        if status >= 400 {
            let message = parsed
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("request failed")
                .to_string();
            return Err(WaysdropError::Api {
                message,
                status_code: parsed
                    .get("statusCode")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(status as u64) as u16,
                details: parsed.get("details").cloned(),
                quota: parsed.get("quota").cloned(),
                path: parsed
                    .get("path")
                    .and_then(|v| v.as_str())
                    .map(String::from),
            });
        }

        if parsed.get("success").and_then(|v| v.as_bool()) == Some(true) {
            return Ok(parsed.get("data").cloned().unwrap_or(Value::Null));
        }

        Ok(parsed)
    }

    fn with_currency(&self, mut body: Value, currency: Option<&str>) -> Value {
        let c = currency.or(self.display_currency.as_deref());
        if let (Some(cur), Value::Object(ref mut map)) = (c, &mut body) {
            if !map.contains_key("currency") {
                map.insert("currency".into(), Value::String(cur.to_string()));
            }
        }
        body
    }

    fn currency_query(&self, currency: Option<&str>) -> Option<HashMap<String, String>> {
        let c = currency.or(self.display_currency.as_deref())?;
        let mut q = HashMap::new();
        q.insert("currency".into(), c.to_string());
        Some(q)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::validate_api_key;

    #[test]
    fn validate_key() {
        let fixture: Value = serde_json::from_str(
            &std::fs::read_to_string("../waysdrop-api-spec/fixtures/signature.json").unwrap(),
        )
        .unwrap();
        validate_api_key(fixture["apiKey"].as_str().unwrap()).unwrap();
    }
}
