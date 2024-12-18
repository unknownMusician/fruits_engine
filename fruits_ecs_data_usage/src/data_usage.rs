use crate::{data_usage_entry::DataUsageEntry, PerTypeDataUsage};

pub enum DataUsage {
    PerType(PerTypeDataUsage),
    // todo: global immutable?
    GlobalMutable,
}

impl DataUsage {
    pub fn new() -> Self {
        DataUsage::PerType(PerTypeDataUsage::new())
    }

    pub fn add(&mut self, usage: DataUsageEntry) {
        let DataUsage::PerType(per_type) = self else {
            return;
        };

        per_type.add(usage);
    }

    pub fn add_all_mut(&mut self) {
        *self = DataUsage::GlobalMutable;
    }
}