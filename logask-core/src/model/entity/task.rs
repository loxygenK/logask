use std::rc::Rc;

use logask_derives::WithId;

use crate::model::{
    id::Id,
    id_ref::IdRef,
    tree::{Children, Parent},
};

use super::project::Project;

#[derive(WithId, Clone)]
pub struct Task {
    id: Id<Self>,
    pub name: String,
    pub description: String,
    pub project: Id<Project>,
    pub parent: Option<Id<Task>>,
    pub children: Vec<Id<Task>>,
}

impl Task {
    pub fn new(
        id: Id<Self>,
        name: String,
        description: String,
        project: Id<Project>,
        parent: Option<Id<Task>>,
        children: Vec<Id<Task>>,
    ) -> Self {
        Self {
            id,
            name,
            description,
            project,
            parent,
            children,
        }
    }

    pub fn id(&self) -> &Id<Self> {
        &self.id
    }
}
