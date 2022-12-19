use std::sync::Arc;

use logask_core::model::{entity::task::Task, id::Id};

use crate::traits::task::TaskRepository;

use super::ctx::Context;

pub struct DBTaskRepository {
    pub(crate) ctx: Arc<Context>,
}

impl TaskRepository for DBTaskRepository {
    fn get(id: &Id<Task>) -> Option<Task> {
        todo!()
    }

    fn update(task: &Task) {}
}
