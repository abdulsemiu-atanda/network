use redis::{Commands, Connection};

use super::errors::NetworkError;

const DEFAULT_EXPIRY_SECONDS: u64 = 600;

/// Namespaced Redis key/value client.
///
/// Keys are prefixed as `{namespace}:{key}` to reduce collisions between
/// different services or resource groups sharing the same Redis instance.
pub struct RedisClient {
  connection: Connection,
  namespace: String,
}

impl RedisClient {
  /// Creates a new namespaced Redis client.
  pub fn new(connection: Connection, namespace: String) -> Self {
    Self { connection, namespace }
  }

  /// Builds a namespaced Redis key.
  pub fn redis_key(&self, key: &str) -> String {
    format!("{}:{}", self.namespace, key)
  }

  /// Retrieves a key value if it exists.
  pub fn retrieve(&mut self, key: &str) -> Result<Option<String>, NetworkError> {
    Ok(self.connection.get(self.redis_key(key))?)
  }

  fn insert_with_expiry(&mut self, key: &str, value: &str, expiry: u64) -> Result<(), NetworkError> {
    if expiry == 0 {
      return Err(NetworkError::InvalidInput("expiry must be greater than zero"));
    }

    Ok(self.connection.set_ex(self.redis_key(key), value, expiry)?)
  }

  /// Inserts a value with either a custom expiry or a default of 600 seconds.
  pub fn insert(&mut self, key: &str, value: &str, expiry: Option<u64>) -> Result<(), NetworkError> {
    let expiry = expiry.unwrap_or(DEFAULT_EXPIRY_SECONDS);

    self.insert_with_expiry(key, value, expiry)
  }

  /// Deletes a key from Redis.
  pub fn delete(&mut self, key: &str) -> Result<(), NetworkError> {
    Ok(self.connection.del(self.redis_key(key))?)
  }
}
