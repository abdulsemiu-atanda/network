use diesel::r2d2::{CustomizeConnection, Pool, R2D2Connection as Connection};
use log::error;

use super::{database::DatabasePool, errors::NetworkError};

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

pub fn test_database_pool(url: &str, max_size: u32) -> Result<DatabasePool, NetworkError> {
  let builder = Pool::builder()
    .max_size(max_size)
    .connection_customizer(Box::new(TestConnectionCustomizer));

  DatabasePool::from_builder(url, builder)
}
