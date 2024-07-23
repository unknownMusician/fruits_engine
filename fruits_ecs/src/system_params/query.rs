use std::{any::TypeId, sync::RwLock};

use crate::{
    data::{
        archetypes::{
            archetype::ArchetypeIteratorItem,
            component::Component,
            world_entities_components::{WorldEntitiesComponents, WorldEntitiesComponentsQuery},
        },
        world_data::WorldData,
    }, data_usage::WorldSharedDataUsage, system::SystemParam, system_data::SystemStatesHolder
};

use fruits_utils::mapped_guard::{MappedGuard, RwLockReadGuarding};

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
    query: MappedGuard::<'w, RwLockReadGuarding, WorldData, WorldEntitiesComponentsQuery<'w, A>>,
}

// todo: unsafe to sealed trait
unsafe impl<'w, A: ArchetypeIteratorItem> SystemParam for WorldQuery<'w, A> {
    type Item<'a> = WorldQuery<'a, A::Item<'a>>;

    fn fill_data_usage(usage: &mut WorldSharedDataUsage) {
        if let WorldSharedDataUsage::PerType(per_type) = usage {
            A::fill_usage(per_type);
        }
    }

    fn new<'a>(data: &'a RwLock<WorldData>, system_data: &'a SystemStatesHolder) -> Option<Self::Item<'a>> {
        let guard = data.try_read().ok()?;

        let mapped_entities_components = MappedGuard::<'a, RwLockReadGuarding, WorldData, _>::map_from(guard, |w| w.entities_components());

        Some(Self::Item::<'a> {
            query: mapped_entities_components.map_into(|e| e.query::<A::Item<'a>>()),
        })
    }
}

impl<'w, A: ArchetypeIteratorItem> WorldQuery<'w, A> {
    pub fn iter<'r>(&'r self) -> impl Iterator<Item = <A::Item<'static> as ArchetypeIteratorItem>::Item<'w>> + 'r
        where 'w: 'r
    {
        self.query.iter()
    }

    pub fn len(&self) -> usize {
        self.query.len()
    }

    pub fn is_empty(&self) -> bool {
        self.query.is_empty()
    }
}
