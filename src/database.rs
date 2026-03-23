use diesel::prelude::*;
use diesel::r2d2::{Builder, ConnectionManager, Pool, PooledConnection, R2D2Connection as Connection};

use super::errors::NetworkError;

pub type DatabaseConnection = PooledConnection<ConnectionManager<PgConnection>>;

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
  pub fn new(url: &str, max_size: u32) -> Result<Self, NetworkError> {
    Self::from_builder(url, Pool::builder().max_size(max_size))
  }

  pub fn from_builder(url: &str, builder: Builder<ConnectionManager<T>>) -> Result<Self, NetworkError> {
    let manager = ConnectionManager::new(url);
    let pool = builder.build(manager)?;

    Ok(Self { pool })
  }

  pub fn connection(&self) -> Result<PooledConnection<ConnectionManager<T>>, NetworkError> {
    Ok(self.pool.get()?)
  }
}

pub type DatabasePool = DatabaseConnectionPool<PgConnection>;
