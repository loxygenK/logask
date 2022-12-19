use std::rc::Rc;

use logask_derives::WithId;

use crate::model::{
    id::Id,
    id_ref::IdRef,
    tree::{Children, Parent},
};

use super::project::Project;

#[derive(WithId)]
pub struct Task {
    id: Id<Self>,
    name: String,
    description: String,
    project: Parent<IdRef<Project>>,
    parent: Parent<IdRef<Task>>,
    children: Children<IdRef<Task>>,
}

impl Task {
    pub fn new(
        id: Id<Self>,
        name: String,
        description: String,
        project: Parent<IdRef<Project>>,
        parent: Parent<IdRef<Task>>,
        children: Children<IdRef<Task>>,
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
}
