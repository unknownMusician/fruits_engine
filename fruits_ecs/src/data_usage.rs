use std::{any::TypeId, collections::HashMap};


pub struct SingleWorldSharedDataUsage {
    pub data_type: TypeId,
    pub is_mutable: bool,
}

impl SingleWorldSharedDataUsage {
    pub fn new(data_type: TypeId, is_mutable: bool) -> Self {
        Self {
            data_type,
            is_mutable,
        }
    }
    pub fn new_mutable(type_id: TypeId) -> Self {
        Self::new(type_id, true)
    }
    pub fn new_readonly(type_id: TypeId) -> Self {
        Self::new(type_id, false)
    }
}

pub struct WorldSharedPerTypeDataUsage {
    is_mutable: HashMap<TypeId, bool>
}

impl WorldSharedPerTypeDataUsage {
    pub fn new() -> Self {
        Self {
            is_mutable: HashMap::new(),
        }
    }

    pub fn add(&mut self, usage: SingleWorldSharedDataUsage) {
        *self.is_mutable.entry(usage.data_type).or_default() |= usage.is_mutable;
    }

    pub fn values(&self) -> &HashMap<TypeId, bool> {
        &self.is_mutable
    }
}

pub enum WorldSharedDataUsage {
    PerType(WorldSharedPerTypeDataUsage),
    // todo: global immutable?
    GlobalMutable,
}

impl WorldSharedDataUsage {
    pub fn new() -> Self {
        WorldSharedDataUsage::PerType(WorldSharedPerTypeDataUsage::new())
    }

    pub fn add(&mut self, usage: SingleWorldSharedDataUsage) {
        let WorldSharedDataUsage::PerType(per_type) = self else {
            return;
        };

        per_type.add(usage);
    }

    pub fn add_all_mut(&mut self) {
        *self = WorldSharedDataUsage::GlobalMutable;
    }
}
