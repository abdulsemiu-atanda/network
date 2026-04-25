#[cfg(feature = "keystore")]
use redis::{ParsingError, RedisError};

#[cfg(feature = "database")]
/// Database-related failures.
pub enum DatabaseError {
  Pool(diesel::r2d2::PoolError),
  Diesel(diesel::result::Error),
  Io(std::io::Error),
}

#[cfg(feature = "database")]
impl From<diesel::r2d2::PoolError> for DatabaseError {
  fn from(e: diesel::r2d2::PoolError) -> Self {
    DatabaseError::Pool(e)
  }
}

#[cfg(feature = "database")]
impl From<diesel::result::Error> for DatabaseError {
  fn from(e: diesel::result::Error) -> Self {
    DatabaseError::Diesel(e)
  }
}

#[cfg(feature = "database")]
impl From<std::io::Error> for DatabaseError {
  fn from(e: std::io::Error) -> Self {
    DatabaseError::Io(e)
  }
}

#[cfg(feature = "database")]
impl std::fmt::Debug for DatabaseError {
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

#[cfg(feature = "database")]
impl std::fmt::Display for DatabaseError {
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

#[cfg(feature = "keystore")]
/// Redis-related failures.
pub enum RedisClientError {
  Redis(RedisError),
  RedisParse(ParsingError),
}

#[cfg(feature = "keystore")]
impl From<RedisError> for RedisClientError {
  fn from(value: RedisError) -> Self {
    Self::Redis(value)
  }
}

#[cfg(feature = "keystore")]
impl From<ParsingError> for RedisClientError {
  fn from(value: ParsingError) -> Self {
    Self::RedisParse(value)
  }
}

#[cfg(feature = "keystore")]
impl std::fmt::Debug for RedisClientError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Redis(error) => write!(f, "{error}"),
      Self::RedisParse(error) => write!(f, "{error}"),
    }
  }
}

#[cfg(feature = "keystore")]
impl std::fmt::Display for RedisClientError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Redis(error) => write!(f, "{error}"),
      Self::RedisParse(error) => write!(f, "{error}"),
    }
  }
}

#[non_exhaustive]
#[derive(Debug)]
/// Top-level crate error used across database, Redis, and HTTP modules.
pub enum NetworkError {
  #[cfg(feature = "database")]
  Database(DatabaseError),
  #[cfg(feature = "keystore")]
  Redis(RedisClientError),
  #[cfg(feature = "http")]
  Transport(reqwest::Error),
  InvalidInput(&'static str),
}

#[cfg(feature = "database")]
impl From<DatabaseError> for NetworkError {
  fn from(value: DatabaseError) -> Self {
    Self::Database(value)
  }
}

#[cfg(feature = "database")]
impl From<diesel::r2d2::PoolError> for NetworkError {
  fn from(value: diesel::r2d2::PoolError) -> Self {
    Self::Database(DatabaseError::from(value))
  }
}

#[cfg(feature = "database")]
impl From<diesel::result::Error> for NetworkError {
  fn from(value: diesel::result::Error) -> Self {
    Self::Database(DatabaseError::from(value))
  }
}

#[cfg(feature = "keystore")]
impl From<RedisClientError> for NetworkError {
  fn from(value: RedisClientError) -> Self {
    Self::Redis(value)
  }
}

#[cfg(feature = "keystore")]
impl From<RedisError> for NetworkError {
  fn from(value: RedisError) -> Self {
    Self::Redis(RedisClientError::from(value))
  }
}

#[cfg(feature = "http")]
impl From<reqwest::Error> for NetworkError {
  fn from(value: reqwest::Error) -> Self {
    Self::Transport(value)
  }
}

impl std::fmt::Display for NetworkError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Transport(error) => write!(f, "{error}"),
      Self::InvalidInput(error) => write!(f, "{error}"),
      #[cfg(feature = "database")]
      Self::Database(error) => write!(f, "{error}"),
      #[cfg(feature = "keystore")]
      Self::Redis(error) => write!(f, "{error}"),
    }
  }
}
