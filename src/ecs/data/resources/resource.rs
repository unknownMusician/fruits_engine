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

trait StoredResource: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
}

impl dyn StoredResource {
    fn downcast<R: Resource>(&self) -> Option<&RwLock<R>> {
        self.as_any().downcast_ref::<RwLock<R>>()
    }
}

impl<R: Resource + 'static> StoredResource for RwLock<R> {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct WorldResources {
    resources: HashMap<TypeId, Box<dyn StoredResource>>,
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
        self.resources.get(&TypeId::of::<R>())?.downcast::<R>()?.try_read().ok()
    }

    pub fn get_mut<R: Resource>(&self) -> Option<RwLockWriteGuard<'_, R>> {
        self.resources.get(&TypeId::of::<R>())?.downcast::<R>()?.try_write().ok()
    }
}