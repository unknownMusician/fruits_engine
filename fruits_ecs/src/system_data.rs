use std::sync::{Mutex, MutexGuard};

use fruits_utils::typed_map::{strategies::ThreadedStrategy, TypedMap};

pub trait SystemState : 'static + Send + Sync + Default { }

pub struct SystemStatesHolder {
    data: Mutex<TypedMap<ThreadedStrategy>>,
}

impl SystemStatesHolder {
    pub fn new() -> Self {
        Self {
            data: Mutex::new(TypedMap::new()),
        }
    }

    pub fn get_or_create<S: SystemState>(&self) -> MutexGuard<S> {
        let ptr = {
            let data = &mut *self.data.lock().unwrap();
            
            if data.get_ref::<Box<Mutex<S>>>().is_none() {
                data.insert(Box::new(Mutex::new(S::default())));
            }

            let s = data.get_ref::<Box<Mutex<S>>>().unwrap();

            Box::as_ref(s) as *const Mutex<S>
        };

        let mutex = unsafe { &*ptr };

        mutex.lock().unwrap()
    }
}
