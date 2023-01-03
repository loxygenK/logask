use std::{collections::HashMap, time::Duration};

use logask_core::model::{
    entity::task::Task,
    id::{Id, WithId},
};

use crate::{
    error::{Created, Read, RepositoryResult, Update},
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
    async fn create(&mut self, task: &Task) -> RepositoryResult<Created<Task>> {
        tokio::time::sleep(Duration::from_millis(500)).await;

        self.0.insert(task.id().clone(), task.clone());

        Ok(Created(task.clone()))
    }

    async fn get(&self, id: &Id<Task>) -> RepositoryResult<Read<Option<Task>>> {
        tokio::time::sleep(Duration::from_millis(500)).await;

        Ok(Read(self.0.get(id).cloned()))
    }

    async fn update(&mut self, task: &Task) -> RepositoryResult<Update<Task>> {
        tokio::time::sleep(Duration::from_millis(500)).await;

        let exist_before = self.0.contains_key(&task.id());
        self.0.insert(task.id().clone(), task.clone());

        if exist_before {
            Ok(Update::Done(task.clone()))
        } else {
            Ok(Update::Created(task.clone()))
        }
    }
}
