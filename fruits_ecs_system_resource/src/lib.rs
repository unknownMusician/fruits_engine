use std::{ops::{Deref, DerefMut}, sync::{Arc, Mutex, RwLock, RwLockWriteGuard}};

use fruits_utils::typed_map::{strategies::SendStrategy, TypedMap};

pub trait SystemState : 'static + Send + Sync + Default { }

pub struct SystemStatesHolder {
    data: Mutex<TypedMap<SendStrategy>>,
}

impl SystemStatesHolder {
    pub fn new() -> Self {
        Self {
            data: Mutex::new(TypedMap::new()),
        }
    }

    pub fn get_or_create<S: SystemState>(&self) -> Option<SystemStatesHolderGuard<S>> {
        let data = &mut self.data.lock().unwrap();
        
        if !data.contains::<Arc<RwLock<S>>>() {
            data.insert(Arc::new(RwLock::new(S::default())));
        }

        let state = data.get_ref::<Arc<RwLock<S>>>().unwrap();

        SystemStatesHolderGuard::new(Arc::clone(state))
    }
}

pub struct SystemStatesHolderGuard<'a, S: SystemState> {
    _guard: Arc<RwLock<S>>,
    _lock: RwLockWriteGuard<'a, S>,
}

impl<'a, S: SystemState> SystemStatesHolderGuard<'a, S> {
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

impl<'a, S: SystemState> Deref for SystemStatesHolderGuard<'a, S> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self._lock
    }
}

impl<'a, S: SystemState> DerefMut for SystemStatesHolderGuard<'a, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self._lock
    }
}

// todo: remove (here just in case something goes wrong)
// impl SystemStatesHolder {
//     pub fn new() -> Self {
//         Self {
//             data: Mutex::new(TypedMap::new()),
//         }
//     }

//     pub fn get_or_create<S: SystemState>(&self) -> MutexGuard<S> {
//         let ptr = {
//             let data = &mut *self.data.lock().unwrap();
            
//             if data.get_ref::<Box<Mutex<S>>>().is_none() {
//                 data.insert(Box::new(Mutex::new(S::default())));
//             }

//             let s = data.get_ref::<Box<Mutex<S>>>().unwrap();

//             Box::as_ref(s) as *const Mutex<S>
//         };

//         let mutex = unsafe { &*ptr };

//         mutex.lock().unwrap()
//     }
// }

