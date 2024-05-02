use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct TypedMap {
    data: HashMap<TypeId, Box<dyn Any>>,
}

impl TypedMap {
    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }

    pub fn insert<T: Any + 'static>(&mut self, v: T) {
        self.data.insert(TypeId::of::<T>(), Box::new(v));
    }

    pub fn get_ref<T: Any + 'static>(&self) -> Option<&T> {
        self.data
            .get(&TypeId::of::<T>())
            .map(|b| b.downcast_ref::<T>().unwrap())
    }

    pub fn get_mut<T: Any + 'static>(&mut self) -> Option<&mut T> {
        self.data
            .get_mut(&TypeId::of::<T>())
            .map(|b| b.downcast_mut::<T>().unwrap())
    }
}