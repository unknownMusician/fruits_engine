use std::{any::TypeId, ops::{Deref, DerefMut}, sync::{RwLock, RwLockWriteGuard}};

use crate::{ecs::{behavior::{system::SystemParam, world_behavior::WorldSharedDataUsage, SingleWorldSharedDataUsage}, data::{resources::Resource, world_data::WorldData}}, mapped_locks::MappedRwLockReadGuard};

pub struct ResMut<'w, R: 'w + Resource> {
    resource: MappedRwLockReadGuard<'w, WorldData, RwLockWriteGuard<'w, R>>,
}

impl<'w, R: Resource> Deref for ResMut<'w, R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

impl<'w, R: Resource> DerefMut for ResMut<'w, R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.resource
    }
}

unsafe impl<'a, R: Resource> SystemParam for ResMut<'a, R> {
    type Item<'b> = ResMut<'b, R>;

    fn fill_data_usage(usage: &mut WorldSharedDataUsage) {
        usage.add(SingleWorldSharedDataUsage::new_mutable(TypeId::of::<R>()));
    }

    fn from_world_data<'w>(data: &'w RwLock<WorldData>) -> Option<Self::Item<'w>> {
        let guard = data.try_read().ok()?;

        Some(ResMut {
            resource: MappedRwLockReadGuard::try_map_from(guard, |w| {
                w.resources().get_mut::<R>()
            })?,
        })
    }
}