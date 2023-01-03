use logask_core::model::{entity::task::Task, id::Id};

use crate::error::RepositoryResult;

#[async_trait::async_trait]
pub trait TaskRepository {
    async fn create(&mut self, task: &Task) -> RepositoryResult<Task>;
    async fn get(&self, id: &Id<Task>) -> RepositoryResult<Option<Task>>;
    async fn update(&mut self, id: &Task) -> RepositoryResult<Task>;
}

pub trait WithTaskRepository {
    type Repo: TaskRepository;

    fn get_task_repotiroy(&self) -> &Self::Repo;
}
