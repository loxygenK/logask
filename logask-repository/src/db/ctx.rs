use std::sync::RwLock;

use sea_orm::DatabaseConnection;

pub(crate) struct Context {
    pub(crate) db: DatabaseConnection,
}

impl Context {
    pub(crate) fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
