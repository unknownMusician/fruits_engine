use std::marker::PhantomData;

use crate::{
    data::world_data::WorldData, data_usage::WorldSharedPerTypeDataUsage, system_params::WorldQuery
};

use fruits_utils::mapped_guard::{MappedGuard, RwLockReadGuarding};

use super::{
    archetype::{Archetype, ArchetypeIteratorItem}, component::{Component, WorldArchetypes}, data_rw_locker::{DataRwLocker, DataRwLockerGuard},
    entity::{Entity, EntityLocation, WorldEntities}, unique_components_set::UniqueComponentsSet,
};

pub struct WorldEntitiesComponents {
    archetypes: WorldArchetypes,
    entity_datas: WorldEntities,
    locks: DataRwLocker,
}

pub struct WorldEntitiesComponentsQuery<'w, A: ArchetypeIteratorItem> {
    archetype_indices: Box<[usize]>,
    archetypes: &'w WorldArchetypes,
    _guards: Box<[DataRwLockerGuard<'w>]>,
    _phantom: PhantomData<fn(A::Item<'static>) -> A::Item<'static>>,
}

impl<'w, A: ArchetypeIteratorItem> WorldEntitiesComponentsQuery<'w, A> {
    fn new_unchecked(archetype_indices: Box<[usize]>, guards: Box<[DataRwLockerGuard<'w>]>, archetypes: &'w WorldArchetypes) -> Self {
        Self {
            archetype_indices,
            archetypes,
            _guards: guards,
            _phantom: Default::default(),
        }
    }
    
    pub fn iter<'r>(&'r self) -> impl Iterator<Item = <A::Item<'static> as ArchetypeIteratorItem>::Item<'w>> + 'r
        where 'w: 'r
    {
        self.archetype_indices.iter()
            .copied()
            .map(|i| self.archetypes.by_id_ref(i).unwrap())
            .flat_map(move |a| a.iter::<A::Item<'static>>())
    }

    pub fn len(&self) -> usize {
        self.archetypes_iter()
            .map(|a| a.entities_count())
            .sum()
    }

    pub fn is_empty(&self) -> bool {
        !self.archetypes_iter().any(|a| a.entities_count() > 0)
    }

    fn archetypes_iter<'r>(&'r self) -> impl Iterator<Item = &'w Archetype> + 'r
        where 'w: 'r
    {
        self.archetype_indices.iter()
            .map(|i| self.archetypes.by_id_ref(*i).unwrap())
    }
}

impl WorldEntitiesComponents {
    pub fn new() -> Self {
        Self {
            archetypes: WorldArchetypes::new(),
            entity_datas: WorldEntities::new(),
            locks: DataRwLocker::new(),
        }
    }

    pub fn query<'w, A: ArchetypeIteratorItem>(&'w self) -> WorldEntitiesComponentsQuery<'w, A> {
        let mut usage = WorldSharedPerTypeDataUsage::new();

        A::fill_usage(&mut usage);

        let components = usage.values();

        let guards = self.locks.lock_by_type_usage(&usage).unwrap();

        let archetypes_with_rarest_component = components
            .keys()
            .map(|c| self.archetypes.ids_by_component(c))
            .flatten()
            .min_by_key(|a| a.len());

        let Some(archetypes_with_rarest_component) = archetypes_with_rarest_component else {
            return WorldEntitiesComponentsQuery::new_unchecked(
                Box::new([]),
                Box::new([]),
                &self.archetypes,
            );
        };

        let mut suitable_archetypes = Vec::new();

        for archetype in archetypes_with_rarest_component.iter() {
            let contains_all_components = components.keys().all(|c| {
                let Some(archetypes_with_component) = self.archetypes.ids_by_component(c) else {
                    return false;
                };

                archetypes_with_component.contains(archetype)
            });

            if contains_all_components {
                suitable_archetypes.push(*archetype);
            }
        }

        WorldEntitiesComponentsQuery::new_unchecked(
            suitable_archetypes.into_boxed_slice(),
            guards,
            &self.archetypes,
        )
    }

    pub fn entities_count(&self) -> usize {
        self.entity_datas.len()
    }

    pub fn create_entity(&mut self) -> Entity {
        let archetype_id = self.archetypes.id_by_components_or_create(UniqueComponentsSet::new()).0;

        let archetype = self.archetypes.by_id_mut(archetype_id).unwrap();

        let entity_archetype_index = archetype.create_entity();

        self.entity_datas.insert(EntityLocation {
            archetype_id,
            entity_archetype_index,
        })
    }

    pub fn destroy_entity(&mut self, entity: Entity) -> bool {
        let Some(entity_location) = self.entity_datas.remove(entity) else {
            return false;
        };

        let archetype = self.archetypes.by_id_mut(entity_location.archetype_id).unwrap();

        let last_entity = archetype.destroy_entity(entity_location.entity_archetype_index).unwrap();

        if last_entity != entity {
            *self.entity_datas.get_mut(last_entity).unwrap() = entity_location;
        }

        return true;
    }

    pub fn add_component<C: Component>(&mut self, entity: Entity, component: C) -> Option<C> {
        let entity_location = self.entity_datas.get(entity)?;

        let src_archetype_id = entity_location.archetype_id;
        
        let mut dst_components_set = {
            let src_archetype = self.archetypes.by_id_ref(src_archetype_id).unwrap();

            src_archetype.components_set().clone()
        };

        if !dst_components_set.insert::<C>() {
            return Some(component);
        }

        let dst_archetype_id = self.archetypes.id_by_components_or_create(dst_components_set).0;

        // len 0 1
        // len 1 1
        // todo: what?
        let (src_archetype, dst_archetype) = self.archetypes.by_2_ids_mut((src_archetype_id, dst_archetype_id)).unwrap();

        let entity_with_added_component_new_location = EntityLocation {
            archetype_id: dst_archetype_id,
            entity_archetype_index: dst_archetype.entities_count(),
        };

        let last_entity = Archetype::add_component(src_archetype, dst_archetype, entity_location.entity_archetype_index, component).ok().unwrap();

        if last_entity != entity {
            *self.entity_datas.get_mut(last_entity).unwrap() = *entity_location;
        }

        *self.entity_datas.get_mut(entity).unwrap() = entity_with_added_component_new_location;

        return None;
    }

    pub fn remove_component<C: Component>(&mut self, entity: Entity) -> Option<C> {
        let entity_location = self.entity_datas.get(entity)?;

        let src_archetype_id = entity_location.archetype_id;

        let mut dst_components_set = {
            let src_archetype = self.archetypes.by_id_ref(src_archetype_id).unwrap();

            src_archetype.components_set().clone()
        };

        if !dst_components_set.remove::<C>() {
            return None;
        }

        let dst_archetype_id = self.archetypes.id_by_components_or_create(dst_components_set).0;

        let (src_archetype, dst_archetype) = self.archetypes.by_2_ids_mut((src_archetype_id, dst_archetype_id)).unwrap();

        let entity_with_removed_component_new_location = EntityLocation {
            archetype_id: dst_archetype_id,
            entity_archetype_index: dst_archetype.entities_count(),
        };

        let (last_entity, component) = Archetype::remove_component(src_archetype, dst_archetype, entity_location.entity_archetype_index).unwrap();

        if last_entity != entity {
            *self.entity_datas.get_mut(last_entity).unwrap() = *entity_location;
        }

        *self.entity_datas.get_mut(entity).unwrap() = entity_with_removed_component_new_location;

        return Some(component);
    }
}
