use std::{
    any::TypeId, collections::HashMap, sync::Mutex
};

use crate::ecs::behavior::{WorldSharedDataUsage, WorldSharedPerTypeDataUsage};

impl DataRwLocker {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(LockState::ByType(HashMap::new())),
        }
    }

    pub fn read(&self, type_id: TypeId) -> Option<DataRwLockerReadGuard> {
        DataRwLockerReadGuard::new(self, type_id)
    }

    pub fn write(&self, type_id: TypeId) -> Option<DataRwLockerWriteGuard> {
        DataRwLockerWriteGuard::new(self, type_id)
    }

    pub fn global(&self) -> Option<DataRwLockerGlobalGuard> {
        DataRwLockerGlobalGuard::new(self)
    }

    pub fn lock(&self, type_id: TypeId, is_mutable: bool) -> Option<DataRwLockerGuard> {
        match is_mutable {
            true => self.write(type_id).map(|g| DataRwLockerGuard::Write(g)),
            false => self.read(type_id).map(|g| DataRwLockerGuard::Read(g)),
        }
    }

    pub fn lock_by_usage(&self, usage: &WorldSharedDataUsage) -> Option<Box<[DataRwLockerGuard]>> {
        match usage {
            WorldSharedDataUsage::PerType(usage) => self.lock_by_type_usage(usage),
            WorldSharedDataUsage::GlobalMutable => self.global().map(|g| std::iter::once(DataRwLockerGuard::Global(g)).collect::<Box<_>>()),
        }
    }

    pub fn lock_by_type_usage(&self, usage: &WorldSharedPerTypeDataUsage) -> Option<Box<[DataRwLockerGuard]>> {
        let mut guards = Vec::new();

        for (&type_id, &is_mutable) in usage.values().iter() {
            guards.push(self.lock(type_id, is_mutable)?);
        }

        Some(guards.into_boxed_slice())
    }
}

enum DataRwLock {
    Read(usize),
    Write,
}

pub enum DataRwLockerGuard<'a> {
    Read(DataRwLockerReadGuard<'a>),
    Write(DataRwLockerWriteGuard<'a>),
    Global(DataRwLockerGlobalGuard<'a>),
}

pub enum LockState {
    ByType(HashMap<TypeId, DataRwLock>),
    Global,
}

pub struct DataRwLocker {
    state: Mutex<LockState>
}

pub struct DataRwLockerReadGuard<'a> {
    type_id: TypeId,
    locker: &'a DataRwLocker,
}

impl<'a> DataRwLockerReadGuard<'a> {
    fn new(locker: &'a DataRwLocker, type_id: TypeId) -> Option<Self> {
        let mut state = locker.state.lock().unwrap();

        let LockState::ByType(locks) = &mut *state else {
            return None;
        };

        match locks.get_mut(&type_id) {
            Some(lock) => {
                let DataRwLock::Read(locks_count) = lock else {
                    return None;
                };

                *locks_count += 1;
                
                Some(Self {
                    type_id,
                    locker: &locker,
                })
            },
            None => {
                locks.insert(type_id, DataRwLock::Read(1));

                Some(Self {
                    type_id,
                    locker: &locker,
                })
            },
        }
    }
}

impl<'a> Drop for DataRwLockerReadGuard<'a> {
    fn drop(&mut self) {
        let mut state = self.locker.state.lock().unwrap();
        
        let LockState::ByType(locks) = &mut *state else {
            unreachable!();
        };

        let lock = locks.get_mut(&self.type_id).unwrap();

        let DataRwLock::Read(locks_count) = lock else { unreachable!() };

        if *locks_count == 1 {
            locks.remove(&self.type_id);
            return;
        }

        *locks_count -= 1;
    }
}

pub struct DataRwLockerWriteGuard<'a> {
    type_id: TypeId,
    locker: &'a DataRwLocker,
}

impl<'a> DataRwLockerWriteGuard<'a> {
    fn new(locker: &'a DataRwLocker, type_id: TypeId) -> Option<Self> {
        let mut state = locker.state.lock().unwrap();

        let LockState::ByType(locks) = &mut *state else {
            return None;
        };

        if locks.get_mut(&type_id).is_some() {
            return None;
        }

        locks.insert(type_id, DataRwLock::Write);

        Some(Self {
            type_id,
            locker: &locker,
        })
    }
}

impl<'a> Drop for DataRwLockerWriteGuard<'a> {
    fn drop(&mut self) {
        let mut state = self.locker.state.lock().unwrap();
        
        let LockState::ByType(locks) = &mut *state else {
            unreachable!();
        };

        locks.remove(&self.type_id);
    }
}

pub struct DataRwLockerGlobalGuard<'a> {
    locker: &'a DataRwLocker,
}

impl<'a> DataRwLockerGlobalGuard<'a> {
    fn new(locker: &'a DataRwLocker) -> Option<Self> {
        let mut state = locker.state.lock().unwrap();

        let LockState::ByType(locks) = &*state else {
            return None;
        };

        if locks.len() != 0 {
            return None;
        }

        *state = LockState::Global;

        Some(Self {
            locker: &locker,
        })
    }
}

impl<'a> Drop for DataRwLockerGlobalGuard<'a> {
    fn drop(&mut self) {
        let mut state = self.locker.state.lock().unwrap();
        *state = LockState::ByType(HashMap::new());
    }
}