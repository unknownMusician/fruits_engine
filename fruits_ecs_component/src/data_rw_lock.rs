use std::{
    any::TypeId, collections::HashMap, sync::Mutex
};

use fruits_ecs_data_usage::{DataUsage, PerTypeDataUsage};

pub struct DataRwLock {
    state: Mutex<LockState>
}

impl DataRwLock {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(LockState::ByType(HashMap::new())),
        }
    }

    pub fn read(&self, type_id: TypeId) -> Option<DataRwLockReadGuard> {
        DataRwLockReadGuard::new(self, type_id)
    }

    pub fn write(&self, type_id: TypeId) -> Option<DataRwLockWriteGuard> {
        DataRwLockWriteGuard::new(self, type_id)
    }

    pub fn global(&self) -> Option<DataRwLockGlobalGuard> {
        DataRwLockGlobalGuard::new(self)
    }

    pub fn lock(&self, type_id: TypeId, is_mutable: bool) -> Option<DataRwLockGuard> {
        match is_mutable {
            true => self.write(type_id).map(|g| DataRwLockGuard::Write(g)),
            false => self.read(type_id).map(|g| DataRwLockGuard::Read(g)),
        }
    }

    pub fn lock_by_usage(&self, usage: &DataUsage) -> Option<Box<[DataRwLockGuard]>> {
        match usage {
            DataUsage::PerType(usage) => self.lock_by_type_usage(usage),
            DataUsage::GlobalMutable => self.global().map(|g| std::iter::once(DataRwLockGuard::Global(g)).collect::<Box<_>>()),
        }
    }

    pub fn lock_by_type_usage(&self, usage: &PerTypeDataUsage) -> Option<Box<[DataRwLockGuard]>> {
        let mut guards = Vec::new();

        for (&type_id, &is_mutable) in usage.values().iter() {
            guards.push(self.lock(type_id, is_mutable)?);
        }

        Some(guards.into_boxed_slice())
    }
}

enum DataRwLockEntryState {
    Read(usize),
    Write,
}

pub enum DataRwLockGuard<'a> {
    Read(DataRwLockReadGuard<'a>),
    Write(DataRwLockWriteGuard<'a>),
    Global(DataRwLockGlobalGuard<'a>),
}

enum LockState {
    ByType(HashMap<TypeId, DataRwLockEntryState>),
    Global,
}

pub struct DataRwLockReadGuard<'a> {
    type_id: TypeId,
    lock: &'a DataRwLock,
}

impl<'a> DataRwLockReadGuard<'a> {
    fn new(lock: &'a DataRwLock, type_id: TypeId) -> Option<Self> {
        let mut state = lock.state.lock().unwrap();

        let LockState::ByType(locks) = &mut *state else {
            return None;
        };

        match locks.get_mut(&type_id) {
            Some(lock_state) => {
                let DataRwLockEntryState::Read(locks_count) = lock_state else {
                    return None;
                };

                *locks_count += 1;
                
                Some(Self {
                    type_id,
                    lock,
                })
            },
            None => {
                locks.insert(type_id, DataRwLockEntryState::Read(1));

                Some(Self {
                    type_id,
                    lock,
                })
            },
        }
    }
}

impl<'a> Drop for DataRwLockReadGuard<'a> {
    fn drop(&mut self) {
        let mut state = self.lock.state.lock().unwrap();
        
        let LockState::ByType(locks) = &mut *state else {
            unreachable!();
        };

        let lock = locks.get_mut(&self.type_id).unwrap();

        let DataRwLockEntryState::Read(locks_count) = lock else { unreachable!() };

        if *locks_count == 1 {
            locks.remove(&self.type_id);
            return;
        }

        *locks_count -= 1;
    }
}

pub struct DataRwLockWriteGuard<'a> {
    type_id: TypeId,
    lock: &'a DataRwLock,
}

impl<'a> DataRwLockWriteGuard<'a> {
    fn new(lock: &'a DataRwLock, type_id: TypeId) -> Option<Self> {
        let mut state = lock.state.lock().unwrap();

        let LockState::ByType(locks) = &mut *state else {
            return None;
        };

        if locks.get_mut(&type_id).is_some() {
            return None;
        }

        locks.insert(type_id, DataRwLockEntryState::Write);

        Some(Self {
            type_id,
            lock,
        })
    }
}

impl<'a> Drop for DataRwLockWriteGuard<'a> {
    fn drop(&mut self) {
        let mut state = self.lock.state.lock().unwrap();
        
        let LockState::ByType(locks) = &mut *state else {
            unreachable!();
        };

        locks.remove(&self.type_id);
    }
}

pub struct DataRwLockGlobalGuard<'a> {
    lock: &'a DataRwLock,
}

impl<'a> DataRwLockGlobalGuard<'a> {
    fn new(lock: &'a DataRwLock) -> Option<Self> {
        let mut state = lock.state.lock().unwrap();

        let LockState::ByType(locks) = &*state else {
            return None;
        };

        if locks.len() != 0 {
            return None;
        }

        *state = LockState::Global;

        Some(Self {
            lock: &lock,
        })
    }
}

impl<'a> Drop for DataRwLockGlobalGuard<'a> {
    fn drop(&mut self) {
        let mut state = self.lock.state.lock().unwrap();
        *state = LockState::ByType(HashMap::new());
    }
}