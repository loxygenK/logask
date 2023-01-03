use std::sync::Arc;

use sea_orm::Database;

use crate::{
    traits::{task::WithTaskRepository, Registry},
    InitializeError,
};

use self::{ctx::Context, task::DBTaskRepository};

pub mod ctx;
pub mod entity;
pub mod task;

pub struct DBRepositoryRegistry {
    task: DBTaskRepository,
}

impl DBRepositoryRegistry {
    pub async fn initialize(
        username: String,
        password: String,
        host: String,
        database: String,
    ) -> Result<Self, InitializeError> {
        let url = format!("postgres://{username}:{password}@{host}/{database}");
        let Ok(db) = Database::connect(url).await else {
            return Err(InitializeError::ConnectionFail);
        };

        let ctx = Arc::new(Context::new(db));

        Ok(Self {
            task: DBTaskRepository(ctx),
        })
    }
}

impl Registry for DBRepositoryRegistry {}

impl WithTaskRepository for DBRepositoryRegistry {
    type Repo = DBTaskRepository;

    fn get_task_repotiroy(&self) -> &Self::Repo {
        &self.task
    }
}
