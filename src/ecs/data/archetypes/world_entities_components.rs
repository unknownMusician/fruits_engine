use crate::{
    ecs::{behavior::WorldSharedPerTypeDataUsage, data::world_data::WorldData},
    mapped_locks::MappedRwLockReadGuard,
    WorldQuery,
};

use super::{
    archetype::{Archetype, ArchetypeIteratorItem}, component::{Component, WorldArchetypes}, data_rw_locker::DataRwLocker,
    entity::{Entity, EntityLocation, WorldEntities}, unique_components_set::UniqueComponentsSet,
};

pub struct WorldEntitiesComponents {
    archetypes: WorldArchetypes,
    entity_datas: WorldEntities,
    locks: DataRwLocker,
}

impl WorldEntitiesComponents {
    pub fn new() -> Self {
        Self {
            archetypes: WorldArchetypes::new(),
            entity_datas: WorldEntities::new(),
            locks: DataRwLocker::new(),
        }
    }

    pub fn create_query<'w, P: ArchetypeIteratorItem>(
        this: MappedRwLockReadGuard<'w, WorldData, &'w Self>,
    ) -> WorldQuery<'w, P> {
        let self_dereferenced = *this;

        let mut usage = WorldSharedPerTypeDataUsage::new();

        P::fill_usage(&mut usage);

        let components = usage.values();

        let guards = self_dereferenced.locks.lock_by_type_usage(&usage).unwrap();

        let archetypes_with_rarest_component = components
            .keys()
            .map(|c| self_dereferenced.archetypes.ids_by_component(c))
            .flatten()
            .min_by_key(|a| a.len());

        let Some(archetypes_with_rarest_component) = archetypes_with_rarest_component else {
            return WorldQuery::new_unchecked(
                Box::new([]),
                Box::new([]),
                this.map_into(|s| &s.archetypes),
            );
        };

        let mut suitable_archetypes = Vec::new();

        for archetype in archetypes_with_rarest_component.iter() {
            let contains_all_components = components.keys().all(|c| {
                let Some(archetypes_with_component) =
                    self_dereferenced.archetypes.ids_by_component(c)
                else {
                    return false;
                };

                archetypes_with_component.contains(archetype)
            });

            if contains_all_components {
                suitable_archetypes.push(*archetype);
            }
        }

        WorldQuery::new_unchecked(
            suitable_archetypes.into_boxed_slice(),
            guards,
            this.map_into(|s| &s.archetypes),
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
