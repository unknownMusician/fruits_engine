use std::{
    any::TypeId,
    collections::BTreeMap,
    hash::Hash
};

use super::{component::Component, type_info::TypeInfo};

#[derive(Clone)]
pub struct UniqueComponentsSet {
    component_infos: BTreeMap<TypeId, TypeInfo>,
}

impl UniqueComponentsSet {
    pub fn new() -> Self {
        Self {
            component_infos: BTreeMap::new(),
        }
    }

    pub fn component_infos(&self) -> &BTreeMap<TypeId, TypeInfo> {
        &self.component_infos
    }

    pub fn insert<C: Component>(&mut self) -> bool {
        let info = TypeInfo::new::<C>();
        self.component_infos.insert(*info.id(), info).is_none()
    }

    pub fn remove<C: Component>(&mut self) -> bool {
        self.component_infos.remove(&TypeId::of::<C>()).is_some()
    }
}

impl Hash for UniqueComponentsSet {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for component_id in self.component_infos.keys() {
            component_id.hash(state);
        }
    }
}

impl Eq for UniqueComponentsSet { }

impl PartialEq for UniqueComponentsSet {
    fn eq(&self, other: &Self) -> bool {
        if self.component_infos.len() != other.component_infos.len() {
            return false;
        }

        for component_id in self.component_infos.keys() {
            if !other.component_infos.contains_key(&component_id) {
                return false;
            }
        }

        true
    }
}
