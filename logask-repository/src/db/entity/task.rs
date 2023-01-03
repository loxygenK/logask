use std::sync::RwLock;

use super::gen;
use logask_core::model::{entity::task::Task, id::Id, id_ref::IdRef};

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
