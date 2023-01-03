use self::task::WithTaskRepository;

pub mod task;

pub trait Registry: WithTaskRepository {}
