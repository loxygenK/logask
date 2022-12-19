use std::sync::RwLock;

use sea_orm::DatabaseConnection;

pub(crate) struct Context {
    db: RwLock<DatabaseConnection>,
}

impl Context {
    pub(crate) fn new(db: DatabaseConnection) -> Self {
        Self {
            db: RwLock::new(db),
        }
    }
}
