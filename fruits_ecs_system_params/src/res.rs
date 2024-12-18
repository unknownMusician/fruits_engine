use std::{
    any::TypeId, ops::Deref, sync::RwLockReadGuard
};

use fruits_ecs_data::WorldData;
use fruits_ecs_data_usage::*;

use fruits_ecs_resource::Resource;
use fruits_ecs_system::{SystemInput, SystemParam};
use fruits_utils::mapped_guard::{MappedGuard, RwLockReadGuarding};

pub struct Res<'w, R: Resource> {
    resource: MappedGuard<'w, RwLockReadGuarding, WorldData, RwLockReadGuard<'w, R>>,
}

impl<'w, R: Resource> Deref for Res<'w, R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

unsafe impl<'a, R: Resource> SystemParam for Res<'a, R> {
    type Item<'d> = Res<'d, R>;

    fn fill_data_usage(usage: &mut DataUsage) {
        usage.add(DataUsageEntry::new_readonly(TypeId::of::<R>()));
    }

    fn new<'d>(input: SystemInput<'d>) -> Option<Self::Item<'d>> {
        let guard = input.world_data.try_read().ok()?;

        // todo: remove MappedRwLockGuard
        Some(Res {
            resource: MappedGuard::<'_, _, WorldData, _>::try_map_from(guard, |w| {
                w.resources().get::<R>()
            })?,
        })
    }
}
