# network

Small Rust utilities for Postgres (Diesel), Redis, and HTTP client integrations.

## Why this crate exists

This crate provides lightweight wrappers around common networking and data-access tasks:

- Postgres connection pool creation and connection checkout.
- Redis key/value operations with namespacing and expiry support.
- HTTP requests (GET, POST, PATCH, PUT, DELETE) with optional headers and JSON payloads.
- A shared error model to unify failure handling across all modules.

## Installation

Add this crate to your Cargo dependencies:

```toml
[dependencies]
network = { path = ".", default-features = false, features = ["http"] }
```

For a published version, replace the path dependency with the crate version.

Available features:

- `http`: enables the `http` module and transport error conversions.
- `database`: enables the `database` module and Diesel-backed database errors.
- `keystore`: enables the `redis` module and Redis-backed cache errors.
- `spec`: enables the `spec_helpers` module for transactional database test setup. This feature also enables `database`.

The default feature set enables only `http`.

## Prerequisites

- Rust toolchain compatible with edition 2024 (see Cargo.toml).
- A reachable Postgres instance for database usage.
- A reachable Redis instance for Redis usage.

## Module overview

- `database`: Diesel `r2d2` pool helpers for Postgres connections. Requires the `database` feature.
- `redis`: namespaced Redis get/set/delete operations. Requires the `keystore` feature.
- `http`: async REST wrapper around `reqwest::Client`. Enabled by default through the `http` feature.
- `spec_helpers`: test-oriented database helpers that create pools whose checked out connections begin a test transaction automatically. Requires the `spec` feature.
- `errors`: shared error enums used by all modules.

## Quickstart

Enable the `database` feature before using the Postgres helpers:

```toml
[dependencies]
network = { path = ".", default-features = false, features = ["database"] }
```

### Postgres pool

```rust
use network::database::DatabasePool;

fn create_pool() -> Result<DatabasePool, network::errors::NetworkError> {
	DatabasePool::new("postgres://user:password@localhost/app", 10)
}
```

Enable the `spec` feature before using transactional database test helpers:

```toml
[dependencies]
network = { path = ".", default-features = false, features = ["spec"] }
```

### Spec helpers

```rust
use network::spec_helpers::test_database_pool;

fn create_test_pool() -> Result<network::database::DatabasePool, network::errors::NetworkError> {
	test_database_pool("postgres://user:password@localhost/app_test", 4)
}
```

Each acquired connection begins a Diesel test transaction automatically, so database state can be isolated per test.

Enable the `keystore` feature before using Redis helpers:

```toml
[dependencies]
network = { path = ".", default-features = false, features = ["keystore"] }
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
input-validation failures plus feature-specific database, Redis, and transport failures when those
integrations are enabled.

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
