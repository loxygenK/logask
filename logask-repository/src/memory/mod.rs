pub mod task;

use crate::traits::{task::WithTaskRepository, Registry};

use self::task::InMemoryTaskRepository;

pub struct InMemoryRepositoryRegistry {
    task_repo: InMemoryTaskRepository,
}

impl InMemoryRepositoryRegistry {
    pub fn new() -> Self {
        Self {
            task_repo: InMemoryTaskRepository::new(),
        }
    }
}

impl Default for InMemoryRepositoryRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl Registry for InMemoryRepositoryRegistry {}

impl WithTaskRepository for InMemoryRepositoryRegistry {
    type Repo = InMemoryTaskRepository;

    fn get_task_repotiroy(&self) -> &Self::Repo {
        &self.task_repo
    }
}
