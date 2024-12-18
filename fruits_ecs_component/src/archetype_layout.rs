use std::{
    any::TypeId,
    collections::HashMap
};

use super::{
    entity::Entity, type_info::TypeInfo, unique_components_set::UniqueComponentsSet, unsafe_archetype::{
        ArchetypeItemPhysicalLocation,
        CHUNK_SIZE,
    }
};

// todo (critical): account for alignment (now it only works safe on Intel CPU)

pub struct ArchetypeItemLayout {
    pub type_info: TypeInfo,
    pub offset: usize,
    pub order: usize,
}

pub struct ArchetypeLayout {
    components_set: UniqueComponentsSet,
    components: HashMap<TypeId, ArchetypeItemLayout>,
    entity_size: usize,
}

impl ArchetypeLayout {
    pub fn new_from_components(components_set: UniqueComponentsSet) -> Self {
        let mut components = HashMap::new();

        let mut offset = std::mem::size_of::<Entity>();

        for (order, (&id, &type_info)) in components_set.component_infos().iter().enumerate() {
            components.insert(id, ArchetypeItemLayout {
                offset,
                type_info,
                order: order + 1,
            });

            offset += type_info.size();
        }

        Self {
            components_set,
            components,
            entity_size: offset,
        }
    }

    pub fn components_set(&self) -> &UniqueComponentsSet {
        &self.components_set
    }

    pub fn entity_item_layout() -> ArchetypeItemLayout {
        ArchetypeItemLayout {
            offset: 0,
            order: 0,
            type_info: TypeInfo::new::<Entity>(),
        }
    }

    pub fn components(&self) -> &HashMap<TypeId, ArchetypeItemLayout> {
        &self.components
    }

    pub fn entity_size(&self) -> usize {
        self.entity_size
    }

    pub fn entities_per_chunk_count(&self) -> usize {
        CHUNK_SIZE / self.entity_size
    }
    
    pub fn is_component_the_only_difference(with_component: &Self, without_component: &Self, component: &TypeId) -> bool {
        let with_component = with_component.components();
        let without_component = without_component.components();
        
        if with_component.len() != without_component.len() + 1 {
            return false;
        }

        if without_component.contains_key(component) {
            return false;
        }

        if !with_component.contains_key(component) {
            return false;
        }

        for component_id in without_component.keys() {
            if !with_component.contains_key(component_id) {
                return false;
            }
        }

        return true;
    }
}

impl ArchetypeLayout {
    pub fn chunk_index(&self, entity_in_archetype_index: usize) -> usize {
        entity_in_archetype_index / self.entities_per_chunk_count()
    }

    pub fn entity_in_chunk_index(&self, entity_in_archetype_index: usize) -> usize {
        entity_in_archetype_index % self.entities_per_chunk_count()
    }

    pub fn component_memory_physical_location(&self, entity_in_archetype_index: usize, component: &TypeId) -> ArchetypeItemPhysicalLocation {
        let item_layout = self.components.get(component).unwrap();

        self.memory_physical_location(entity_in_archetype_index, item_layout)
    }

    pub fn entity_memory_physical_location(&self, entity_in_archetype_index: usize) -> ArchetypeItemPhysicalLocation {
        let item_layout = ArchetypeLayout::entity_item_layout();

        self.memory_physical_location(entity_in_archetype_index, &item_layout)
    }

    fn memory_physical_location(&self, entity_in_archetype_index: usize, item_layout: &ArchetypeItemLayout) -> ArchetypeItemPhysicalLocation {
        let entity_in_chunk_index = self.entity_in_chunk_index(entity_in_archetype_index);

        let components_line_offset = self.entities_per_chunk_count() * item_layout.offset;

        let memory_size = item_layout.type_info.size();

        let memory_offset = components_line_offset + entity_in_chunk_index * memory_size;

        ArchetypeItemPhysicalLocation {
            chunk_index: self.chunk_index(entity_in_archetype_index),
            memory_offset,
            memory_size,
        }
    }
}