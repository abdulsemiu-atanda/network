use redis::{ParsingError, RedisError};
use std::fmt;

pub enum DatabaseError {
  Pool(diesel::r2d2::PoolError),
  Diesel(diesel::result::Error),
  Io(std::io::Error),
}

impl From<diesel::r2d2::PoolError> for DatabaseError {
  fn from(e: diesel::r2d2::PoolError) -> Self {
    DatabaseError::Pool(e)
  }
}

impl From<diesel::result::Error> for DatabaseError {
  fn from(e: diesel::result::Error) -> Self {
    DatabaseError::Diesel(e)
  }
}

impl From<std::io::Error> for DatabaseError {
  fn from(e: std::io::Error) -> Self {
    DatabaseError::Io(e)
  }
}

impl fmt::Debug for DatabaseError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      DatabaseError::Pool(error) => {
        write!(f, "{error}")
      }
      DatabaseError::Diesel(error) => {
        write!(f, "{error}")
      }
      DatabaseError::Io(error) => {
        write!(f, "{error}")
      }
    }
  }
}

impl fmt::Display for DatabaseError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      DatabaseError::Pool(error) => {
        write!(f, "{error}")
      }
      DatabaseError::Diesel(error) => {
        write!(f, "{error}")
      }
      DatabaseError::Io(error) => {
        write!(f, "{error}")
      }
    }
  }
}

/// Redis Errors
pub enum RedisClientError {
  Redis(RedisError),
  RedisParse(ParsingError),
}

impl From<RedisError> for RedisClientError {
  fn from(value: RedisError) -> Self {
    Self::Redis(value)
  }
}

impl From<ParsingError> for RedisClientError {
  fn from(value: ParsingError) -> Self {
    Self::RedisParse(value)
  }
}

impl fmt::Debug for RedisClientError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Redis(error) => write!(f, "{error}"),
      Self::RedisParse(error) => write!(f, "{error}"),
    }
  }
}

impl fmt::Display for RedisClientError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Redis(error) => write!(f, "{error}"),
      Self::RedisParse(error) => write!(f, "{error}"),
    }
  }
}

#[non_exhaustive]
#[derive(Debug)]
pub enum NetworkError {
  Database(DatabaseError),
  Redis(RedisClientError),
  Transport(reqwest::Error),
  InvalidInput(&'static str),
}

impl From<DatabaseError> for NetworkError {
  fn from(value: DatabaseError) -> Self {
    Self::Database(value)
  }
}

impl From<RedisClientError> for NetworkError {
  fn from(value: RedisClientError) -> Self {
    Self::Redis(value)
  }
}

impl From<RedisError> for NetworkError {
  fn from(value: RedisError) -> Self {
    Self::Redis(RedisClientError::from(value))
  }
}

impl From<reqwest::Error> for NetworkError {
  fn from(value: reqwest::Error) -> Self {
    Self::Transport(value)
  }
}
