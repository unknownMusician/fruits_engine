use std::{any::TypeId, ops::{Deref, DerefMut}};

use fruits_ecs_data_usage::*;
use fruits_ecs_system::{SystemInput, SystemParam};
use fruits_ecs_system_resource::{SystemState, SystemStatesHolderGuard};

pub struct Local<'d, S: SystemState> {
    data: SystemStatesHolderGuard<'d, S>,
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

    fn fill_data_usage(usage: &mut DataUsage) {
        usage.add(DataUsageEntry::new_mutable(TypeId::of::<S>()));
    }

    fn new<'d>(input: SystemInput<'d>) -> Option<Self::Item<'d>> {
        Some(Local {
            data: input.system_data.get_or_create::<S>()?,
        })
    }
}