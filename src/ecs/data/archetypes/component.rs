use std::{any::TypeId, collections::{HashMap, HashSet}};

use super::{
    archetype::Archetype,
    unique_components_set::UniqueComponentsSet
};

pub trait Component: 'static {
    
}

pub struct WorldArchetypes {
    archetype_id_by_components: HashMap<UniqueComponentsSet, usize>,
    archetype_ids_by_component: HashMap<TypeId, HashSet<usize>>,
    archetypes: Vec<Archetype>,
}

impl WorldArchetypes {
    pub fn new() -> Self {
        Self {
            archetype_id_by_components: HashMap::new(),
            archetype_ids_by_component: HashMap::new(),
            archetypes: Vec::new(),
        }
    }

    pub fn by_id_ref(&self, id: usize) -> Option<&Archetype> {
        self.archetypes.get(id)
    }
    pub fn by_id_mut(&mut self, id: usize) -> Option<&mut Archetype> {
        self.archetypes.get_mut(id)
    }
    pub fn by_2_ids_ref(&self, mut id: (usize, usize)) -> Option<(&Archetype, &Archetype)> {
        if id.0 >= self.archetypes.len() || id.1 >= self.archetypes.len() {
            return None;
        }

        if id.0 > id.1 {
            id = (id.1, id.0);
        }

        let slices = self.archetypes.as_slice()[id.0..].split_at(id.1 - id.0);

        Some((&slices.0[0], &slices.1[0]))
    }
    pub fn by_2_ids_mut(&mut self, mut id: (usize, usize)) -> Option<(&mut Archetype, &mut Archetype)> {
        if id.0 >= self.archetypes.len() || id.1 >= self.archetypes.len() {
            return None;
        }

        if id.0 > id.1 {
            id = (id.1, id.0);
        }

        let slices = self.archetypes.as_mut_slice()[id.0..].split_at_mut(id.1 - id.0);

        Some((&mut slices.0[0], &mut slices.1[0]))
    }
    pub fn by_components_ref(&self, components: &UniqueComponentsSet) -> Option<&Archetype> {
        let id = self.id_by_components(components)?;
        self.by_id_ref(id)
    }
    pub fn by_components_mut(&mut self, components: &UniqueComponentsSet) -> Option<&mut Archetype> {
        let id = self.id_by_components(components)?;
        self.by_id_mut(id)
    }
    pub fn id_by_components(&self, components: &UniqueComponentsSet) -> Option<usize> {
        self.archetype_id_by_components.get(&components).copied()
    }
    pub fn ids_by_component(&self, component: &TypeId) -> Option<&HashSet<usize>> {
        self.archetype_ids_by_component.get(component)
    }
    pub fn create(&mut self, components: UniqueComponentsSet) -> Result<usize, UniqueComponentsSet> {
        if self.archetype_id_by_components.contains_key(&components) {
            return Err(components);
        }

        let id = self.archetype_id_by_components.len();

        for &component_type in components.component_infos().keys() {
            self.archetype_ids_by_component.entry(component_type).or_default().insert(id);
        }

        self.archetypes.push(Archetype::new_from_components(components.clone()));
        self.archetype_id_by_components.insert(components, id);

        Ok(id)
    }
    pub fn id_by_components_or_create(&mut self, components: UniqueComponentsSet) -> (usize, Option<UniqueComponentsSet>) {
        let Some(id) = self.id_by_components(&components) else {
            return (self.create(components).ok().unwrap(), None);
        };

        (id, Some(components))
    }
}