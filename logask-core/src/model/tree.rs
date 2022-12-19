use std::{
    rc::Weak,
    sync::{Arc, RwLock},
};

pub type Parent<T> = RwLock<Weak<T>>;
pub type Child<T> = RwLock<Arc<T>>;
pub type Children<T> = RwLock<Vec<Arc<T>>>;
