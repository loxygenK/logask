use std::marker::PhantomData;

pub struct Id<T: ?Sized + WithId> {
    id: String,
    _phantom: PhantomData<fn() -> T>,
}

impl<T: WithId> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl<T: WithId> Eq for Id<T> {}

impl<T: WithId> Id<T> {
    pub fn new(id: String) -> Self {
        Self {
            id,
            _phantom: PhantomData,
        }
    }
}

pub trait WithId {
    fn id(&self) -> &Id<Self>;
    fn take_id(self) -> Id<Self>;
}

pub trait IdGenerator {
    fn next<T: WithId>(&mut self) -> Id<T>;
}
