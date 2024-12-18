use std::{any::TypeId, ops::{Deref, DerefMut}, sync::RwLockWriteGuard};

use fruits_ecs_data::WorldData;
use fruits_ecs_data_usage::*;

use fruits_ecs_resource::Resource;
use fruits_ecs_system::{SystemInput, SystemParam};
use fruits_utils::mapped_guard::{MappedGuard, RwLockReadGuarding};

pub struct ResMut<'d, R: Resource> {
    resource: MappedGuard<'d, RwLockReadGuarding, WorldData, RwLockWriteGuard<'d, R>>,
}

impl<'d, R: Resource> Deref for ResMut<'d, R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

impl<'d, R: Resource> DerefMut for ResMut<'d, R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.resource
    }
}

unsafe impl<'a, R: Resource> SystemParam for ResMut<'a, R> {
    type Item<'d> = ResMut<'d, R>;

    fn fill_data_usage(usage: &mut DataUsage) {
        usage.add(DataUsageEntry::new_mutable(TypeId::of::<R>()));
    }

    fn new<'d>(input: SystemInput<'d>) -> Option<Self::Item<'d>> {
        let guard = input.world_data.try_read().ok()?;

        Some(ResMut {
            resource: MappedGuard::<'_, _, WorldData, _>::try_map_from(guard, |w| {
                w.resources().get_mut::<R>()
            })?,
        })
    }

}