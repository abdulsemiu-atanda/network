use redis::{Commands, Connection};

use super::errors::NetworkError;

const DEFAULT_EXPIRY_SECONDS: u64 = 600;

pub struct RedisClient {
  connection: Connection,
  namespace: String,
}

impl RedisClient {
  pub fn new(connection: Connection, namespace: String) -> Self {
    Self { connection, namespace }
  }

  pub fn redis_key(&self, key: &str) -> String {
    format!("{}:{}", self.namespace, key)
  }

  pub fn retrieve(&mut self, key: &str) -> Result<Option<String>, NetworkError> {
    Ok(self.connection.get(self.redis_key(key))?)
  }

  fn insert_with_expiry(&mut self, key: &str, value: &str, expiry: u64) -> Result<(), NetworkError> {
    if expiry == 0 {
      return Err(NetworkError::InvalidInput("expiry must be greater than zero"));
    }

    Ok(self.connection.set_ex(self.redis_key(key), value, expiry)?)
  }

  pub fn insert(&mut self, key: &str, value: &str, expiry: Option<u64>) -> Result<(), NetworkError> {
    let expiry = expiry.unwrap_or(DEFAULT_EXPIRY_SECONDS);

    self.insert_with_expiry(key, value, expiry)
  }

  pub fn delete(&mut self, key: &str) -> Result<(), NetworkError> {
    Ok(self.connection.del(self.redis_key(key))?)
  }
}
