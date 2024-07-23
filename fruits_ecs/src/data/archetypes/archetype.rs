use std::{any::TypeId, marker::PhantomData, sync::Arc};

use crate::data_usage::{SingleWorldSharedDataUsage, WorldSharedPerTypeDataUsage};

use super::{archetype_layout::ArchetypeLayout, component::Component, entity::Entity, unique_components_set::UniqueComponentsSet, unsafe_archetype::{ArchetypeItemPhysicalLocation, UnsafeArchetype}};

pub unsafe trait ArchetypeIteratorItem {
    type Item<'w>: 'w + ArchetypeIteratorItem;
    
    fn from_archetype<'w>(entity_index: usize, archetype: &'w UnsafeArchetype, layout: &ArchetypeLayout) -> Self::Item<'w>;
    fn fill_usage(usage: &mut WorldSharedPerTypeDataUsage);
}

unsafe impl<'a, C: Component> ArchetypeIteratorItem for &'a C {
    type Item<'w> = &'w C;
    
    fn from_archetype<'w>(entity_index: usize, archetype: &'w UnsafeArchetype, layout: &ArchetypeLayout) -> Self::Item<'w> {
        let item_location = layout.component_memory_physical_location(entity_index, &TypeId::of::<C>());

        unsafe {
            let memory_ref = archetype.get_memory(&item_location);

            &*(memory_ref.0 as *const C)
        }
    }
    
    fn fill_usage(usage: &mut WorldSharedPerTypeDataUsage) {
        usage.add(SingleWorldSharedDataUsage::new_readonly(TypeId::of::<C>()));
    }
}

unsafe impl<C: Component> ArchetypeIteratorItem for &mut C {
    type Item<'w> = &'w mut C;
    
    fn from_archetype<'w>(entity_index: usize, archetype: &'w UnsafeArchetype, layout: &ArchetypeLayout) -> Self::Item<'w> {
        let item_location = layout.component_memory_physical_location(entity_index, &TypeId::of::<C>());

        unsafe {
            let memory_ref = archetype.get_memory(&item_location);

            &mut *(memory_ref.0 as *mut C)
        }
    }
    
    fn fill_usage(usage: &mut WorldSharedPerTypeDataUsage) {
        usage.add(SingleWorldSharedDataUsage::new_mutable(TypeId::of::<C>()));
    }
}

unsafe impl ArchetypeIteratorItem for Entity {
    type Item<'w> = Entity;
    
    fn from_archetype<'w>(entity_index: usize, archetype: &'w UnsafeArchetype, layout: &ArchetypeLayout) -> Self::Item<'w> {
        let item_location = layout.entity_memory_physical_location(entity_index);

        unsafe {
            let memory_ref = archetype.get_memory(&item_location);

            *(memory_ref.0 as *const Entity)
        }
    }
    
    fn fill_usage(usage: &mut WorldSharedPerTypeDataUsage) { }
}

macro_rules! archetype_iterator_item_impl {
    ($($P: ident),+) => {
        #[allow(unused_parens)]
        unsafe impl<$($P),+> ArchetypeIteratorItem for ($($P),+)
        where
            $($P: ArchetypeIteratorItem),+
        {
            type Item<'w> = (
                $($P::Item<'w>),+
            );
            
            fn from_archetype<'w>(entity_index: usize, archetype: &'w UnsafeArchetype, layout: &ArchetypeLayout) -> Self::Item<'w> {
                (
                    $($P::from_archetype(entity_index, archetype, layout)),+
                )
            }
            
            fn fill_usage(usage: &mut WorldSharedPerTypeDataUsage) {
                $($P::fill_usage(usage));+;
            }
        }
    };
}

archetype_iterator_item_impl!(P0, P1);
archetype_iterator_item_impl!(P0, P1, P2);
archetype_iterator_item_impl!(P0, P1, P2, P3);
archetype_iterator_item_impl!(P0, P1, P2, P3, P4);
archetype_iterator_item_impl!(P0, P1, P2, P3, P4, P5);
archetype_iterator_item_impl!(P0, P1, P2, P3, P4, P5, P6);
archetype_iterator_item_impl!(P0, P1, P2, P3, P4, P5, P6, P7);
archetype_iterator_item_impl!(P0, P1, P2, P3, P4, P5, P6, P7, P8);
archetype_iterator_item_impl!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9);
archetype_iterator_item_impl!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10);
archetype_iterator_item_impl!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11);
archetype_iterator_item_impl!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12);
archetype_iterator_item_impl!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13);
archetype_iterator_item_impl!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14);


pub struct ArchetypeIterator<'a, A: ArchetypeIteratorItem> {
    archetype: &'a UnsafeArchetype,
    archetype_layout: Arc<ArchetypeLayout>,
    entities_count: usize,
    entity_index: usize,
    _phantom: PhantomData<&'a mut A>,
}

impl<'a, A: ArchetypeIteratorItem> ArchetypeIterator<'a, A> {
    pub fn new(archetype: &'a UnsafeArchetype, archetype_layout: Arc<ArchetypeLayout>, entities_count: usize) -> Self {
        Self {
            archetype,
            archetype_layout,
            entities_count,
            entity_index: 0,
            _phantom: Default::default(),
        }
    }
}

impl<'a, A: ArchetypeIteratorItem> Iterator for ArchetypeIterator<'a, A> {
    type Item = A::Item<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.entity_index >= self.entities_count {
            return None;
        }
        
        let result = A::from_archetype(self.entity_index, &self.archetype, &self.archetype_layout);

        self.entity_index += 1;

        Some(result)
    }
}

pub struct Archetype {
    layout: Arc<ArchetypeLayout>,
    archetype: UnsafeArchetype,
    alive_entities_count: usize,
}

impl Archetype {
    pub fn new_from_components(components_set: UniqueComponentsSet) -> Self {
        let components_count = components_set.component_infos().len();

        Self {
            layout: Arc::new(ArchetypeLayout::new_from_components(components_set)),
            archetype: UnsafeArchetype::new(),
            alive_entities_count: 0,
        }
    }

    pub fn contains_component_type<C: Component>(&self) -> bool {
        self.layout.components().contains_key(&TypeId::of::<C>())
    }

    pub fn components_set(&self) -> &UniqueComponentsSet {
        self.layout.components_set()
    }

    pub fn iter<'a, A: ArchetypeIteratorItem>(&'a self) -> ArchetypeIterator<'a, A> {
        // todo: threading guards
        ArchetypeIterator::new(
            &self.archetype,
            Arc::clone(&self.layout),
            self.alive_entities_count,
        )
    }

    pub fn entities_count(&self) -> usize {
        self.alive_entities_count
    }

    pub fn get_entity(&self, entity_index: usize) -> Option<Entity> {
        if entity_index >= self.alive_entities_count {
            return None;
        }

        let physical_location = self.layout.entity_memory_physical_location(entity_index);
        
        let entity = unsafe {
            let memory = self.archetype.get_memory(&physical_location);
            *(memory.0 as *const Entity)
        };

        Some(entity)
    }

    unsafe fn get_entity_ref_unchecked(&self, entity_index: usize) -> &Entity {
        let physical_location = self.layout.entity_memory_physical_location(entity_index);
        
        let memory = self.archetype.get_memory(&physical_location);
        &*(memory.0 as *const Entity)
    }

    unsafe fn get_entity_mut_unchecked(&self, entity_index: usize) -> &mut Entity {
        let physical_location = self.layout.entity_memory_physical_location(entity_index);
        
        let memory = self.archetype.get_memory(&physical_location);
        &mut *(memory.0 as *mut Entity)
    }

    pub fn get_component_ref<C: Component>(&self, entity_index: usize) -> Option<&C> {
        unsafe {
            self.get_component_ptr::<C>(entity_index).map(|p| &*(p as *const C))
        }
    }

    unsafe fn get_component_ptr<C: Component>(&self, entity_index: usize) -> Option<*mut ()> {
        if entity_index >= self.alive_entities_count {
            return None;
        }

        let component_type = TypeId::of::<C>();

        if !self.layout.components().contains_key(&component_type) {
            return None;
        }

        let physical_location = self.layout.component_memory_physical_location(entity_index, &component_type);
        
        Some(self.archetype.get_memory(&physical_location).0)
    }

    pub fn get_component_mut<C: Component>(&self, entity_index: usize) -> Option<&mut C> {
        if entity_index >= self.alive_entities_count {
            return None;
        }

        let component_type = TypeId::of::<C>();

        if !self.layout.components().contains_key(&component_type) {
            return None;
        }

        let physical_location = self.layout.component_memory_physical_location(entity_index, &component_type);
        
        let component = unsafe {
            let memory = self.archetype.get_memory(&physical_location);
            &mut *(memory.0 as *mut C)
        };

        Some(component)
    }

    pub fn create_entity(&mut self) -> usize {
        // todo: initialize components
        let entity_in_archetype_index = self.alive_entities_count;

        let chunk_index = self.layout.chunk_index(entity_in_archetype_index);

        if chunk_index >= self.archetype.chunks_count() {
            unsafe { self.archetype.push_chunk() };
        }

        self.alive_entities_count += 1;

        entity_in_archetype_index
    }

    /// Returns the last entity before the destroy.
    pub fn destroy_entity(&mut self, entity_index: usize) -> Option<Entity> {
        if entity_index >= self.alive_entities_count {
            return None;
        }

        for item_layout in self.layout.components().values() {
            let location = self.layout.component_memory_physical_location(entity_index, item_layout.type_info.id());
            unsafe {
                let memory = self.archetype.get_memory(&location);
                item_layout.type_info.drop(memory.0)
            }
        }

        self.erase_entity(entity_index)
    }

    fn erase_entity(&mut self, entity_index: usize) -> Option<Entity> {
        // todo: Release the unneded chunks?
        if entity_index >= self.alive_entities_count {
            return None;
        }

        let last_index = self.alive_entities_count - 1;

        if entity_index != last_index {
            
            let last_components_locations = Self::get_items_locations_iter(&self.layout, last_index, &self.layout);
            let entity_components_locations = Self::get_items_locations_iter(&self.layout, entity_index, &self.layout);

            for (src_location, dst_location) in last_components_locations.zip(entity_components_locations) {
                unsafe {
                    let src_mem = self.archetype.get_memory(&src_location);
                    let dst_mem = self.archetype.get_memory(&dst_location);
                    std::ptr::copy_nonoverlapping(src_mem.0 as *mut u8, dst_mem.0 as *mut u8, dst_mem.1)
                }
            }
        }

        let last_entity_location = self.layout.entity_memory_physical_location(last_index);

        let last_entity = unsafe {
            let last_enity_memory = self.archetype.get_memory(&last_entity_location).0;
            *(last_enity_memory as *const Entity)
        };

        self.alive_entities_count -= 1;

        Some(last_entity)
    }

    /// Returns the last entity from src archetype before the movement.
    pub fn add_component<C: Component>(src: &mut Self, dst: &mut Self, src_entity_index: usize, component: C) -> Result<Entity, C> {
        if !ArchetypeLayout::is_component_the_only_difference(&dst.layout, &src.layout, &TypeId::of::<C>()) {
            return Err(component);
        }

        let dst_entity_index = dst.create_entity();

        let src_components_locations = Self::get_items_locations_iter(&src.layout, src_entity_index, &src.layout);
        let dst_components_locations = Self::get_items_locations_iter(&src.layout, dst_entity_index, &dst.layout);

        for (src_location, dst_location) in src_components_locations.zip(dst_components_locations) {
            unsafe {
                let src_mem = src.archetype.get_memory(&src_location);
                let dst_mem = dst.archetype.get_memory(&dst_location);
                std::ptr::copy_nonoverlapping(src_mem.0 as *mut u8, dst_mem.0 as *mut u8, dst_mem.1);
            }
        }

        let added_component_location = dst.layout.component_memory_physical_location(dst_entity_index, &TypeId::of::<C>());

        unsafe {
            let added_mem = dst.archetype.get_memory(&added_component_location);
            *(added_mem.0 as *mut C) = component;
        }
        
        Ok(src.erase_entity(src_entity_index).unwrap())
    }

    fn get_items_locations_iter<'l>(componens_layout: &'l ArchetypeLayout, entity_in_archetype_index: usize, memory_layout: &'l ArchetypeLayout) -> impl Iterator<Item = ArchetypeItemPhysicalLocation> + 'l {
        componens_layout
            .components()
            .keys()
            .map(move |t| memory_layout.component_memory_physical_location(entity_in_archetype_index.clone(), &t.clone()))
            .chain(std::iter::once(memory_layout.entity_memory_physical_location(entity_in_archetype_index.clone())))
    }

    /// Returns the last entity from src archetype before the movement.
    pub fn remove_component<C: Component>(src: &mut Self, dst: &mut Self, src_entity_index: usize) -> Option<(Entity, C)> {
        if !ArchetypeLayout::is_component_the_only_difference(&src.layout, &dst.layout, &TypeId::of::<C>()) {
            return None;
        }

        let dst_entity_index = dst.create_entity();

        let src_components_locations = Self::get_items_locations_iter(&dst.layout, src_entity_index, &src.layout);
        let dst_components_locations = Self::get_items_locations_iter(&dst.layout, dst_entity_index, &dst.layout);

        for (src_location, dst_location) in src_components_locations.zip(dst_components_locations) {
            unsafe {
                let src_mem = src.archetype.get_memory(&src_location);
                let dst_mem = dst.archetype.get_memory(&dst_location);
                std::ptr::copy_nonoverlapping(src_mem.0 as *mut u8, dst_mem.0 as *mut u8, dst_mem.1)
            }
        }

        let component_ref = src.get_component_ref::<C>(src_entity_index).unwrap();

        let component = unsafe { std::mem::transmute_copy(component_ref) };

        Some((src.erase_entity(src_entity_index).unwrap(), component))
    }
}