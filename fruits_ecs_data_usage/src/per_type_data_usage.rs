use std::{any::TypeId, collections::HashMap};

use crate::data_usage_entry::DataUsageEntry;

pub struct PerTypeDataUsage {
    is_mutable: HashMap<TypeId, bool>
}

impl PerTypeDataUsage {
    pub fn new() -> Self {
        Self {
            is_mutable: HashMap::new(),
        }
    }

    pub fn add(&mut self, usage: DataUsageEntry) {
        *self.is_mutable.entry(usage.data_type).or_default() |= usage.is_mutable;
    }

    pub fn values(&self) -> &HashMap<TypeId, bool> {
        &self.is_mutable
    }

    pub fn into_values(self) -> HashMap<TypeId, bool> {
        self.is_mutable
    }
}