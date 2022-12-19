pub mod task;

use crate::traits::{task::WithTaskRepository, Registry};

use self::task::InMemoryTaskRepository;

pub struct InMemoryRepositoryRegistry {}

impl InMemoryRepositoryRegistry {
    pub fn new() -> Self {
        Self {}
    }
}

impl Registry for InMemoryRepositoryRegistry {}

impl WithTaskRepository for InMemoryRepositoryRegistry {
    type Repo = InMemoryTaskRepository;

    fn get_task_repotiroy(&self) -> &Self::Repo {
        todo!()
    }
}
