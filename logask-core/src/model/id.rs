use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

#[derive(Clone)]
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

impl<T: WithId> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl<T: WithId> ToString for Id<T> {
    fn to_string(&self) -> String {
        self.id.clone()
    }
}

impl<T: WithId> From<&str> for Id<T> {
    fn from(value: &str) -> Self {
        Id::new(value.to_string())
    }
}

impl<T: WithId> From<String> for Id<T> {
    fn from(value: String) -> Self {
        Id::new(value)
    }
}

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
