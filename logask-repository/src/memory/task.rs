use crate::traits::task::TaskRepository;

pub struct InMemoryTaskRepository;

impl TaskRepository for InMemoryTaskRepository {
    fn get(
        id: &logask_core::model::id::Id<logask_core::model::entity::task::Task>,
    ) -> Option<logask_core::model::entity::task::Task> {
        todo!()
    }

    fn update(id: &logask_core::model::entity::task::Task) {
        todo!()
    }
}
