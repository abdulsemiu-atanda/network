# Usage Guide

Enable the `database` feature before using the database module:

```toml
[dependencies]
network = { path = ".", default-features = false, features = ["database"] }
```

## Database pool

```rust
use network::database::DatabasePool;
use network::errors::NetworkError;

fn create_pool() -> Result<DatabasePool, NetworkError> {
  DatabasePool::new("postgres://user:password@localhost/app", 10)
}

fn checkout_connection(pool: &DatabasePool) -> Result<(), NetworkError> {
  let _connection = pool.connection()?;
  Ok(())
}
```

If you need custom `r2d2` builder settings, use `DatabasePool::from_builder` with a caller-provided builder.

Enable the `spec` feature before using transactional database test helpers:

```toml
[dependencies]
network = { path = ".", default-features = false, features = ["spec"] }
```

## Spec helpers

```rust
use network::database::DatabasePool;
use network::errors::NetworkError;
use network::spec_helpers::test_database_pool;

fn create_test_pool() -> Result<DatabasePool, NetworkError> {
  test_database_pool("postgres://user:password@localhost/app_test", 4)
}
```

`test_database_pool` creates a Diesel pool whose checked out connections begin a test transaction on acquire. This is useful for integration or spec-style tests that should isolate database writes per test.

Enable the `keystore` feature before using Redis helpers:

```toml
[dependencies]
network = { path = ".", default-features = false, features = ["keystore"] }
```

## Redis operations

```rust
use network::errors::NetworkError;
use network::redis::RedisClient;

fn cache_value(connection: redis::Connection) -> Result<(), NetworkError> {
  let mut redis = RedisClient::new(connection, "app".to_string());

  redis.insert("session:42", "active", Some(300))?;
  let value = redis.retrieve("session:42")?;
  println!("cached value: {:?}", value);
  redis.delete("session:42")?;

  Ok(())
}
```

When `expiry` is `None`, Redis inserts default to a 600-second TTL.

## HTTP requests

The `http` feature is enabled by default. If you disable default features, re-enable it explicitly:

```toml
[dependencies]
network = { path = ".", default-features = false, features = ["http"] }
```

```rust
use network::errors::NetworkError;
use network::http::RestClient;
use serde::Serialize;

#[derive(Serialize)]
struct CreateItem {
  name: String,
}

async fn call_api() -> Result<(), NetworkError> {
  let client = RestClient::new(
    "https://api.example.com".to_string(),
    "network/0.1".to_string(),
  );

  let _create = client
    .post("/v1/items", None, CreateItem { name: "widget".to_string() })
    .await?;

  let _list = client.get("/v1/items?limit=10", None).await?;

  let _delete = client.delete("/v1/items/42", None).await?;

  Ok(())
}
```

## Error handling pattern

```rust
use network::errors::NetworkError;

fn map_error(error: NetworkError) -> &'static str {
  match error {
    NetworkError::InvalidInput(_) => "bad request",
    #[cfg(feature = "database")]
    NetworkError::Database(_) => "database failure",
    #[cfg(feature = "keystore")]
    NetworkError::Redis(_) => "cache failure",
    #[cfg(feature = "http")]
    NetworkError::Transport(_) => "http transport failure",
    _ => "unknown failure",
  }
}
```

The fallback arm is included because `NetworkError` is `#[non_exhaustive]`.
