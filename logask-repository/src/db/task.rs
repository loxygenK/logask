use std::{ops::Deref, sync::Arc};

use logask_core::model::{entity::task::Task, id::Id};
use sea_orm::{prelude::Uuid, EntityTrait};

use crate::traits::task::TaskRepository;

use super::{ctx::Context, entity::gen::task};

pub struct DBTaskRepository(pub(crate) Arc<Context>);

#[async_trait::async_trait]
impl TaskRepository for DBTaskRepository {
    async fn get(&self, id: &Id<Task>) -> Option<&Task> {
        task::Entity::find_by_id(Uuid::parse_str(&id.to_string()).unwrap())
            .one(self.0.db.read().unwrap().deref());

        todo!()
    }

    async fn update(&mut self, task: &Task) {}
}
