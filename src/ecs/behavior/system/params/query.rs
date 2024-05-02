use std::{any::TypeId, marker::PhantomData, sync::RwLock};

use crate::{
    ecs::{
        behavior::{system::SystemParam, WorldSharedDataUsage},
        data::{
            archetypes::{
                archetype::{Archetype, ArchetypeIteratorItem},
                component::{Component, WorldArchetypes},
                data_rw_locker::DataRwLockerGuard,
                world_entities_components::WorldEntitiesComponents,
            },
            world_data::WorldData,
        }
    },
    mapped_locks::MappedRwLockReadGuard,
};

pub unsafe trait WorldQueryIterParam {
    fn component_type() -> TypeId;
    fn is_mutable() -> bool;
}

unsafe impl<P: Component> WorldQueryIterParam for &P {
    fn component_type() -> TypeId { TypeId::of::<P>() }
    fn is_mutable() -> bool { false }
}

unsafe impl<P: Component> WorldQueryIterParam for &mut P {
    fn component_type() -> TypeId { TypeId::of::<P>() }
    fn is_mutable() -> bool { true }
}

pub struct WorldQuery<'w, A: ArchetypeIteratorItem> {
    archetype_indices: Box<[usize]>,
    archetypes: MappedRwLockReadGuard<'w, WorldData, &'w WorldArchetypes>,
    _guards: Box<[DataRwLockerGuard<'w>]>,
    _phantom: PhantomData<fn(A) -> A>,
}

unsafe impl<'b, A: ArchetypeIteratorItem> SystemParam for WorldQuery<'b, A>
{
    type Item<'a> = WorldQuery<'a, <A as ArchetypeIteratorItem>::Item<'a>>;

    fn fill_data_usage(usage: &mut WorldSharedDataUsage) {
        if let WorldSharedDataUsage::PerType(per_type) = usage {
            A::fill_usage(per_type);
        }
    }

    fn from_world_data<'w>(data: &'w RwLock<WorldData>) -> Option<Self::Item<'w>> {
        let guard = data.try_read().ok()?;

        let mapped_entities_components = MappedRwLockReadGuard::map_from(guard, |w| w.entities_components());

        Some(WorldEntitiesComponents::create_query(mapped_entities_components))
    }
}

impl<'w, A: ArchetypeIteratorItem> WorldQuery<'w, A> {
    pub fn new_unchecked(archetype_indices: Box<[usize]>, guards: Box<[DataRwLockerGuard<'w>]>, archetypes: MappedRwLockReadGuard<'w, WorldData, &'w WorldArchetypes>) -> Self {
        Self {
            archetype_indices,
            _guards: guards,
            archetypes,
            _phantom: Default::default(),
        }
    }

    pub fn iter<'s>(&'s self) -> impl Iterator<Item = A::Item<'w>> + 's
        where 's : 'w
    {
        self.archetypes_iter()
            .map(|a| a.iter::<A>())
            .flatten()
    }

    pub fn len(&self) -> usize {
        self.archetypes_iter()
            .map(|a| a.entities_count())
            .sum()
    }

    pub fn is_empty(&self) -> bool {
        !self.archetypes_iter().any(|a| a.entities_count() > 0)
    }

    fn archetypes_iter<'s>(&'s self) -> impl Iterator<Item = &Archetype> + 's {
        self.archetype_indices.iter()
            .map(|i| self.archetypes.by_id_ref(*i).unwrap())
    }
}