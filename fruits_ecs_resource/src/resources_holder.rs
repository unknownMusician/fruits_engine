use std::{
    any::Any,
    sync::{
        RwLock, RwLockReadGuard, RwLockWriteGuard
    }
};

use fruits_utils::typed_map::{strategies::SendSyncStrategy, TypedMap};

use crate::resource::Resource;

pub struct ResourcesHolder {
    resources: TypedMap<SendSyncStrategy>,
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

    pub fn get<R: Resource>(&self) -> Option<RwLockReadGuard<R>> {
        self.resources.get_ref::<RwLock<R>>()?.try_read().ok()
    }

    pub fn get_mut<R: Resource>(&self) -> Option<RwLockWriteGuard<R>> {
        self.resources.get_ref::<RwLock<R>>()?.try_write().ok()
    }
}