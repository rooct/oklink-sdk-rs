# OKLink SDK for Rust

A Rust SDK for interacting with the OKLink API.

## Features

- Interact with OKLink API endpoints.
- Easy-to-use interface for common operations.
- Supports various API functionalities.

## Installation

Add the following to your `Cargo.toml`:

#### usage
```rust
use oklink_sdk_rs::OklinkClient;

#[tokio::main]
async fn main() {
        let client = OkLinkClient::new(
            "https://www.oklink.com/api/v5/explorer".to_string(),
            api_key,
            "btc".to_string(),
            1,
        );
}
```