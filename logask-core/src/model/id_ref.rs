use crate::model::id::Id;

use super::id::WithId;

pub enum IdRef<T: WithId> {
    NotResolved(Id<T>),
    Resolved(T),
}

pub enum MakeResolvedError {
    AlreadyResolved,
    IdMismatch,
}

impl<T: WithId> Into<Id<T>> for IdRef<T> {
    fn into(self) -> Id<T> {
        match self {
            IdRef::NotResolved(id) => id,
            IdRef::Resolved(resolved) => resolved.take_id(),
        }
    }
}

impl<T: WithId> From<T> for IdRef<T> {
    fn from(value: T) -> Self {
        IdRef::Resolved(value)
    }
}

impl<T: WithId> From<Id<T>> for IdRef<T> {
    fn from(id: Id<T>) -> Self {
        IdRef::NotResolved(id)
    }
}

impl<T: WithId> IdRef<T> {
    pub fn unwrap(self) -> T {
        let IdRef::Resolved(resolved) = self else {
            panic!("unwrap() on unresolved reference!")
        };

        resolved
    }

    pub fn unwrap_id(self) -> Id<T> {
        let IdRef::NotResolved(id) = self else {
            panic!("unwrap_id() on unresolved reference!")
        };

        id
    }

    pub fn resolved(self) -> Option<T> {
        match self {
            Self::Resolved(resolved) => Some(resolved),
            Self::NotResolved(_) => None,
        }
    }

    pub fn id(self) -> Option<Id<T>> {
        match self {
            Self::Resolved(_) => None,
            Self::NotResolved(id) => Some(id),
        }
    }

    pub fn is_resolved(&self) -> bool {
        match self {
            Self::Resolved(_) => true,
            Self::NotResolved(_) => false,
        }
    }

    pub fn make_resolved(&mut self, resolved: T) -> Result<(), MakeResolvedError> {
        let Self::NotResolved(id) = self else {
            return Err(MakeResolvedError::AlreadyResolved);
        };

        if id != resolved.id() {
            return Err(MakeResolvedError::IdMismatch);
        }

        unsafe {
            self.make_resolved_unchecked(resolved);
        }

        Ok(())
    }

    pub unsafe fn make_resolved_unchecked(&mut self, resolved: T) {
        *self = Self::Resolved(resolved);
    }
}
