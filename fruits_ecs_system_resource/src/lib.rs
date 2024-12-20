use std::{ops::{Deref, DerefMut}, sync::{Arc, Mutex, RwLock, RwLockWriteGuard}};

use fruits_utils::typed_map::{strategies::SendStrategy, TypedMap};

pub trait SystemResource : 'static + Send + Sync + Default { }

pub struct SystemResourcesHolder {
    data: Mutex<TypedMap<SendStrategy>>,
}

impl SystemResourcesHolder {
    pub fn new() -> Self {
        Self {
            data: Mutex::new(TypedMap::new()),
        }
    }

    pub fn get_or_create<S: SystemResource>(&self) -> Option<SystemResourcesHolderGuard<S>> {
        let data = &mut self.data.lock().unwrap();
        
        if !data.contains::<Arc<RwLock<S>>>() {
            data.insert(Arc::new(RwLock::new(S::default())));
        }

        let state = data.get_ref::<Arc<RwLock<S>>>().unwrap();

        SystemResourcesHolderGuard::new(Arc::clone(state))
    }
}

pub struct SystemResourcesHolderGuard<'a, S: SystemResource> {
    _guard: Arc<RwLock<S>>,
    _lock: RwLockWriteGuard<'a, S>,
}

impl<'a, S: SystemResource> SystemResourcesHolderGuard<'a, S> {
    fn new(guard: Arc<RwLock<S>>) -> Option<Self> {
        let ptr = &*guard as *const RwLock<S>;
        // safe because lock will live as long as the guard and guard will leave as long as the SystemStatesHolderGuard lives.
        let lock: RwLockWriteGuard<'_, S> = unsafe{ &*ptr }.try_write().ok()?;

        Some(Self {
            _guard: guard,
            _lock: lock,
        })
    }
}

impl<'a, S: SystemResource> Deref for SystemResourcesHolderGuard<'a, S> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self._lock
    }
}

impl<'a, S: SystemResource> DerefMut for SystemResourcesHolderGuard<'a, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self._lock
    }
}
