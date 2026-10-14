# auth0-sdk

[![License: Apache-2.0 OR MIT](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg)](LICENSE-APACHE)

An asynchronous Rust client for selected endpoints of the
[Auth0 Authentication API](https://auth0.com/docs/api/authentication) and
[Management API](https://auth0.com/docs/api/management/v2).

## What it supports

- Authentication: client credentials, authorization code (including PKCE),
  device authorization code requests, and the resource owner password flow.
- Management: list and retrieve users and clients.

The resource owner password flow is included for compatibility and must be
enabled and configured in your Auth0 tenant. Prefer modern authorization flows
for new applications.

The Management API currently exposes read operations for users and clients.
Calls require an access token with the relevant Management API permissions.

## Installation

The crate is currently available from GitHub:

```toml
[dependencies]
auth0-sdk = { git = "https://github.com/hypermynds/rust-auth0-sdk" }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

The API is asynchronous, so use it from an async runtime such as Tokio.

## Request an access token

Create an Auth0 application configured for the client credentials flow and an
API with the required audience. Set `AUTH0_DOMAIN` to your tenant URL (including
`https://`), and provide the application credentials and API audience through
environment variables.

```rust
use auth0_sdk::AuthenticationApi;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let domain = env::var("AUTH0_DOMAIN")?;
    let client_id = env::var("AUTH0_CLIENT_ID")?;
    let client_secret = env::var("AUTH0_CLIENT_SECRET")?;
    let audience = env::var("AUTH0_API_AUDIENCE")?;

    let auth = AuthenticationApi::with_client_secret(&domain, client_id, client_secret)?;
    let token = auth.get_token(audience)?.send().await?;

    println!("Access token expires in {} seconds", token.expires_in);
    Ok(())
}
```

Do not commit credentials or print access tokens in application logs.

## Call the Management API

Provide a Management API access token with the required permissions, such as
`read:users` to list users. `ManagementApi` adds the Bearer authorization
scheme to the token.

```rust
use auth0_sdk::ManagementApi;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let domain = env::var("AUTH0_DOMAIN")?;
    let management_token = env::var("AUTH0_MANAGEMENT_API_TOKEN")?;

    let management = ManagementApi::new(&domain, &management_token)?;
    let response = management.users().list().per_page(10).send().await?;

    println!("Retrieved {} users", response.users.len());
    Ok(())
}
```

The Management API token must target your Auth0 tenant's Management API and
include the permissions needed for the requested endpoint. Keep it secret.

## Development

Install [Just](https://just.systems/) to use the repository commands:

```sh
just           # list available commands
just ci-check  # run formatting, Clippy, and tests
```

The equivalent checks can also be run directly with Cargo:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --tests --all-features -- -D warnings
cargo test --workspace --all-features
```

## License

Licensed under either of [Apache License 2.0](LICENSE-APACHE) or
[MIT License](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
