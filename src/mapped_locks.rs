use std::{
    ops::{
        Deref,
        DerefMut
    },
    sync::{
        RwLockReadGuard,
        RwLockWriteGuard
    }
};

pub struct MappedRwLockReadGuard<'a, O: 'a, M: 'a> {
    _guard: RwLockReadGuard<'a, O>,
    mapped: M,
}

impl<'a, O: 'a, M: 'a> MappedRwLockReadGuard<'a, O, M> {
    pub fn map_from(guard: RwLockReadGuard<'a, O>, f: impl FnOnce(&'a O) -> M) -> Self {
        MappedRwLockReadGuard::map_from_flat(guard).map_into(f)
    }

    pub fn try_map_from(guard: RwLockReadGuard<'a, O>, f: impl FnOnce(&'a O) -> Option<M>) -> Option<Self> {
        MappedRwLockReadGuard::map_from_flat(guard).try_map_into(f)
    }

    pub fn map_into<T: 'a>(self, f: impl FnOnce(M) -> T) -> MappedRwLockReadGuard<'a, O, T> {
        MappedRwLockReadGuard {
            _guard: self._guard,
            mapped: f(self.mapped),
        }
    }

    pub fn try_map_into<T: 'a>(self, f: impl FnOnce(M) -> Option<T>) -> Option<MappedRwLockReadGuard<'a, O, T>> {
        Some(MappedRwLockReadGuard {
            _guard: self._guard,
            mapped: f(self.mapped)?,
        })
    }
}

impl<'a, O: 'a> MappedRwLockReadGuard<'a, O, &O> {
    pub fn map_from_flat(guard: RwLockReadGuard<'a, O>) -> Self {
        let ptr = &*guard as *const O;

        let mapped = unsafe { &*ptr };

        Self {
            _guard: guard,
            mapped,
        }
    }
}

impl<'a, O: 'a, M: 'a> Deref for MappedRwLockReadGuard<'a, O, M> {
    type Target = M;

    fn deref(&self) -> &Self::Target {
        &self.mapped
    }
}


impl<'a, O: 'a, M: 'a> DerefMut for MappedRwLockReadGuard<'a, O, M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mapped
    }
}

pub struct MappedRwLockWriteGuard<'a, O: 'a, M: 'a> {
    _guard: RwLockWriteGuard<'a, O>,
    mapped: M,
}

impl<'a, O: 'a, M: 'a> MappedRwLockWriteGuard<'a, O, M> {
    pub fn map_from(guard: RwLockWriteGuard<'a, O>, f: impl FnOnce(&'a mut O) -> M) -> Self {
        MappedRwLockWriteGuard::map_from_flat(guard).map_into(f)
    }

    pub fn try_map_from(guard: RwLockWriteGuard<'a, O>, f: impl FnOnce(&'a mut O) -> Option<M>) -> Option<Self> {
        MappedRwLockWriteGuard::map_from_flat(guard).try_map_into(f)
    }

    pub fn map_into<T: 'a>(self, f: impl FnOnce(M) -> T) -> MappedRwLockWriteGuard<'a, O, T> {
        MappedRwLockWriteGuard {
            _guard: self._guard,
            mapped: f(self.mapped),
        }
    }

    pub fn try_map_into<T: 'a>(self, f: impl FnOnce(M) -> Option<T>) -> Option<MappedRwLockWriteGuard<'a, O, T>> {
        Some(MappedRwLockWriteGuard {
            _guard: self._guard,
            mapped: f(self.mapped)?,
        })
    }
}

impl<'a, O: 'a> MappedRwLockWriteGuard<'a, O, &mut O> {
    pub fn map_from_flat(mut guard: RwLockWriteGuard<'a, O>) -> Self {
        let ptr = &mut *guard as *mut O;

        let reference = unsafe { &mut *ptr };

        Self {
            _guard: guard,
            mapped: reference,
        }
    }
}

impl<'a, O: 'a, M: 'a> Deref for MappedRwLockWriteGuard<'a, O, M> {
    type Target = M;

    fn deref(&self) -> &Self::Target {
        &self.mapped
    }
}

impl<'a, O: 'a, M: 'a> DerefMut for MappedRwLockWriteGuard<'a, O, M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mapped
    }
}