use std::{
    any::{
        Any,
        TypeId
    },
    collections::HashMap,
    sync::{
        RwLock,
        RwLockReadGuard,
        RwLockWriteGuard
    }
};

pub trait Resource: 'static + Send + Sync {

}

pub struct WorldResources {
    resources: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl WorldResources {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    pub fn insert<R: Resource + Any>(&mut self, resource: R) {
        self.resources.insert(TypeId::of::<R>(), Box::new(RwLock::new(resource)));
    }

    pub fn get<R: Resource>(&self) -> Option<RwLockReadGuard<'_, R>> {
        self.resources.get(&TypeId::of::<R>())?.downcast_ref::<RwLock<R>>()?.try_read().ok()
    }

    pub fn get_mut<R: Resource>(&self) -> Option<RwLockWriteGuard<'_, R>> {
        self.resources.get(&TypeId::of::<R>())?.downcast_ref::<RwLock<R>>()?.try_write().ok()
    }
}