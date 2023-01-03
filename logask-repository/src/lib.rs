#[cfg(feature = "db")]
use db::DBRepositoryRegistry;

#[cfg(feature = "in-memory")]
use memory::InMemoryRepositoryRegistry;

use traits::Registry;

pub mod db;
pub mod error;
pub mod memory;
pub mod traits;

#[cfg(not(any(feature = "db", feature = "in-memory")))]
compile_error!("Either of features `db` or `in-memory` should be specified");

#[cfg(all(feature = "db", feature = "in-memory"))]
compile_error!("Features `db` or `in-memory` cannot be specified at once");

pub enum InitializeError {
    ConnectionFail,
}

#[cfg(feature = "db")]
pub async fn initialize(
    username: String,
    password: String,
    host: String,
    database: String,
) -> Result<impl Registry, InitializeError> {
    DBRepositoryRegistry::initialize(username, password, host, database).await
}

#[cfg(feature = "in-memory")]
pub async fn initialize() -> impl Future<Output = Result<impl Registry, InitializeError>> {
    InMemoryRepositoryRegistry::new()
}
