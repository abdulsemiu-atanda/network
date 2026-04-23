//! Test-oriented helpers for database-backed specs and integration tests.
//!
//! Enable this module with the `spec` feature. The provided pool builder installs
//! a connection customizer that starts a Diesel test transaction whenever a
//! connection is acquired from the pool.

use diesel::r2d2::{CustomizeConnection, Pool, R2D2Connection as Connection};
use log::error;

use super::{database::DatabasePool, errors::NetworkError};

/// Connection customizer that starts a Diesel test transaction on checkout.
#[derive(Debug)]
struct TestConnectionCustomizer;

impl<C, E> CustomizeConnection<C, E> for TestConnectionCustomizer
where
  C: Connection,
  E: std::error::Error + Sync + Send,
{
  fn on_acquire(&self, conn: &mut C) -> Result<(), E> {
    if let Err(e) = conn.begin_test_transaction() {
      error!("Error beginning test transaction: {}", e);
    }
    Ok(())
  }
}

/// Builds a Postgres pool configured for transactional tests.
///
/// Each acquired connection attempts to begin a Diesel test transaction during
/// pool checkout. This keeps writes isolated for spec or integration test runs
/// that share a test database.
pub fn test_database_pool(url: &str, max_size: u32) -> Result<DatabasePool, NetworkError> {
  let builder = Pool::builder()
    .max_size(max_size)
    .connection_customizer(Box::new(TestConnectionCustomizer));

  DatabasePool::from_builder(url, builder)
}
