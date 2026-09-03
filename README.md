# waysdrop (Rust crate)

Official Waysdrop SDK for Rust (async, Tokio).

Partner API (`/api/*`), webhook helpers, and **OAuth** (Sign in with Waysdrop) in module `waysdrop::oauth`.

```bash
cargo add waysdrop
```

## Authentication

API keys are sent as the `api-key` header.

| Key type | Prefix                                        | Default base URL                                                                |
| -------- | --------------------------------------------- | ------------------------------------------------------------------------------- |
| Secret   | `wsp_live_` / `wsp_staging_` + 64 hex         | staging → `https://staging-api.waysdrop.com`, live → `https://api.waysdrop.com` |
| Public   | `wsp_pub_live_` / `wsp_pub_staging_` + 64 hex | same                                                                            |

```rust
use waysdrop::{infer_key_type, validate_api_key, ApiKeyType};

validate_api_key("wsp_staging_...")?;
let kind = infer_key_type("wsp_pub_live_...")?; // ApiKeyType::Public
```

### Public API keys (v1.2+)

Public keys work in the browser on quote/geo and payment routes (configure **allowed origins** in the dashboard). Deliveries, wallet, and packages require the **secret** key on your backend.

## Client

```rust
use waysdrop::{WaysdropClient, WaysdropClientOptions};

#[tokio::main]
async fn main() -> Result<(), waysdrop::WaysdropError> {
    let client = WaysdropClient::new(WaysdropClientOptions {
        api_key: std::env::var("WAYSDROP_API_KEY").expect("WAYSDROP_API_KEY"),
        display_currency: Some("NGN".into()),
        ..WaysdropClientOptions::new("wsp_staging_...")
    })?;

    let account = client.get_account().await?;
    println!("{:?}", account.user_id);
    Ok(())
}
```

Errors are `WaysdropError`.

---

## API reference

All methods are `async` on `WaysdropClient`.

### Account

| Method          | HTTP               | Returns          |
| --------------- | ------------------ | ---------------- |
| `get_account()` | `GET /api/account` | `AccountSummary` |

### Locations

| Method                          | HTTP                 | Parameters                    |
| ------------------------------- | -------------------- | ----------------------------- |
| `list_countries(search, limit)` | `GET /api/countries` | `Option<&str>`, `Option<u32>` |
| `list_states(search, limit)`    | `GET /api/states`    | same                          |
| `list_cities(search, limit)`    | `GET /api/cities`    | same                          |

### Routing & pricing

| Method                                  | HTTP                   | Returns             |
| --------------------------------------- | ---------------------- | ------------------- |
| `get_route(origin, destination: Value)` | `POST /api/route`      | `RouteDataResponse` |
| `list_fleet_types()`                    | `GET /api/fleet-types` | `Vec<FleetType>`    |
| `get_pricing(body: Value, currency)`    | `POST /api/pricing`    | `PricingResponse`   |

```rust
use serde_json::json;

let pricing = client.get_pricing(
    json!({
        "origin": { "address": "Ikeja, Lagos" },
        "destination": { "address": "Lekki, Lagos" },
        "packagesId": ["package-uuid"],
        "courierSelection": "ANYONE"
    }),
    Some("NGN"),
).await?;
```

### Packages

| Method                                     | HTTP                      | Returns                |
| ------------------------------------------ | ------------------------- | ---------------------- |
| `create_or_update_package(body, currency)` | `POST /api/package`       | `DeliveryPackage`      |
| `delete_package(package_id)`               | `DELETE /api/package/:id` | `()`                   |
| `list_packages(currency)`                  | `GET /api/packages`       | `Vec<DeliveryPackage>` |

### Deliveries

| Method                                             | HTTP                           | Returns                  |
| -------------------------------------------------- | ------------------------------ | ------------------------ |
| `create_delivery_request(body, currency)`          | `POST /api/request`            | `CreateDeliveryResponse` |
| `cancel_delivery_request(delivery_id)`             | `POST /api/request/:id/cancel` | `CancelDeliveryResponse` |
| `list_deliveries(params: HashMap<String, String>)` | `GET /api/deliveries`          | `ListDeliveriesResponse` |
| `get_delivery(delivery_id, currency)`              | `GET /api/deliveries/:id`      | `DeliveryDetail`         |

### Wallet & payments

| Method                                    | HTTP                                            | Returns                   |
| ----------------------------------------- | ----------------------------------------------- | ------------------------- |
| `get_wallet(currency)`                    | `GET /api/wallet`                               | `MerchantWallet`          |
| `create_payment_checkout(body, currency)` | `POST /api/payments/checkout`                   | `PaymentCheckoutResponse` |
| `get_payment_by_external_reference(ref)`  | `GET /api/payments/by-external-reference/{ref}` | deposit summary           |

Include `externalReference` in checkout / delivery bodies for idempotent reconciliation.

### FX

| Method                               | HTTP                     | Returns                   |
| ------------------------------------ | ------------------------ | ------------------------- |
| `get_exchange_rate(from, to)`        | `GET /api/exchange-rate` | `ExchangeRateResponse`    |
| `convert_currency(amount, from, to)` | `GET /api/convert`       | `ConvertCurrencyResponse` |

Request bodies for create/pricing/route use `serde_json::Value` or `json!()` macros.

---

## Webhooks

Crate-level functions:

| Function                                                | Returns                           |
| ------------------------------------------------------- | --------------------------------- |
| `verify_signature(raw_body, signature_header, api_key)` | `bool`                            |
| `parse_webhook(raw_body)`                               | `WebhookEnvelope { event, data }` |

```rust
use waysdrop::{parse_webhook, verify_signature};

assert!(verify_signature(raw_body, &sig, &api_key));
let envelope = parse_webhook(raw_body).expect("valid json");
```

---

## Types

Exported from the crate root: `AccountSummary`, `PricingResponse`, `DeliveryDetail`, `WebhookEnvelope`, etc. See `src/types.rs`.

---

## OAuth (Sign in with Waysdrop)

Module `waysdrop::oauth`. Client IDs: `wdo_live_<32 hex>` / `wdo_staging_<32 hex>`.

| Method                                                 | Description          |
| ------------------------------------------------------ | -------------------- |
| `OAuthClient::get_discovery()`                         | OpenID configuration |
| `OAuthClient::build_authorize_url(scope, state, pkce)` | Authorization URL    |
| `OAuthClient::exchange_code(code, code_verifier)`      | Code → tokens        |
| `OAuthClient::refresh_token(refresh_token)`            | Refresh access token |
| `OAuthClient::revoke_token(token)`                     | Revoke token         |
| `OAuthClient::get_user_info(access_token)`             | User profile         |

PKCE: `generate_pkce_pair()`, `generate_code_verifier()`, `generate_code_challenge()`.

```rust
use waysdrop::oauth::{OAuthClient, OAuthClientOptions, generate_pkce_pair};

let mut oauth = OAuthClient::new(OAuthClientOptions {
    client_id: "wdo_staging_…".into(),
    client_secret: Some("wdos_…".into()), // confidential apps only
    redirect_uri: "https://example.com/oauth/callback".into(),
    issuer: None,
    timeout_ms: None,
})?;

let pkce = generate_pkce_pair();
let authorize_url = oauth.build_authorize_url(
    Some("openid profile email"),
    Some("csrf"),
    Some(&pkce),
);
// Redirect → on callback:
let tokens = oauth.exchange_code(&code, Some(&pkce.code_verifier)).await?;
let user = oauth.get_user_info(&tokens.access_token).await?;
```

OAuth responses are raw JSON. Errors are `OAuthError`.

See [OAuth docs](https://docs.waysdrop.com/get-started/oauth).

---

## License

MIT
