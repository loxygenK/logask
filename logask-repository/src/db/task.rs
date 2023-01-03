use std::sync::Arc;

use logask_core::model::{entity::task::Task, id::Id};
use sea_orm::{prelude::Uuid, ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    error::{RepositoryError, RepositoryResult},
    traits::task::TaskRepository,
};

use super::{ctx::Context, entity::gen};

pub struct DBTaskRepository(pub(crate) Arc<Context>);

#[async_trait::async_trait]
impl TaskRepository for DBTaskRepository {
    async fn get(&self, id: &Id<Task>) -> RepositoryResult<Option<Task>> {
        let id = Uuid::parse_str(&id.to_string()).unwrap();

        let partial_task = gen::task::Entity::find_by_id(id).one(&self.0.db).await;

        let partial_task = match partial_task {
            Ok(Some(partial_task)) => partial_task,
            Ok(None) => return Ok(None),
            Err(e) => return Err(RepositoryError::from(e)),
        };

        let children_id: Vec<Id<Task>> = gen::task::Entity::find()
            .filter(gen::task::Column::ParentTask.eq(id))
            .all(&self.0.db)
            .await
            .map_err(RepositoryError::from)?
            .iter()
            .map(|child_task| child_task.id.to_string().into())
            .collect();

        Ok(Some(partial_task.into_task_with_children(children_id)))
    }

    async fn update(&mut self, task: &Task) -> RepositoryResult<()> {
        todo!()
    }
}
