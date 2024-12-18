use std::any::TypeId;

pub struct DataUsageEntry {
    pub data_type: TypeId,
    pub is_mutable: bool,
}

impl DataUsageEntry {
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