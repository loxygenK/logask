use logask_core::model::{entity::task::Task, id::Id};

#[async_trait::async_trait]
pub trait TaskRepository {
    async fn get(&self, id: &Id<Task>) -> Option<&Task>;
    async fn update(&mut self, id: &Task);
}

pub trait WithTaskRepository {
    type Repo: TaskRepository;

    fn get_task_repotiroy(&self) -> &Self::Repo;
}
