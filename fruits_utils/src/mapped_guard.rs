use std::{
    cell::{Ref, RefMut}, marker::PhantomData, ops::{
        Deref,
        DerefMut,
    }, sync::{
        Arc, MutexGuard, RwLockReadGuard, RwLockWriteGuard
    }
};

trait Sealed { }
impl Sealed for RwLockReadGuarding { }
impl Sealed for RwLockWriteGuarding { }
impl Sealed for MutexGuarding { }
impl Sealed for RefCellReadGuarding { }
impl Sealed for RefCellWriteGuarding { }
impl Sealed for ArcGuarding { }
// todo: implement MappableGuarding for MappedGuard
// impl<'oo, Guarding: MappableGuarding, HiddenOrigin: 'oo> Sealed for RecursiveGuarding<'oo, Guarding, HiddenOrigin> { }

//

#[allow(private_bounds)]
pub trait MappableGuarding : Sealed {
    type Guard<'o, Origin: 'o>;

    // unsafe because allows multiple different references from a single guard.
    unsafe fn get_ref<'o, T: 'o>(guard: &Self::Guard<'o, T>) -> &'o T;
}

pub trait MutMappableGuarding : MappableGuarding {
    // unsafe because allows multiple different references from a single guard.
    unsafe fn get_mut<'o, T: 'o>(guard: &mut Self::Guard<'o, T>) -> &'o mut T;
}

//

pub struct RwLockReadGuarding;
impl MappableGuarding for RwLockReadGuarding {
    type Guard<'o, Origin: 'o> = RwLockReadGuard<'o, Origin>;
    
    unsafe fn get_ref<'o, T: 'o>(guard: &Self::Guard<'o, T>) -> &'o T {
        &*(guard as *const Self::Guard<'o, T>)
    }
}

pub struct RwLockWriteGuarding;
impl MappableGuarding for RwLockWriteGuarding {
    type Guard<'o, Origin: 'o> = RwLockWriteGuard<'o, Origin>;
    
    unsafe fn get_ref<'o, T: 'o>(guard: &Self::Guard<'o, T>) -> &'o T {
        &*(guard as *const Self::Guard<'o, T>)
    }
}

impl MutMappableGuarding for RwLockWriteGuarding {
    unsafe fn get_mut<'o, T: 'o>(guard: &mut Self::Guard<'o, T>) -> &'o mut T {
        &mut *(guard as *mut Self::Guard<'o, T>)
    }
}

pub struct MutexGuarding;
impl MappableGuarding for MutexGuarding {
    type Guard<'o, Origin: 'o> = MutexGuard<'o, Origin>;

    unsafe fn get_ref<'o, T: 'o>(guard: &Self::Guard<'o, T>) -> &'o T {
        &*(guard as *const Self::Guard<'o, T>)
    }
}

impl MutMappableGuarding for MutexGuarding {
    unsafe fn get_mut<'o, T: 'o>(guard: &mut Self::Guard<'o, T>) -> &'o mut T {
        &mut *(guard as *mut Self::Guard<'o, T>)
    }
}

pub struct RefCellReadGuarding;
impl MappableGuarding for RefCellReadGuarding {
    type Guard<'o, Origin: 'o> = Ref<'o, Origin>;

    unsafe fn get_ref<'o, T: 'o>(guard: &Self::Guard<'o, T>) -> &'o T {
        &*(guard as *const Self::Guard<'o, T>)
    }
}

pub struct RefCellWriteGuarding;
impl MappableGuarding for RefCellWriteGuarding {
    type Guard<'o, Origin: 'o> = RefMut<'o, Origin>;

    unsafe fn get_ref<'o, T: 'o>(guard: &Self::Guard<'o, T>) -> &'o T {
        &*(guard as *const Self::Guard<'o, T>)
    }
}

impl MutMappableGuarding for RefCellWriteGuarding {
    unsafe fn get_mut<'o, T: 'o>(guard: &mut Self::Guard<'o, T>) -> &'o mut T {
        &mut *(guard as *mut Self::Guard<'o, T>)
    }
}

pub struct ArcGuarding;
impl MappableGuarding for ArcGuarding {
    type Guard<'o, Origin: 'o> = Arc<Origin>;

    unsafe fn get_ref<'o, T: 'o>(guard: &Self::Guard<'o, T>) -> &'o T {
        &*(guard as *const Self::Guard<'o, T>)
    }
}

// #[derive(Default)]
// pub struct RecursiveGuarding<'o, Guarding: MappableGuarding, Origin: 'o>{
//     _phantom: PhantomData<(Guarding, &'o Origin)>,
// }

// impl<'oo, Guarding: MappableGuarding, HiddenOrigin: 'oo> MappableGuarding for RecursiveGuarding<'oo, Guarding, HiddenOrigin> {
//     type Guard<'o, Origin: 'o> = MappedGuard<'oo, Guarding, HiddenOrigin, Origin>;

//     unsafe fn get_ref<'o, T: 'o>(guard: &Self::Guard<'o, T>) -> &'o T {
//         &*(guard as *const Self::Guard<'o, T>)
//     }
// }

//

pub struct MappedGuard<'o, Guarding: MappableGuarding, Origin: 'o, Mapped> {
    _guard: Guarding::Guard<'o, Origin>,
    mapped: Mapped,
}

impl<'o, Guarding: MappableGuarding, Origin: 'o> MappedGuard<'o, Guarding, Origin, &'o Origin> {
    pub fn map_from_flat_ref(guard: Guarding::Guard<'o, Origin>) -> Self {
        // safe because only one mutable reference is held
        let guard_ref: &'o Origin = unsafe { Guarding::get_ref(&guard) };

        Self {
            _guard: guard,
            mapped: guard_ref,
        }
    }
}

impl<'o, Guarding: MutMappableGuarding, Origin: 'o> MappedGuard<'o, Guarding, Origin, &'o mut Origin> {
    pub fn map_from_flat_mut(mut guard: Guarding::Guard<'o, Origin>) -> Self {
        // safe because only one mutable reference is held
        let guard_mut = unsafe { Guarding::get_mut(&mut guard) };

        Self {
            _guard: guard,
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