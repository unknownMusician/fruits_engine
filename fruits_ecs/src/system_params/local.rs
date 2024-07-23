use std::{any::TypeId, ops::{Deref, DerefMut}, sync::{MutexGuard, RwLock}};

use crate::{
    data::world_data::WorldData, data_usage::{SingleWorldSharedDataUsage, WorldSharedDataUsage}, system::SystemParam, system_data::{SystemState, SystemStatesHolder}
};

pub struct Local<'d, S: SystemState> {
    data: MutexGuard<'d, S>,
}

impl<'d, S: SystemState> Deref for Local<'d, S> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &*self.data
    }
}

impl<'d, S: SystemState> DerefMut for Local<'d, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.data
    }
}

unsafe impl<'a, S: SystemState> SystemParam for Local<'a, S> {
    type Item<'d> = Local<'d, S>;

    fn fill_data_usage(usage: &mut WorldSharedDataUsage) {
        usage.add(SingleWorldSharedDataUsage::new_mutable(TypeId::of::<S>()));
    }

    fn new<'d>(world_data: &'d RwLock<WorldData>, system_data: &'d SystemStatesHolder) -> Option<Self::Item<'d>> {
        Some(Local {
            data: system_data.get_or_create::<S>(),
        })
    }
}