use sea_orm::ActiveValue::*;

use super::gen;
use logask_core::model::{entity::task::Task, id::Id};

impl gen::task::Model {
    pub(crate) fn into_task_with_children(self, children_id: Vec<Id<Task>>) -> Task {
        Task::new(
            Id::new(self.id.to_string()),
            self.name,
            self.description,
            self.project.to_string().into(),
            self.parent_task.map(|uuid| uuid.to_string().into()),
            children_id,
        )
    }
}

impl From<Task> for gen::task::Model {
    fn from(task: Task) -> Self {
        gen::task::Model {
            id: task.id().to_string().as_str().try_into().unwrap(),
            name: task.name.clone(),
            description: task.description.clone(),
            project: task.project.to_string().as_str().try_into().unwrap(),
            parent_task: task
                .parent
                .map(|id| id.to_string().as_str().try_into().unwrap()),
        }
    }
}
