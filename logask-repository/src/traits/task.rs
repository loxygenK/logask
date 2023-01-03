use logask_core::model::{entity::task::Task, id::Id};

use crate::error::{Created, Read, RepositoryResult, Update};

#[async_trait::async_trait]
pub trait TaskRepository {
    async fn create(&mut self, task: &Task) -> RepositoryResult<Created<Task>>;
    async fn get(&self, id: &Id<Task>) -> RepositoryResult<Read<Option<Task>>>;
    async fn update(&mut self, id: &Task) -> RepositoryResult<Update<Task>>;
}

pub trait WithTaskRepository {
    type Repo: TaskRepository;

    fn get_task_repotiroy(&self) -> &Self::Repo;
}
