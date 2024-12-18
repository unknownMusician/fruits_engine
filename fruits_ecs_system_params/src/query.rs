use std::any::TypeId;

use fruits_ecs_component::{ArchetypeIteratorItem, Component, WorldEntitiesComponentsQuery};
use fruits_ecs_data::WorldData;
use fruits_ecs_data_usage::*;

use fruits_ecs_system::{SystemInput, SystemParam};
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

    fn fill_data_usage(usage: &mut DataUsage) {
        if let DataUsage::PerType(per_type) = usage {
            A::fill_usage(per_type);
        }
    }

    fn new<'d>(input: SystemInput<'d>) -> Option<Self::Item<'d>> {
        let guard = input.world_data.try_read().ok()?;

        let mapped_entities_components = MappedGuard::<'d, RwLockReadGuarding, WorldData, _>::map_from(guard, |w| w.entities_components());

        Some(Self::Item::<'d> {
            query: mapped_entities_components.map_into(|e| e.query::<A::Item<'d>>()),
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
