use std::{collections::HashMap, time::Duration};

use logask_core::model::{
    entity::task::Task,
    id::{Id, WithId},
};

use crate::{
    error::{RepositoryReport::*, RepositoryResult},
    traits::task::TaskRepository,
};

pub struct InMemoryTaskRepository(HashMap<Id<Task>, Task>);
impl InMemoryTaskRepository {
    pub(crate) fn new() -> Self {
        Self(
            (0..10)
                .map(|i| {
                    (
                        format!("task-{i}").into(),
                        Task::new(
                            format!("task-{i}").into(),
                            format!("In memory task #{i}"),
                            format!("This is very nice task number {i}"),
                            format!("project-{}", i / 3).into(),
                            None,
                            vec![],
                        ),
                    )
                })
                .collect(),
        )
    }
}

#[async_trait::async_trait]
impl TaskRepository for InMemoryTaskRepository {
    async fn create(&mut self, task: &Task) -> RepositoryResult<Task> {
        tokio::time::sleep(Duration::from_millis(500)).await;

        self.0.insert(task.id().clone(), task.clone());

        Ok(Created(task.clone()))
    }

    async fn get(&self, id: &Id<Task>) -> RepositoryResult<Option<Task>> {
        tokio::time::sleep(Duration::from_millis(500)).await;

        Ok(Read(self.0.get(id).cloned()))
    }

    async fn update(&mut self, task: &Task) -> RepositoryResult<()> {
        tokio::time::sleep(Duration::from_millis(500)).await;

        self.0.insert(task.id().clone(), task.clone());

        Ok(Updated)
    }
}
