use std::{
    any::Any,
    sync::{
        RwLock, RwLockReadGuard, RwLockWriteGuard
    }
};

use fruits_utils::typed_map::{strategies::ThreadedStrategy, TypedMap};

pub trait Resource: 'static + Send + Sync { }

pub struct ResourcesHolder {
    resources: TypedMap<ThreadedStrategy>,
}

impl ResourcesHolder {
    pub fn new() -> Self {
        Self {
            resources: TypedMap::new(),
        }
    }

    pub fn insert<R: Resource + Any>(&mut self, resource: R) {
        self.resources.insert(RwLock::new(resource));
    }

    pub fn get<R: Resource>(&self) -> Option<RwLockReadGuard<'_, R>> {
        self.resources.get_ref::<RwLock<R>>()?.try_read().ok()
    }

    pub fn get_mut<R: Resource>(&self) -> Option<RwLockWriteGuard<'_, R>> {
        self.resources.get_ref::<RwLock<R>>()?.try_write().ok()
    }
}