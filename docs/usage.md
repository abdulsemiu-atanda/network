# Usage Guide

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

## HTTP requests

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
    "network-client/0.1".to_string(),
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
    NetworkError::Database(_) => "database failure",
    NetworkError::Redis(_) => "cache failure",
    NetworkError::Transport(_) => "http transport failure",
    _ => "unknown failure",
  }
}
```

The fallback arm is included because `NetworkError` is `#[non_exhaustive]`.
