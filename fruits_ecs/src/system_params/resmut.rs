use std::{any::TypeId, ops::{Deref, DerefMut}, sync::{RwLock, RwLockWriteGuard}};

use crate::{
    data::{
        resource::Resource,
        world_data::WorldData
    }, data_usage::{SingleWorldSharedDataUsage, WorldSharedDataUsage}, system::SystemParam, system_data::SystemStatesHolder
};

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

    fn fill_data_usage(usage: &mut WorldSharedDataUsage) {
        usage.add(SingleWorldSharedDataUsage::new_mutable(TypeId::of::<R>()));
    }

    fn new<'d>(data: &'d RwLock<WorldData>, system_data: &'d SystemStatesHolder) -> Option<Self::Item<'d>> {
        let guard = data.try_read().ok()?;

        Some(ResMut {
            resource: MappedGuard::<'_, _, WorldData, _>::try_map_from(guard, |w| {
                w.resources().get_mut::<R>()
            })?,
        })
    }
}