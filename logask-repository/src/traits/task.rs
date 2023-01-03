use logask_core::model::{entity::task::Task, id::Id};

pub trait TaskRepository {
    fn get(&self, id: &Id<Task>) -> Option<Task>;
    fn update(&mut self, id: &Task);
}

pub trait WithTaskRepository {
    type Repo: TaskRepository;

    fn get_task_repotiroy(&self) -> &Self::Repo;
}
