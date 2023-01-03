use std::rc::Rc;

use logask_derives::WithId;

use crate::model::{id::Id, id_ref::IdRef, value::color::Color};

#[derive(WithId, Clone)]
pub struct Project {
    id: Id<Self>,
    name: String,
    color: Color,
    parent: Option<Rc<IdRef<Self>>>,
}
