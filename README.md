# waysdrop

Official Waysdrop Partner API SDK for Rust.

## Install

```toml
[dependencies]
waysdrop = "1.0"
```

## Quickstart

```rust
use waysdrop::{WaysdropClient, WaysdropClientOptions};

#[tokio::main]
async fn main() -> Result<(), waysdrop::WaysdropError> {
    let client = WaysdropClient::new(WaysdropClientOptions::new(
        std::env::var("WAYSDROP_API_KEY").unwrap(),
    ))?;
    let account = client.get_account().await?;
    println!("{account}");
    Ok(())
}
```

## Webhooks

```rust
use waysdrop::{parse_webhook, verify_signature};

if !verify_signature(raw_body, signature_header, api_key) {
    return Err("invalid signature");
}
let (event, data) = parse_webhook(raw_body)?;
```

## License

MIT
