# waysdrop (Rust crate)

Official Waysdrop **Partner API** SDK for Rust (async, Tokio).

```bash
cargo add waysdrop
```

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

`wsp_staging_*` → staging API; `wsp_live_*` → production. Errors are `WaysdropError`.

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

| Method                                    | HTTP                          | Returns                   |
| ----------------------------------------- | ----------------------------- | ------------------------- |
| `get_wallet(currency)`                    | `GET /api/wallet`             | `MerchantWallet`          |
| `create_payment_checkout(body, currency)` | `POST /api/payments/checkout` | `PaymentCheckoutResponse` |

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

## License

MIT
