use std::{
    ops::{
        Deref,
        DerefMut,
    },
    sync::{
        MutexGuard,
        RwLockReadGuard,
        RwLockWriteGuard,
    }
};

trait Sealed { }
impl Sealed for RwLockReadGuarding { }
impl Sealed for RwLockWriteGuarding { }
impl Sealed for MutexGuarding { }
// todo: implement MappableGuarding for MappedGuard

//

#[allow(private_bounds)]
pub trait MappableGuarding : Sealed + 'static {
    type Guard<'o, Origin: 'o>;

    fn get_ref<'a, 'o: 'a, T: 'o>(guard: &'a Self::Guard<'o, T>) -> &'o T;
}

pub trait MutMappableGuarding : MappableGuarding {
    fn get_mut<'a, 'o: 'a, T: 'o>(guard: &'a mut Self::Guard<'o, T>) -> &'o mut T;
}

//

pub struct RwLockReadGuarding;
impl MappableGuarding for RwLockReadGuarding {
    type Guard<'o, Origin: 'o> = RwLockReadGuard<'o, Origin>;
    
    fn get_ref<'a, 'o: 'a, T: 'o>(guard: &'a Self::Guard<'o, T>) -> &'o T {
        unsafe { &*(guard as *const Self::Guard<'o, T>) }
    }
}

pub struct RwLockWriteGuarding;
impl MappableGuarding for RwLockWriteGuarding {
    type Guard<'o, Origin: 'o> = RwLockWriteGuard<'o, Origin>;
    
    fn get_ref<'a, 'o: 'a, T: 'o>(guard: &'a Self::Guard<'o, T>) -> &'o T {
        unsafe { &*(guard as *const Self::Guard<'o, T>) }
    }
}

impl MutMappableGuarding for RwLockWriteGuarding {
    fn get_mut<'a, 'o: 'a, T: 'o>(guard: &'a mut Self::Guard<'o, T>) -> &'o mut T {
        unsafe { &mut *(guard as *mut Self::Guard<'o, T>) }
    }
}

pub struct MutexGuarding;
impl MappableGuarding for MutexGuarding {
    type Guard<'a, Origin: 'a> = MutexGuard<'a, Origin>;

    fn get_ref<'a, 'o: 'a, T: 'o>(guard: &'a Self::Guard<'o, T>) -> &'o T {
        unsafe { &*(guard as *const Self::Guard<'o, T>) }
    }
}

impl MutMappableGuarding for MutexGuarding {
    fn get_mut<'a, 'o: 'a, T: 'o>(guard: &'a mut Self::Guard<'o, T>) -> &'o mut T {
        unsafe { &mut *(guard as *mut Self::Guard<'o, T>) }
    }
}

//

pub struct MappedGuard<'o, Guarding: MappableGuarding, Origin: 'o, Mapped> {
    _guard: Guarding::Guard<'o, Origin>,
    mapped: Mapped,
}

impl<'o, Guarding: MappableGuarding, Origin: 'o> MappedGuard<'o, Guarding, Origin, &'o Origin> {
    pub fn map_from_flat_ref(guard: Guarding::Guard<'o, Origin>) -> Self {
        let guard_ref: &'o Origin = Guarding::get_ref(&guard);
        // let guard_ptr = &guard as *const Guarding::Guard<'o, Origin>;
        // let ptr = Guarding::get_ref(unsafe { &*guard_ptr }) as *const Origin;

        Self {
            _guard: guard,
            // mapped: unsafe { &*ptr },
            mapped: guard_ref,
        }
    }
}

impl<'o, Guarding: MutMappableGuarding, Origin: 'o> MappedGuard<'o, Guarding, Origin, &'o mut Origin> {
    pub fn map_from_flat_mut(mut guard: Guarding::Guard<'o, Origin>) -> Self {
        let guard_mut = Guarding::get_mut(&mut guard);
        // let guard_ptr = &mut guard as *mut Guarding::Guard<'a, Origin>;
        // let ptr = Guarding::get_mut(unsafe { &mut *guard_ptr }) as *mut Origin;

        Self {
            _guard: guard,
            // mapped: unsafe { &mut *ptr },
            mapped: guard_mut,
        }
    }
}

impl<'o, Guarding: MappableGuarding, Origin: 'o, Mapped> MappedGuard<'o, Guarding, Origin, Mapped> {
    pub fn map_from(guard: Guarding::Guard<'o, Origin>, f: impl FnOnce(&'o Origin) -> Mapped) -> Self {
        MappedGuard::<'o, Guarding, Origin, &Origin>::map_from_flat_ref(guard).map_into(f)
    }

    pub fn try_map_from(guard: Guarding::Guard<'o, Origin>, f: impl FnOnce(&'o Origin) -> Option<Mapped>) -> Option<Self> {
        MappedGuard::<'o, Guarding, Origin, &Origin>::map_from_flat_ref(guard).try_map_into(f)
    }

    pub fn map_into<T>(self, f: impl FnOnce(Mapped) -> T) -> MappedGuard<'o, Guarding, Origin, T>
    {
        MappedGuard {
            _guard: self._guard,
            mapped: f(self.mapped),
        }
    }

    pub fn try_map_into<T>(self, f: impl FnOnce(Mapped) -> Option<T>) -> Option<MappedGuard<'o, Guarding, Origin, T>> {
        Some(MappedGuard {
            _guard: self._guard,
            mapped: f(self.mapped)?,
        })
    }
}

impl<'o, Guarding: MutMappableGuarding, Origin: 'o, Mapped> MappedGuard<'o, Guarding, Origin, Mapped> {
    pub fn map_from_mut(guard: Guarding::Guard<'o, Origin>, f: impl FnOnce(&'o mut Origin) -> Mapped) -> Self {
        MappedGuard::<'o, Guarding, Origin, &mut Origin>::map_from_flat_mut(guard).map_into(f)
    }

    pub fn try_map_from_mut(guard: Guarding::Guard<'o, Origin>, f: impl FnOnce(&'o mut Origin) -> Option<Mapped>) -> Option<Self> {
        MappedGuard::<'o, Guarding, Origin, &mut Origin>::map_from_flat_mut(guard).try_map_into(f)
    }
}

impl<'o, Guarding: MappableGuarding, Origin: 'o, Mapped> Deref for MappedGuard<'o, Guarding, Origin, Mapped> {
    type Target = Mapped;

    fn deref(&self) -> &Self::Target {
        &self.mapped
    }
}

impl<'o, Guarding: MappableGuarding, Origin: 'o, Mapped> DerefMut for MappedGuard<'o, Guarding, Origin, Mapped> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mapped
    }
}