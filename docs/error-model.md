# Error Model

## Goal

Provide a single top-level error type (`NetworkError`) for operations that touch database, cache, and HTTP layers.
Feature-specific variants are compiled only when their integration is enabled.

## Error hierarchy

- `NetworkError::Database(DatabaseError)` when the `database` feature is enabled
  - `DatabaseError::Pool`
  - `DatabaseError::Diesel`
  - `DatabaseError::Io`
- `NetworkError::Redis(RedisClientError)` when the `keystore` feature is enabled
  - `RedisClientError::Redis`
  - `RedisClientError::RedisParse`
- `NetworkError::Transport(reqwest::Error)` when the `http` feature is enabled
- `NetworkError::InvalidInput(&'static str)`

## Conversion pathways

The crate implements `From` conversions so callers can use `?` with underlying errors when the
matching feature is enabled:

- Diesel pool errors -> `DatabaseError` -> `NetworkError`
- Diesel query errors -> `DatabaseError` -> `NetworkError`
- Redis errors -> `RedisClientError` -> `NetworkError`
- Reqwest errors -> `NetworkError::Transport`

## Validation errors

Input validation is represented with `NetworkError::InvalidInput`. For example, Redis insert rejects `expiry == 0`.

## Forward compatibility

`NetworkError` is marked `#[non_exhaustive]`, so downstream `match` statements should include a fallback arm (`_ => ...`) to remain compatible with future variants.
