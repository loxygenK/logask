use std::sync::RwLock;

use super::gen;
use logask_core::model::{entity::task::Task, id::Id, id_ref::IdRef};

impl gen::task::Model {
    fn from_with(self, children_id: Vec<Id<Task>>) -> Task {
        Task::new(
            Id::new(self.id.to_string()),
            self.name,
            self.description,
            self.project.to_string().into(),
            self.parent_task.map(Into::into),
            self.child,
        )
    }
}
