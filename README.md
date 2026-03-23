# network

Small Rust utilities for Postgres (Diesel), Redis, and HTTP client integrations.

## Why this crate exists

This crate provides lightweight wrappers around common networking and data-access tasks:

- Postgres connection pool creation and connection checkout.
- Redis key/value operations with namespacing and expiry support.
- HTTP requests (GET, POST, PATCH, PUT) with optional headers and JSON payloads.
- A shared error model to unify failure handling across all modules.

## Installation

Add this crate to your Cargo dependencies:

```toml
[dependencies]
network = { path = "." }
```

For a published version, replace the path dependency with the crate version.

## Prerequisites

- Rust toolchain compatible with edition 2024 (see Cargo.toml).
- A reachable Postgres instance for database usage.
- A reachable Redis instance for Redis usage.

## Module overview

- `database`: Diesel `r2d2` pool helpers for Postgres connections.
- `redis`: namespaced Redis get/set/delete operations.
- `http`: async REST wrapper around `reqwest::Client`.
- `errors`: shared error enums used by all modules.

## Quickstart

### Postgres pool

```rust
use network::database::DatabasePool;

fn create_pool() -> Result<DatabasePool, network::errors::NetworkError> {
	DatabasePool::new("postgres://user:password@localhost/app", 10)
}
```

### Redis client

```rust
use network::redis::RedisClient;

fn cache_example(connection: redis::Connection) -> Result<(), network::errors::NetworkError> {
	let mut client = RedisClient::new(connection, "app".to_string());
	client.insert("session-1", "active", Some(300))?;
	let _value = client.retrieve("session-1")?;
	client.delete("session-1")?;
	Ok(())
}
```

### HTTP client

```rust
use network::http::RestClient;
use serde::Serialize;

#[derive(Serialize)]
struct CreateItem {
	name: String,
}

async fn create_item() -> Result<(), network::errors::NetworkError> {
	let client = RestClient::new("https://api.example.com".to_string(), "network/0.1".to_string());
	let _response = client
		.post("/v1/items", None, CreateItem { name: "widget".to_string() })
		.await?;

	Ok(())
}
```

## Error handling

All operations return `Result<_, network::errors::NetworkError>`. The top-level error enum groups
database, Redis, transport, and input-validation failures.

## Development commands

Run these commands from the project root:

```bash
cargo fmt --check
cargo test
cargo doc --no-deps
```

## Additional docs

- `docs/architecture.md`: design boundaries and dependency choices.
- `docs/error-model.md`: error taxonomy and conversion pathways.
- `docs/usage.md`: practical examples for each module.
