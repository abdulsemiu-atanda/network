use diesel::prelude::*;
use diesel::r2d2::{Builder, ConnectionManager, Pool, PooledConnection, R2D2Connection as Connection};

use super::errors::NetworkError;

/// Pooled Postgres connection type.
pub type DatabaseConnection = PooledConnection<ConnectionManager<PgConnection>>;

/// Generic Diesel connection pool wrapper.
///
/// This abstraction stores an `r2d2` pool and converts pool-related failures into
/// `NetworkError` so callers can reuse a shared error type.
pub struct DatabaseConnectionPool<T>
where
  T: Connection + 'static,
{
  pub pool: Pool<ConnectionManager<T>>,
}

impl<T> Clone for DatabaseConnectionPool<T>
where
  T: Connection + 'static,
{
  fn clone(&self) -> Self {
    Self {
      pool: self.pool.to_owned(),
    }
  }
}

impl<T> DatabaseConnectionPool<T>
where
  T: Connection + 'static,
{
  /// Builds a pool using a maximum pool size.
  pub fn new(url: &str, max_size: u32) -> Result<Self, NetworkError> {
    Self::from_builder(url, Pool::builder().max_size(max_size))
  }

  /// Builds a pool with a caller-provided `r2d2` builder.
  pub fn from_builder(url: &str, builder: Builder<ConnectionManager<T>>) -> Result<Self, NetworkError> {
    let manager = ConnectionManager::new(url);
    let pool = builder.build(manager)?;

    Ok(Self { pool })
  }

  /// Retrieves a single pooled connection.
  pub fn connection(&self) -> Result<PooledConnection<ConnectionManager<T>>, NetworkError> {
    Ok(self.pool.get()?)
  }
}

/// Convenience alias for a Postgres (`PgConnection`) pool.
pub type DatabasePool = DatabaseConnectionPool<PgConnection>;
